use crate::runtime::{Result, RuntimeError, container::Container};
use std::path::PathBuf;

pub fn cmd_exec(
    id: String,
    argv: Vec<String>,
    env: Vec<String>,
    cwd: Option<PathBuf>,
) -> Result<()> {
    let env = parse_env_kv(env)?;
    let c = Container::open(id)?;
    let exit_code = c.exec(argv, env, cwd)?;
    println!("exitCode={exit_code}");

    Ok(())
}

fn parse_env_kv(items: Vec<String>) -> Result<Vec<(String, String)>> {
    let mut out = Vec::with_capacity(items.len());

    for s in items {
        let (k, v) = s
            .split_once('=')
            .ok_or_else(|| RuntimeError::Msg(format!("invalid --env '{s}', expected KEY=VALUE")))?;

        if k.is_empty() {
            return Err(RuntimeError::Msg(format!("invalid --env '{s}', empty KEY")));
        }

        out.push((k.to_string(), v.to_string()));
    }

    Ok(out)
}
