use crate::runtime::{Result, container::Container};

pub fn cmd_start(id: String) -> Result<()> {
    let mut c = Container::open(id)?;
    let pid = c.start()?;
    println!("started {} pid={pid}", c.get_id());
    Ok(())
}
