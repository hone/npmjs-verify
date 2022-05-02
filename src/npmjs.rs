mod package;

pub use package::*;

use reqwest::{header, Client as ReqwestClient, ClientBuilder};

const NPMJS_REGISTRY_BASE: &str = "https://registry.npmjs.org";

pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub fn new(auth: Option<String>) -> Result<Client, reqwest::Error> {
        Ok(Client {
            client: client(auth)?,
        })
    }

    pub async fn package(&self, name: &str) -> Result<Option<Package>, reqwest::Error> {
        let response = self
            .client
            .get(format!("{NPMJS_REGISTRY_BASE}/{name}"))
            .send()
            .await?;

        if response.status().is_success() {
            Ok(Some(response.json::<Package>().await?))
        } else {
            Ok(None)
        }
    }
}

fn client(auth: Option<String>) -> Result<ReqwestClient, reqwest::Error> {
    let mut builder = ClientBuilder::new();

    if let Some(auth) = auth {
        let mut headers = header::HeaderMap::new();
        let mut auth_value = header::HeaderValue::from_str(&format!("Bearer {auth}")).unwrap();
        auth_value.set_sensitive(true);
        headers.insert(header::AUTHORIZATION, auth_value);
        builder = builder.default_headers(headers);
    }

    builder.build()
}
