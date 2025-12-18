use crate::runtime::error::Result;
use crate::runtime::{spec::Spec, state::State, store::Store};
use std::path::PathBuf;

pub struct Container {
    id: String,
    store: Store,
    spec: Spec,
    state: State,
}

impl Container {
    pub fn create(id: String, rootfs: PathBuf, argv: Vec<String>) -> Result<Self> {
        unimplemented!("create() not implemented yet");
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
