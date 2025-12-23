use crate::runtime::{Result, container::Container};

pub fn cmd_delete(id: String) -> Result<()> {
    let c = Container::open(id)?;
    let cid = c.get_id().to_string();
    c.delete()?;
    println!("deleted {}", cid);
    Ok(())
}
