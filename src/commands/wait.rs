use crate::runtime::{Result, container::Container};

pub fn cmd_wait(id: String) -> Result<()> {
    let mut c = Container::open(id)?;
    let code = c.wait()?;
    println!("exitCode={code}");
    Ok(())
}
