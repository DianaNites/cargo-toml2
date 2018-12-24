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
    pub target: Option<ConfigTarget>,
    pub registry: Option<Registry>,
    pub http: Option<Http>,
    pub build: Option<Build>,
    pub term: Option<Term>,
    pub net: Option<Net>,
    pub alias: Option<Alias>,
}

#[derive(Deserialize, Debug, Serialize, Default)]
pub struct CargoNew {
    pub name: Option<String>,
    pub email: Option<String>,
    pub vcs: Option<String>,
}

#[derive(Deserialize, Debug, Serialize, Default)]
#[serde(transparent)]
#[allow(clippy::stutter)]
pub struct ConfigTarget {
    targets: BTreeMap<String, ConfigTargetVal>,
}

#[derive(Deserialize, Debug, Serialize, Default)]
#[allow(clippy::stutter)]
pub struct ConfigTargetVal {
    linker: Option<String>,
    ar: Option<String>,
    runner: Option<String>,
    rustflags: Option<Vec<String>>,
}

#[derive(Deserialize, Debug, Serialize, Default)]
pub struct Registry {
    pub index: Option<String>,
    pub token: Option<String>,
    pub default: Option<String>,
}

#[derive(Deserialize, Debug, Serialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Http {
    pub proxy: Option<String>,
    pub timeout: Option<i64>,
    pub cainfo: Option<String>,
    pub check_revoke: Option<bool>,
    pub low_speed_limit: Option<i64>,
    pub multiplexing: Option<bool>,
    pub debug: Option<bool>,
}

#[derive(Deserialize, Debug, Serialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Build {
    pub jobs: Option<i64>,
    pub rustc: Option<String>,
    pub rustdoc: Option<String>,
    pub target: Option<String>,
    pub target_dir: Option<PathBuf>,
    pub rustflags: Option<Vec<String>>,
    pub incremental: Option<bool>,
    pub dep_info_basedir: Option<PathBuf>,
}

#[derive(Deserialize, Debug, Serialize, Default)]
pub struct Term {
    pub verbose: Option<bool>,
    pub color: Option<String>,
}

#[derive(Deserialize, Debug, Serialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Net {
    pub retry: Option<i64>,
    pub git_fetch_with_cli: Option<bool>,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(untagged)]
pub enum AliasType {
    Simple(String),
    List(Vec<String>),
}

#[derive(Deserialize, Debug, Serialize, Default)]
#[serde(transparent)]
pub struct Alias {
    pub aliases: Option<BTreeMap<String, AliasType>>,
}
