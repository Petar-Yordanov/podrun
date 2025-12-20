use super::unix::now_unix;
use crate::linux::setup::ContainerSetup;
use crate::runtime::state::Status;
use crate::runtime::{Result, RuntimeError, spec::Spec, state::State, store::Store};
use std::io;
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::Command;

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

    pub fn kill(&mut self, _signal: i32) -> Result<()> {
        unimplemented!("kill() not implemented yet");
    }

    pub fn delete(self) -> Result<()> {
        unimplemented!("delete() not implemented yet");
    }

    pub fn wait(&mut self) -> Result<i32> {
        unimplemented!("wait() not implemented yet");
    }

    pub fn exec(
        &self,
        _argv: Vec<String>,
        _env: Vec<(String, String)>,
        _cwd: Option<std::path::PathBuf>,
    ) -> Result<i32> {
        unimplemented!("exec() not implemented yet");
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
}
