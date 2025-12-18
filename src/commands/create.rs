use crate::runtime::{Result, container::Container};
use std::path::PathBuf;

pub fn cmd_create(id: String, rootfs: PathBuf, argv: Vec<String>) -> Result<()> {
    let c = Container::create(id, rootfs, argv)?;
    println!("created {}", c.get_id());
    Ok(())
}
