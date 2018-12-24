#![warn(clippy::all)]
#![warn(clippy::pedantic)]
use serde_derive::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::PathBuf};
use toml::Value;

fn default_edition() -> String {
    "2015".into()
}

fn default_branch() -> Option<String> {
    Some("master".into())
}

fn default_service() -> Option<String> {
    Some("github".into())
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(untagged)]
pub enum StringOrBool {
    String(String),
    Bool(bool),
}

impl Default for StringOrBool {
    fn default() -> Self {
        StringOrBool::String("build.rs".into())
    }
}

type DependencyT = BTreeMap<String, Dependency>;

/// The root Cargo.toml
#[derive(Deserialize, Debug, Serialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct CargoToml {
    pub package: Package,
    pub badges: Option<Badges>,
    pub metadata: Option<BTreeMap<String, Value>>,
    pub dependencies: Option<DependencyT>,
    pub dev_dependencies: Option<DependencyT>,
    pub build_dependencies: Option<DependencyT>,
    pub target: Option<Target>,
    pub profile: Option<Profile>,
}

#[derive(Deserialize, Debug, Serialize, Default)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
    #[serde(default = "default_edition")]
    pub edition: String,
    #[serde(default)]
    pub build: StringOrBool,
    pub links: Option<String>,
    // FIXME: Real default is docs.rs link with the current version
    pub documentation: Option<String>,
    // FIXME: Real default is seeded with VCS ignore, eg .gitignore
    pub exclude: Option<Vec<String>>,
    // FIXME: Overrides exclude, mutually exclusive.
    pub include: Option<Vec<String>>,
    pub publish: Option<bool>,
    // FIXME: Real default is inferred by looking up the directory tree.
    pub workspace: Option<PathBuf>,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub readme: Option<String>,
    // FIXME: Max length of 5
    pub keywords: Option<Vec<String>>,
    // FIXME: Max length of 5, and must match crates.io/category_slugs
    pub categories: Option<Vec<String>>,
    // FIXME: SPDX 2.1
    pub license: Option<String>,
    #[serde(rename = "license-file")]
    pub license_file: Option<String>,
}

#[derive(Deserialize, Debug, Serialize, Default)]
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

#[derive(Deserialize, Debug, Serialize, Default)]
pub struct Maintenance {
    // FIXME: Can only be values listed at https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata
    pub status: String,
}

/// These are more or less common to all currently supported badges.
#[derive(Deserialize, Debug, Serialize, Default)]
pub struct BuildBadge {
    // This is the only one valid for the is-it-maintained variants
    pub repository: String,
    #[serde(default = "default_branch")]
    pub branch: Option<String>,
    // FIXME: Valid values are only Github, bitbucket, and gitlab
    #[serde(default = "default_service")]
    pub service: Option<String>,
    // Only appveyor
    // FIXME: Assumes this is string but may not be?
    pub id: Option<String>,
    pub project_name: Option<String>,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(untagged)]
pub enum Dependency {
    Simple(String),
    Full(DependencyFull),
}

#[derive(Deserialize, Debug, Serialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct DependencyFull {
    pub git: Option<String>,
    pub branch: Option<String>,
    pub path: Option<PathBuf>,
    pub version: Option<String>,
    pub features: Option<Vec<String>>,
    pub default_features: Option<bool>,
    pub optional: Option<bool>,
    pub package: Option<String>,
}

#[derive(Deserialize, Debug, Serialize, Default)]
#[serde(transparent)]
pub struct Target {
    targets: BTreeMap<String, TargetDep>,
}

#[derive(Deserialize, Debug, Serialize, Default)]
pub struct TargetDep {
    pub dependencies: Option<DependencyT>,
    pub dev_dependencies: Option<DependencyT>,
    pub build_dependencies: Option<DependencyT>,
}

#[derive(Deserialize, Debug, Serialize, Default)]
#[serde(transparent)]
pub struct Profile {
    profiles: BTreeMap<String, ProfileVal>,
}

#[derive(Deserialize, Debug, Serialize, Default)]
#[serde(rename_all = "kebab-case")]
// FIXME: Defaults here depend on what the profile actually is, so different
// There are only four currently supported profiles, so i could manually do them?
pub struct ProfileVal {
    pub opt_level: Option<u64>,
    pub debug: Option<bool>,
    pub rpath: Option<bool>,
    pub lto: Option<bool>,
    pub debug_assertions: Option<bool>,
    pub codegen_units: Option<u64>,
    pub panic: Option<String>,
    pub incremental: Option<bool>,
    pub overflow_checks: Option<bool>,
}
