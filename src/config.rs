//! Structures for pasing .cargo/config
use serde_derive::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::PathBuf};

/// .cargo/config
#[derive(Deserialize, Debug, Serialize, Default, Clone)]
#[serde(rename_all = "kebab-case")]
#[allow(clippy::module_name_repetitions)]
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

#[derive(Deserialize, Debug, Serialize, Default, Clone)]
pub struct CargoNew {
    pub name: Option<String>,
    pub email: Option<String>,
    pub vcs: Option<String>,
}

#[derive(Deserialize, Debug, Serialize, Default, Clone)]
#[serde(transparent)]
#[allow(clippy::module_name_repetitions)]
pub struct ConfigTarget {
    pub targets: BTreeMap<String, ConfigTargetVal>,
}

#[derive(Deserialize, Debug, Serialize, Default, Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct ConfigTargetVal {
    pub linker: Option<String>,
    pub ar: Option<String>,
    pub runner: Option<String>,
    pub rustflags: Option<Vec<String>>,
}

#[derive(Deserialize, Debug, Serialize, Default, Clone)]
pub struct Registry {
    pub index: Option<String>,
    pub token: Option<String>,
    pub default: Option<String>,
}

#[derive(Deserialize, Debug, Serialize, Default, Clone)]
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

#[derive(Deserialize, Debug, Serialize, Default, Clone)]
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

#[derive(Deserialize, Debug, Serialize, Default, Clone)]
pub struct Term {
    pub verbose: Option<bool>,
    pub color: Option<String>,
}

#[derive(Deserialize, Debug, Serialize, Default, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Net {
    pub retry: Option<i64>,
    pub git_fetch_with_cli: Option<bool>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum AliasType {
    Simple(String),
    List(Vec<String>),
}

#[derive(Deserialize, Debug, Serialize, Default, Clone)]
#[serde(transparent)]
pub struct Alias {
    pub aliases: Option<BTreeMap<String, AliasType>>,
}
