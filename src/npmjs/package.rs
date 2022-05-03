use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Package {
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "dist-tags")]
    pub dist_tags: HashMap<String, node_semver::Version>,
    pub versions: HashMap<node_semver::Version, Version>,
    pub readme: String,
    pub maintainers: Vec<Contributor>,
    pub time: HashMap<String, String>,
    pub author: Option<Contributor>,
    pub repository: Option<Repository>,
    pub readme_filename: String,
    #[serde(default)]
    pub users: HashMap<String, bool>,
    pub homepage: Option<String>,
    #[serde(default)]
    pub keywords: Vec<String>,
    #[serde(default)]
    pub contributors: Vec<Contributor>,
    pub bugs: Option<Bugs>,
    pub license: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Version {
    pub name: String,
    pub version: node_semver::Version,
    pub description: Option<String>,
    pub author: Option<Contributor>,
    pub bugs: Option<Bugs>,
    pub license: Option<String>,
    pub main: Option<String>,
    #[serde(default)]
    pub scripts: HashMap<String, String>,
    pub homepage: Option<String>,
    #[serde(default)]
    pub keywords: Vec<String>,
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
    #[serde(default)]
    pub dev_dependencies: HashMap<String, String>,
    pub dist: Dist,
    #[serde(default)]
    pub contributors: Vec<Contributor>,
    pub git_head: Option<String>,
    #[serde(default)]
    pub maintainers: Vec<Contributor>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
#[allow(dead_code)]
pub enum Contributor {
    Object {
        email: Option<String>,
        name: Option<String>,
    },
    String(String),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct Dist {
    pub shasum: String,
    pub tarball: String,
    pub integrity: Option<String>,
    pub file_count: Option<usize>,
    pub unpacked_size: Option<usize>,
    #[serde(rename = "npm-signature")]
    pub npm_signature: Option<String>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Repository {
    r#type: String,
    url: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Bugs {
    url: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses() {
        let input = include_str!("../../fixtures/light-cycle.json");
        let result = serde_json::from_str::<Package>(input);

        assert!(result.is_ok());
    }

    #[test]
    fn it_parses_empty_contributor_object() {
        let input = include_str!("../../fixtures/ember-slds-components.json");
        let result = serde_json::from_str::<Package>(input);

        assert!(result.is_ok());
    }
}
