#![warn(clippy::all)]
#![warn(clippy::pedantic)]
use serde_derive::{Deserialize, Serialize};
use std::path::PathBuf;

fn default_edition() -> String {
    "2015".into()
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

/// The root Cargo.toml
#[derive(Deserialize, Debug, Serialize, Default)]
pub struct CargoToml {
    pub package: Package,
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
