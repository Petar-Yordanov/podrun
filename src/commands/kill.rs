use crate::runtime::{Result, container::Container};

pub fn cmd_kill(id: String, signal: i32) -> Result<()> {
    let mut c = Container::open(id)?;
    c.kill(signal)?;
    Ok(())
}
