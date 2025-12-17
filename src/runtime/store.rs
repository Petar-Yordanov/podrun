use crate::runtime::{spec::Spec, state::State};
use std::{fs, io, path::PathBuf};

pub struct Store {
    root: PathBuf,
}

impl Store {
    pub fn new() -> io::Result<Self> {
        let home = std::env::var_os("HOME")
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "HOME not set"))?;
        Ok(Self {
            root: PathBuf::from(home).join(".myrt").join("containers"),
        })
    }

    pub fn dir(&self, id: &str) -> PathBuf {
        self.root.join(id)
    }

    pub fn spec_path(&self, id: &str) -> PathBuf {
        self.dir(id).join("spec.json")
    }

    pub fn state_path(&self, id: &str) -> PathBuf {
        self.dir(id).join("state.json")
    }

    pub fn exists(&self, id: &str) -> bool {
        self.dir(id).exists()
    }

    pub fn create_container(&self, id: &str, spec: &Spec, state: &State) -> io::Result<()> {
        fs::create_dir_all(self.dir(id))?;
        write_json(self.spec_path(id), spec)?;
        write_json(self.state_path(id), state)?;
        Ok(())
    }

    pub fn load_spec(&self, id: &str) -> io::Result<Spec> {
        read_json(self.spec_path(id))
    }

    pub fn load_state(&self, id: &str) -> io::Result<State> {
        read_json(self.state_path(id))
    }

    pub fn save_state(&self, id: &str, state: &State) -> io::Result<()> {
        write_json(self.state_path(id), state)
    }
}

fn write_json<T: serde::Serialize>(path: PathBuf, v: &T) -> io::Result<()> {
    let s = serde_json::to_string_pretty(v).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    fs::write(path, s)
}

fn read_json<T: serde::de::DeserializeOwned>(path: PathBuf) -> io::Result<T> {
    let s = fs::read_to_string(path)?;
    serde_json::from_str(&s).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}
