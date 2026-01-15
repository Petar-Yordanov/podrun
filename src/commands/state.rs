use crate::runtime::{container::Container, Result, RuntimeError};
use serde::Serialize;

#[derive(Serialize)]
struct StateView<'a> {
    id: &'a str,
    status: crate::runtime::state::Status,
    pid: Option<i32>,
    pid_alive: bool,
    created_at_unix: u64,
    started_at_unix: Option<u64>,
}

pub fn cmd_state(id: String, json: bool) -> Result<()> {
    let mut c = Container::open(id)?;
    c.refresh_state()?; // ensure Running isnt stale (get state doesnt refresh)

    let state = c.get_state();

    let pid_alive = state
        .pid
        .map(|pid| std::path::Path::new("/proc").join(pid.to_string()).exists())
        .unwrap_or(false);

    let view = StateView {
        id: c.get_id(),
        status: state.status,
        pid: state.pid,
        pid_alive,
        created_at_unix: state.created_at_unix,
        started_at_unix: state.started_at_unix,
    };

    if json {
        let s = serde_json::to_string_pretty(&view)
            .map_err(|e| RuntimeError::Msg(e.to_string()))?;
        println!("{s}");
    } else {
        println!("id: {}", view.id);
        println!("status: {:?}", view.status);
        println!("pid: {:?}", view.pid);
        println!("pid_alive: {}", view.pid_alive);
        println!("created_at_unix: {}", view.created_at_unix);
        println!("started_at_unix: {:?}", view.started_at_unix);
    }

    Ok(())
}
