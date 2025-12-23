use super::unix::now_unix;
use crate::linux::isolation::nix_to_io;
use crate::linux::setup::ContainerSetup;
use crate::runtime::state::Status;
use crate::runtime::{Result, RuntimeError, spec::Spec, state::State, store::Store};
use nix::sched::CloneFlags;
use nix::sys::signal::{self};
use nix::unistd::Pid;
use std::fs;
use std::fs::File;
use std::io;
use std::os::fd::AsFd;
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;
use std::{thread, time::Duration};

pub struct Container {
    id: String,
    store: Store,
    spec: Spec,
    state: State,
}

impl Container {
    pub fn create(id: String, rootfs: PathBuf, argv: Vec<String>) -> Result<Self> {
        let store = Store::new()?;

        if store.exists(&id) {
            return Err(RuntimeError::Msg(format!("container {id} already exists")));
        }

        if argv.is_empty() {
            return Err(RuntimeError::Msg("no command provided".into()));
        }

        if !rootfs.exists() {
            return Err(RuntimeError::Msg(format!(
                "rootfs does not exist: {}",
                rootfs.display()
            )));
        }

        if !rootfs.is_dir() {
            return Err(RuntimeError::Msg(format!(
                "rootfs is not a directory: {}",
                rootfs.display()
            )));
        }

        let spec = Spec {
            rootfs,
            argv,
            env: vec![],
            cwd: None,
            hostname: Some(id.clone()),
        };

        let state = State::new_created(id.clone());

        store.create_container(&id, &spec, &state)?;

        Ok(Self {
            id,
            store,
            spec,
            state,
        })
    }

    pub fn open(id: String) -> Result<Self> {
        let store = Store::new()?;

        if !store.exists(&id) {
            return Err(RuntimeError::Msg(format!("container {id} not found")));
        }

        let spec = store.load_spec(&id)?;
        let state = store.load_state(&id)?;

        Ok(Self {
            id,
            store,
            spec,
            state,
        })
    }

    pub fn start(&mut self) -> Result<i32> {
        if matches!(self.state.status, Status::Running) {
            return Err(RuntimeError::Msg("already running".into()));
        }
        if self.spec.argv.is_empty() {
            return Err(RuntimeError::Msg("spec argv is empty".into()));
        }

        // Container program path
        let prog = self.spec.argv[0].clone();

        // Validate
        let rel = prog.trim_start_matches('/');
        let in_rootfs = self.spec.rootfs.join(rel);
        if !in_rootfs.exists() {
            return Err(RuntimeError::Msg(format!(
                "executable not found in rootfs: {} (from {})",
                in_rootfs.display(),
                prog
            )));
        }

        let args: Vec<String> = self.spec.argv.iter().skip(1).cloned().collect();

        let rootfs = self.spec.rootfs.clone();
        let hostname = self.spec.hostname.clone();

        let mut cmd = Command::new(&prog);
        cmd.args(&args);

        if let Some(cwd) = &self.spec.cwd {
            cmd.current_dir(cwd);
        }

        for (k, v) in &self.spec.env {
            cmd.env(k, v);
        }

        unsafe {
            cmd.pre_exec(move || {
                ContainerSetup::new(&rootfs)
                    .hostname(hostname.as_deref())
                    .mount_proc(true)
                    .mount_dev(true)
                    .apply()
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
                Ok(())
            });
        }

        let child = cmd.spawn()?;
        let pid = child.id() as i32;

        self.state.status = Status::Running;
        self.state.pid = Some(pid);
        self.state.started_at_unix = Some(now_unix());

        self.store.save_state(&self.id, &self.state)?;
        Ok(pid)
    }

    pub fn kill(&mut self, signal_num: i32) -> Result<()> {
        let pid_i32 = self
            .state
            .pid
            .ok_or_else(|| RuntimeError::Msg("no pid recorded for container".into()))?;

        let pid = Pid::from_raw(pid_i32);

        let sig = Self::signal_from_i32(signal_num)?;

        // If it's already dead, treat as success and update state
        if !Self::proc_exists(pid_i32) {
            self.state.status = Status::Stopped;
            self.state.pid = None;
            self.store.save_state(&self.id, &self.state)?;
            return Ok(());
        }

        let res = match sig {
            None => signal::kill(pid, None),
            Some(s) => signal::kill(pid, Some(s)),
        };

        if let Err(e) = res {
            // ESRCH - "no such process"
            if e == nix::errno::Errno::ESRCH {
                self.state.status = Status::Stopped;
                self.state.pid = None;
                self.store.save_state(&self.id, &self.state)?;
                return Ok(());
            }
            return Err(RuntimeError::Io(nix_to_io(e)));
        }

        if sig.is_some() {
            for _ in 0..50 {
                if !Self::proc_exists(pid_i32) {
                    self.state.status = Status::Stopped;
                    self.state.pid = None;
                    self.store.save_state(&self.id, &self.state)?;
                    return Ok(());
                }
                thread::sleep(Duration::from_millis(10));
            }
        }

        self.state.status = Status::Running;
        self.store.save_state(&self.id, &self.state)?;

        Ok(())
    }

    pub fn delete(self) -> Result<()> {
        if matches!(self.state.status, Status::Running) {
            if let Some(pid) = self.state.pid {
                if Self::proc_exists(pid) {
                    return Err(RuntimeError::Msg(
                        "refusing to delete: container is Running (kill it first)".into(),
                    ));
                }
            }
        }

        let dir = self.store.dir(&self.id);
        if dir.exists() {
            fs::remove_dir_all(&dir)?;
        }

        Ok(())
    }

    pub fn wait(&mut self) -> Result<i32> {
        let pid = self
            .state
            .pid
            .ok_or_else(|| RuntimeError::Msg("no pid recorded for container".into()))?;

        if !Self::proc_exists(pid) {
            self.state.status = Status::Stopped;
            self.state.pid = None;
            self.store.save_state(&self.id, &self.state)?;
            return Ok(0);
        }

        while Self::proc_exists(pid) {
            thread::sleep(Duration::from_millis(50));
        }

        self.state.status = Status::Stopped;
        self.state.pid = None;
        self.store.save_state(&self.id, &self.state)?;

        Ok(0)
    }

    pub fn exec(
        &self,
        argv: Vec<String>,
        env: Vec<(String, String)>,
        cwd: Option<std::path::PathBuf>,
    ) -> Result<i32> {
        if argv.is_empty() {
            return Err(RuntimeError::Msg("exec argv is empty".into()));
        }

        let target_pid = self
            .state
            .pid
            .ok_or_else(|| RuntimeError::Msg("container has no pid (not running?)".into()))?;

        if !Self::proc_exists(target_pid) {
            return Err(RuntimeError::Msg("container pid is not alive".into()));
        }

        let ns_mnt = PathBuf::from(format!("/proc/{}/ns/mnt", target_pid));
        let ns_uts = PathBuf::from(format!("/proc/{}/ns/uts", target_pid));
        let proc_root = PathBuf::from(format!("/proc/{}/root", target_pid));
        let cwd = cwd.unwrap_or_else(|| PathBuf::from("/"));

        let prog = argv[0].clone();
        let args: Vec<String> = argv.into_iter().skip(1).collect();

        let mut cmd = Command::new(&prog);
        cmd.args(&args);

        for (k, v) in env {
            cmd.env(k, v);
        }

        cmd.stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());

        unsafe {
            cmd.pre_exec(move || {
                Self::setns_file(&ns_mnt, CloneFlags::CLONE_NEWNS)?;
                Self::setns_file(&ns_uts, CloneFlags::CLONE_NEWUTS)?;

                nix::unistd::chdir(&proc_root).map_err(nix_to_io)?;
                nix::unistd::chroot(".").map_err(nix_to_io)?;

                nix::unistd::chdir(&cwd).map_err(nix_to_io)?;
                Ok(())
            });
        }

        let st = cmd.status()?;
        Ok(st.code().unwrap_or(0))
    }

    fn setns_file(path: &std::path::Path, nstype: nix::sched::CloneFlags) -> io::Result<()> {
        let f = File::open(path)?;
        nix::sched::setns(f.as_fd(), nstype).map_err(nix_to_io)?;
        Ok(())
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn get_spec(&self) -> &Spec {
        &self.spec
    }

    pub fn get_state(&self) -> &State {
        &self.state
    }

    fn signal_from_i32(sig: i32) -> Result<Option<nix::sys::signal::Signal>> {
        use nix::sys::signal::Signal;

        if sig == 0 {
            return Ok(None);
        }

        Signal::try_from(sig)
            .map(Some)
            .map_err(|_| RuntimeError::Msg(format!("invalid signal: {sig}")))
    }

    fn proc_exists(pid: i32) -> bool {
        std::path::Path::new("/proc").join(pid.to_string()).exists()
    }
}
