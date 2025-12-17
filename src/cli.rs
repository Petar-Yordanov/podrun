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
}
