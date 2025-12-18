use crate::runtime::{Result, RuntimeError, spec::Spec, state::State, store::Store};
use std::path::PathBuf;

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
        unimplemented!("open() not implemented yet");
    }

    pub fn start(&mut self) -> Result<i32> {
        unimplemented!("start() not implemented yet");
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
