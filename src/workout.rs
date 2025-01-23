use std::path::PathBuf;

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Exercise {
    pub id: String,
    pub name: String,
    pub force: Option<Force>,
    pub level: Level,
    pub mechanic: Option<Mechanic>,
    pub equipment: Option<Equipment>,
    #[serde(rename = "primaryMuscles")]
    pub primary_muscles: Vec<Muscle>,
    #[serde(rename = "secondaryMuscles")]
    pub secondary_muscles: Vec<Muscle>,
    pub instructions: Vec<String>,
    pub category: Category,
    pub images: Vec<PathBuf>,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Force {
    #[value(name = "pull")]
    #[serde(rename = "pull")]
    Pull,
    #[value(name = "push")]
    #[serde(rename = "push")]
    Push,
    #[value(name = "static")]
    #[serde(rename = "static")]
    Static,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Level {
    #[value(name = "beginner", alias = "easy", alias = "1")]
    #[serde(rename = "beginner")]
    Beginner,
    #[value(name = "intermediate", alias = "medium", alias = "2")]
    #[serde(rename = "intermediate")]
    Intermediate,
    #[value(name = "expert", alias = "hard", alias = "3")]
    #[serde(rename = "expert")]
    Expert,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Mechanic {
    #[value(name = "compound", alias = "comp", alias = "cmp")]
    #[serde(rename = "compound")]
    Compound,
    #[value(name = "isolation", alias = "isol", alias = "iso")]
    #[serde(rename = "isolation")]
    Isolation,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Equipment {
    #[value(name = "bands")]
    #[serde(rename = "bands")]
    Bands,
    #[value(name = "barbell")]
    #[serde(rename = "barbell")]
    Barbell,
    #[value(name = "body only")]
    #[serde(rename = "body only")]
    BodyOnly,
    #[value(name = "cable")]
    #[serde(rename = "cable")]
    Cable,
    #[value(name = "dumbbell")]
    #[serde(rename = "dumbbell")]
    Dumbbell,
    #[value(name = "exercise ball")]
    #[serde(rename = "exercise ball")]
    ExerciseBall,
    #[value(name = "e-z curl bar")]
    #[serde(rename = "e-z curl bar")]
    EZCurlBar,
    #[value(name = "foam roll")]
    #[serde(rename = "foam roll")]
    FoamRoll,
    #[value(name = "kettlebells")]
    #[serde(rename = "kettlebells")]
    Kettlebells,
    #[value(name = "machine")]
    #[serde(rename = "machine")]
    Machine,
    #[value(name = "medicine ball")]
    #[serde(rename = "medicine ball")]
    MedicineBall,
    #[value(name = "other")]
    #[serde(rename = "other")]
    Other,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Category {
    #[value(name = "cardio")]
    #[serde(rename = "cardio")]
    Cardio,
    #[value(name = "olympic weightlifting")]
    #[serde(rename = "olympic weightlifting")]
    OlympicWeightlifting,
    #[value(name = "plyometrics")]
    #[serde(rename = "plyometrics")]
    Plyometrics,
    #[value(name = "powerlifting")]
    #[serde(rename = "powerlifting")]
    Powerlifting,
    #[value(name = "strength")]
    #[serde(rename = "strength")]
    Strength,
    #[value(name = "stretching")]
    #[serde(rename = "stretching")]
    Stretching,
    #[value(name = "strongman")]
    #[serde(rename = "strongman")]
    Strongman,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Muscle {
    #[value(name = "abdominals")]
    #[serde(rename = "abdominals")]
    Abdominals,
    #[value(name = "abductors")]
    #[serde(rename = "abductors")]
    Abductors,
    #[value(name = "adductors")]
    #[serde(rename = "adductors")]
    Adductors,
    #[value(name = "biceps")]
    #[serde(rename = "biceps")]
    Biceps,
    #[value(name = "calves")]
    #[serde(rename = "calves")]
    Calves,
    #[value(name = "chest")]
    #[serde(rename = "chest")]
    Chest,
    #[value(name = "forearms")]
    #[serde(rename = "forearms")]
    Forearms,
    #[value(name = "glutes")]
    #[serde(rename = "glutes")]
    Glutes,
    #[value(name = "hamstrings")]
    #[serde(rename = "hamstrings")]
    Hamstrings,
    #[value(name = "lats")]
    #[serde(rename = "lats")]
    Lats,
    #[value(name = "lower back")]
    #[serde(rename = "lower back")]
    LowerBack,
    #[value(name = "middle back")]
    #[serde(rename = "middle back")]
    MiddleBack,
    #[value(name = "neck")]
    #[serde(rename = "neck")]
    Neck,
    #[value(name = "quadriceps")]
    #[serde(rename = "quadriceps")]
    Quadriceps,
    #[value(name = "shoulders")]
    #[serde(rename = "shoulders")]
    Shoulders,
    #[value(name = "traps")]
    #[serde(rename = "traps")]
    Traps,
    #[value(name = "triceps")]
    #[serde(rename = "triceps")]
    Triceps,
}
