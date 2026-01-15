use crate::runtime::{state::Status, store::Store, Result};

pub fn cmd_list() -> Result<()> {
    let store = Store::new()?;
    let ids = store.list_ids()?;

    if ids.is_empty() {
        return Ok(());
    }

    println!(
        "{:<24} {:<10} {:<8} {:<12} {:<12}",
        "id", "status", "pid", "created", "started"
    );

    for id in ids {
        let mut state = match store.load_state(&id) {
            Ok(s) => s,
            Err(err) => {
                println!(
                    "{:<24} {:<10} {:<8} {:<12} {:<12}  (state read error: {err})",
                    id, "?", "?", "?", "?"
                );
                continue;
            }
        };

        if matches!(state.status, Status::Running) {
            let pid_alive = state
                .pid
                .map(|pid| std::path::Path::new("/proc").join(pid.to_string()).exists())
                .unwrap_or(false);

            if !pid_alive {
                state.status = Status::Stopped;
                state.pid = None;
                let _ = store.save_state(&id, &state);
            }
        }

        let pid_str = state.pid.map(|p| p.to_string()).unwrap_or_else(|| "-".into());
        let started_str = state
            .started_at_unix
            .map(|t| t.to_string())
            .unwrap_or_else(|| "-".into());

        println!(
            "{:<24} {:<10} {:<8} {:<12} {:<12}",
            id,
            format!("{:?}", state.status),
            pid_str,
            state.created_at_unix,
            started_str
        );
    }

    Ok(())
}
