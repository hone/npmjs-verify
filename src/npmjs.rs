mod package;

pub use package::*;

const NPMJS_REGISTRY_BASE: &str = "https://registry.npmjs.org";

pub async fn package(name: &str) -> Result<Option<Package>, reqwest::Error> {
    let response = reqwest::get(format!("{NPMJS_REGISTRY_BASE}/{name}")).await?;

    if response.status().is_success() {
        Ok(Some(response.json::<Package>().await?))
    } else {
        Ok(None)
    }
}
