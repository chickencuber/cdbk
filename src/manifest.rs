use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Manifest {
    pub name: String,
    pub version: String,
    pub payload: PathBuf,

    pub icon: Option<PathBuf>,
    pub description: Option<String>,

    #[serde(default)]
    pub categories: Vec<String>,
    #[serde(default)]
    pub terminal: bool,
}
