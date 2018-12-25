//! Structures for pasing Cargo.toml
use serde_derive::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::PathBuf};
use toml::Value;

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum StringOrBool {
    String(String),
    Bool(bool),
}

type DependencyT = BTreeMap<String, Dependency>;

/// The root Cargo.toml
#[derive(Deserialize, Debug, Serialize, Default, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct CargoToml {
    pub package: Package,
    pub badges: Option<Badges>,
    pub dependencies: Option<DependencyT>,
    pub dev_dependencies: Option<DependencyT>,
    pub build_dependencies: Option<DependencyT>,
    pub target: Option<Target>,
    pub profile: Option<Profile>,
    pub features: Option<Features>,
    pub workspace: Option<Workspace>,
    #[serde(rename = "example")]
    pub examples: Option<Vec<TargetConfig>>,
    #[serde(rename = "bin")]
    pub bins: Option<Vec<TargetConfig>>,
    pub lib: Option<TargetConfig>,
    #[serde(rename = "bench")]
    pub benches: Option<Vec<TargetConfig>>,
    #[serde(rename = "test")]
    pub tests: Option<Vec<TargetConfig>>,
    pub patch: Option<Patches>,
    pub replace: Option<DependencyT>,
}

#[derive(Deserialize, Debug, Serialize, Default, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Package {
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
    pub edition: Option<String>,
    pub build: Option<StringOrBool>,
    pub links: Option<String>,
    pub documentation: Option<String>,
    pub exclude: Option<Vec<String>>,
    pub include: Option<Vec<String>>,
    pub publish: Option<bool>,
    pub workspace: Option<PathBuf>,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub readme: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
    pub license: Option<String>,
    pub license_file: Option<String>,
    pub autobins: Option<bool>,
    pub autoexamples: Option<bool>,
    pub autotests: Option<bool>,
    pub autobenches: Option<bool>,
    pub metadata: Option<BTreeMap<String, Value>>,
}

#[derive(Deserialize, Debug, Serialize, Default, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Badges {
    pub appveyor: Option<BuildBadge>,
    pub circle_ci: Option<BuildBadge>,
    pub gitlab: Option<BuildBadge>,
    pub travis_ci: Option<BuildBadge>,
    pub codecov: Option<BuildBadge>,
    pub coveralls: Option<BuildBadge>,
    pub is_it_maintained_issue_resolution: Option<BuildBadge>,
    pub is_it_maintained_open_issues: Option<BuildBadge>,
    pub maintenance: Option<Maintenance>,
}

#[derive(Deserialize, Debug, Serialize, Default, Clone)]
pub struct Maintenance {
    pub status: String,
}

/// These are more or less common to all currently supported badges.
#[derive(Deserialize, Debug, Serialize, Default, Clone)]
pub struct BuildBadge {
    // This is the only one valid for the is-it-maintained variants
    pub repository: String,
    pub branch: Option<String>,
    pub service: Option<String>,
    // Only appveyor
    pub id: Option<String>,
    pub project_name: Option<String>,
}

/// Due to issues with `toml-rs`, this will fail to serialize if both Simple and Full variants exist.
/// Specifically, issue #256 blocks this working properly.
#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum Dependency {
    Simple(String),
    Full(DependencyFull),
}

#[derive(Deserialize, Debug, Serialize, Default, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct DependencyFull {
    pub git: Option<String>,
    pub branch: Option<String>,
    pub tag: Option<String>,
    pub rev: Option<String>,
    pub path: Option<PathBuf>,
    pub version: Option<String>,
    pub features: Option<Vec<String>>,
    pub default_features: Option<bool>,
    pub optional: Option<bool>,
    pub package: Option<String>,
}

#[derive(Deserialize, Debug, Serialize, Default, Clone)]
#[serde(transparent)]
pub struct Target {
    pub targets: BTreeMap<String, TargetDep>,
}

#[derive(Deserialize, Debug, Serialize, Default, Clone)]
pub struct TargetDep {
    pub dependencies: Option<DependencyT>,
    pub dev_dependencies: Option<DependencyT>,
    pub build_dependencies: Option<DependencyT>,
}

#[derive(Deserialize, Debug, Serialize, Default, Clone)]
pub struct Profile {
    pub dev: Option<ProfileVal>,
    pub release: Option<ProfileVal>,
    pub test: Option<ProfileVal>,
    pub bench: Option<ProfileVal>,
}

#[derive(Deserialize, Debug, Serialize, Default, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct ProfileVal {
    pub opt_level: Option<i64>,
    pub debug: Option<bool>,
    pub rpath: Option<bool>,
    pub lto: Option<bool>,
    pub debug_assertions: Option<bool>,
    pub codegen_units: Option<u64>,
    pub panic: Option<String>,
    pub incremental: Option<bool>,
    pub overflow_checks: Option<bool>,
}

#[derive(Deserialize, Debug, Serialize, Default, Clone)]
pub struct Features {
    pub default: Option<Vec<String>>,
    #[serde(flatten)]
    pub features: BTreeMap<String, Vec<String>>,
}

#[derive(Deserialize, Debug, Serialize, Default, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Workspace {
    pub members: Option<Vec<String>>,
    pub default_members: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
}

/// All the sections here use the same stuff.
/// <https://doc.rust-lang.org/cargo/reference/manifest.html#configuring-a-target>
#[derive(Deserialize, Debug, Serialize, Default, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct TargetConfig {
    pub name: Option<String>,
    pub path: Option<PathBuf>,
    pub test: Option<bool>,
    pub doctest: Option<bool>,
    pub bench: Option<bool>,
    pub plugin: Option<bool>,
    pub proc_macro: Option<bool>,
    pub harness: Option<bool>,
    pub edition: Option<String>,
    pub required_features: Option<Vec<String>>,
    pub crate_type: Option<Vec<String>>,
}

#[derive(Deserialize, Debug, Serialize, Default, Clone)]
#[serde(transparent, rename_all = "kebab-case")]
pub struct Patches {
    pub sources: BTreeMap<String, DependencyT>,
}
