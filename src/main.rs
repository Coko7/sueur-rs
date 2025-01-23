use std::{fs, path::PathBuf};

use anyhow::{anyhow, Result};
use clap::Parser;
use cli::Cli;
use workout::{Category, Equipment, Exercise, Force, Level, Mechanic};

mod cli;
mod workout;

fn main() -> Result<()> {
    let args = Cli::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    process_args(args)?;
    Ok(())
}

fn process_args(args: Cli) -> Result<()> {
    let data_path = match args.data_path {
        Some(path) => path,
        None => PathBuf::from("./data"),
    };

    let exercises = load_exercises(data_path)?;

    match args.command {
        cli::Commands::Get { exercise_id } => {
            match exercises.iter().find(|exo| *exo.id == exercise_id) {
                Some(exo) => {
                    let serialized = serde_json::to_string(exo)?;
                    Ok(println!("{}", serialized))
                }
                None => Err(anyhow!(
                    "No such exercise: {}",
                    exercise_id.to_str().unwrap()
                )),
            }
        }
        cli::Commands::List {
            force,
            level,
            mechanic,
            equipment,
            category,
        } => filter_exercises(&exercises, force, level, mechanic, equipment, category),
    }
}

fn filter_exercises(
    exercises: &Vec<Exercise>,
    force: Option<Force>,
    level: Option<Level>,
    mechanic: Option<Mechanic>,
    equipment: Option<Equipment>,
    category: Option<Category>,
) -> Result<()> {
    let exos: Vec<_> = exercises
        .iter()
        .filter(|exo| force.map_or(true, |f| exo.force == Some(f)))
        .filter(|exo| level.map_or(true, |l| exo.level == l))
        .filter(|exo| mechanic.map_or(true, |m| exo.mechanic == Some(m)))
        .filter(|exo| equipment.map_or(true, |e| exo.equipment == Some(e)))
        .filter(|exo| category.map_or(true, |c| exo.category == c))
        .cloned()
        .collect();

    match exos.is_empty() {
        true => Err(anyhow!("No exercise found for these filters!")),
        false => {
            for exo in exos.iter() {
                println!("{}", exo.id)
            }
            Ok(())
        }
    }
}

fn load_exercises(data_path: PathBuf) -> Result<Vec<Exercise>> {
    let exo_json_path = data_path.join("exercises.json");
    let data = fs::read_to_string(exo_json_path)?;
    let exercises: Vec<Exercise> = serde_json::from_str(&data)?;
    Ok(exercises)
}
