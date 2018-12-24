//! Structures for pasing .cargo/config
use serde_derive::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::PathBuf};

/// .cargo/config
#[derive(Deserialize, Debug, Serialize, Default)]
#[serde(rename_all = "kebab-case")]
#[allow(clippy::stutter)]
pub struct CargoConfig {
    pub paths: Option<Vec<PathBuf>>,
    pub cargo_new: Option<CargoNew>,
}

#[derive(Deserialize, Debug, Serialize, Default)]
pub struct CargoNew {
    pub name: Option<String>,
    pub email: Option<String>,
    pub vcs: Option<String>,
}
