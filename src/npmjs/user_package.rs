use serde::Deserialize;
use std::collections::HashMap;

/// NPMJS API Endpoint `/-/{user}/package` format
pub type UserPackage = HashMap<String, Mode>;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Read/Write access for a particular package
pub enum Mode {
    Read,
    Write,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses() {
        let input = include_str!("../../fixtures/user_package.json");
        let result = serde_json::from_str::<UserPackage>(input);

        assert!(result.is_ok());
    }
}
