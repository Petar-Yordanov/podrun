use crate::runtime::{Result, spec::Spec, state::State};
use std::{fs, io, path::PathBuf};

pub struct Store {
    root: PathBuf,
}

impl Store {
    pub fn new() -> Result<Self> {
        let home = std::env::var_os("HOME")
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "HOME not set"))?;
        Ok(Self {
            root: PathBuf::from(home).join(".podrun").join("containers"),
        })
    }

    pub fn exists(&self, id: &str) -> bool {
        self.root.join(id).exists()
    }

    pub fn create_container(&self, id: &str, spec: &Spec, state: &State) -> Result<()> {
        fs::create_dir_all(self.root.join(id))?;
        write_json(self.root.join(id).join("spec.json"), spec)?;
        write_json(self.root.join(id).join("state.json"), state)?;
        Ok(())
    }

    pub fn load_spec(&self, id: &str) -> Result<Spec> {
        read_json(self.root.join(id).join("spec.json"))
    }

    pub fn load_state(&self, id: &str) -> Result<State> {
        read_json(self.root.join(id).join("state.json"))
    }

    pub fn save_state(&self, id: &str, state: &State) -> Result<()> {
        write_json(self.root.join(id).join("state.json"), state)
    }
}

fn write_json<T: serde::Serialize>(path: PathBuf, v: &T) -> Result<()> {
    let s = serde_json::to_string_pretty(v).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    fs::write(path, s)?;
    Ok(())
}

fn read_json<T: serde::de::DeserializeOwned>(path: PathBuf) -> Result<T> {
    let s = fs::read_to_string(path)?;
    serde_json::from_str(&s).map_err(|e| io::Error::new(io::ErrorKind::Other, e).into())
}
