use crate::runtime::{Result, container::Container};

pub fn cmd_delete(id: String) -> Result<()> {
    let c = Container::open(id)?;
    c.delete()?;
    Ok(())
}
