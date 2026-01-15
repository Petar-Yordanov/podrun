use crate::runtime::{Result, spec::Spec};
use crate::runtime::state::State;
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

    pub fn dir(&self, id: &str) -> PathBuf {
        self.root.join(id)
    }

    pub fn list_ids(&self) -> crate::runtime::Result<Vec<String>> {
        let root = self.root.clone();

        if !root.exists() {
            return Ok(vec![]);
        }

        let mut ids = Vec::new();
        for entry in fs::read_dir(&root)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                ids.push(entry.file_name().to_string_lossy().into_owned());
            }
        }
        ids.sort();
        Ok(ids)
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
