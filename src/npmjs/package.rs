use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Package {
    pub name: String,
    pub description: String,
    #[serde(rename = "dist-tags")]
    pub dist_tags: HashMap<String, node_semver::Version>,
    pub versions: HashMap<node_semver::Version, Version>,
    pub readme: String,
    pub maintainers: Vec<Contributor>,
    pub time: HashMap<String, String>,
    pub author: Contributor,
    pub repository: Repository,
    pub readme_filename: String,
    #[serde(default)]
    pub users: HashMap<String, bool>,
    pub homepage: String,
    #[serde(default)]
    pub keywords: Vec<String>,
    #[serde(default)]
    pub contributors: Vec<Contributor>,
    pub bugs: Bugs,
    pub license: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub name: String,
    pub version: node_semver::Version,
    pub description: String,
    pub author: Contributor,
    pub bugs: Option<Bugs>,
    pub license: Option<String>,
    pub main: String,
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
pub struct Contributor {
    pub email: Option<String>,
    pub name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
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
pub struct Repository {
    r#type: String,
    url: String,
}

#[derive(Deserialize, Debug)]
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

        result.unwrap();
        //assert!(result.is_ok());
    }
}
