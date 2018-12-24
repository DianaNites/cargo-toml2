use serde_derive::{Deserialize, Serialize};

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
    // TODO: Real default is docs.rs link with the current version
    pub documentation: Option<String>,
    // TODO: Real default is seeded with VCS ignore, eg .gitignore
    pub exclude: Option<Vec<String>>,
    // FIXME: Mutually exclusive with exclude
    pub include: Option<Vec<String>>,
}
