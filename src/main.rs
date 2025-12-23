use clap::Parser;

mod cli;
mod commands;
mod linux;
mod runtime;

fn main() -> runtime::Result<()> {
    let cli = cli::Cli::parse();
    match cli.cmd {
        cli::Cmd::Create { id, rootfs, argv } => commands::create::cmd_create(id, rootfs, argv)?,
        cli::Cmd::Start { id } => commands::start::cmd_start(id)?,
        cli::Cmd::Kill { id, signal } => commands::kill::cmd_kill(id, signal)?,
        cli::Cmd::Delete { id } => commands::delete::cmd_delete(id)?,
        cli::Cmd::Wait { id } => commands::wait::cmd_wait(id)?,
        cli::Cmd::Exec { id, argv, env, cwd } => commands::exec::cmd_exec(id, argv, env, cwd)?,
    }
    Ok(())
}
