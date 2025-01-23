use std::{ffi::OsString, path::PathBuf};

use crate::workout::{Category, Equipment, Force, Level, Mechanic};

use clap::{command, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "swt")]
#[command(about = "Efficiently query workout exercices", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Path to the data directory (includes JSON and images)
    #[arg(long = "data")]
    pub data_path: Option<PathBuf>,

    #[command(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Get all data relating to a specific exercise
    Get {
        /// ID of the exercise
        exercise_id: OsString,
    },
    /// Get the list of exercises matching the filters
    #[command(alias = "ls")]
    List {
        #[arg(short = 'f', long = "force")]
        force: Option<Force>,

        #[arg(short = 'l', long = "level")]
        level: Option<Level>,

        #[arg(short = 'm', long = "mechanic")]
        mechanic: Option<Mechanic>,

        #[arg(short = 'e', long = "equipment")]
        equipment: Option<Equipment>,

        #[arg(short = 'c', long = "category")]
        category: Option<Category>,
    },
}
