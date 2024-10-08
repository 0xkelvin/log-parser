use clap::{Command, Parser, Subcommand};
use std::path::{Path, PathBuf};

#[derive(Debug, Parser)]
#[command(version, about, long_about= None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// parse booting logs
    HmiBooting {
        /// origin log file path
        #[arg(short, long)]
        source_path: Option<PathBuf>,
        /// path for the log files after parsing
        #[arg(short, long)]
        dest_path: Option<PathBuf>,
    },
    // parse cpu load
    // parse memory free
    // parse hmi power
    // parse token power
}
