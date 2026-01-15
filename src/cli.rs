use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub cmd: Cmd,
}

#[derive(Subcommand)]
pub enum Cmd {
    Create {
        id: String,
        #[arg(long)]
        rootfs: PathBuf,
        #[arg(last = true, required = true)]
        argv: Vec<String>,
    },
    Start {
        id: String,
    },
    Kill {
        id: String,
        #[arg(long, default_value_t = 15)]
        signal: i32,
    },
    Delete {
        id: String,
    },
    Wait {
        id: String,
    },
    Exec {
        id: String,

        #[arg(long = "env")]
        env: Vec<String>,
        #[arg(long)]
        cwd: Option<PathBuf>,
        #[arg(last = true, required = true)]
        argv: Vec<String>,
    },
    State {
        id: String,
        #[arg(long)]
        json: bool,
    },
    List,
}
