mod package;

pub use package::*;

use reqwest::{header, Client as ReqwestClient, ClientBuilder};
use std::collections::HashMap;

const NPMJS_REGISTRY_BASE: &str = "https://registry.npmjs.org";

pub type UserPackage = HashMap<String, String>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Failed to fetch request: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("{message}: {source}")]
    Context {
        source: reqwest::Error,
        message: String,
    },
}

/// NPM API Client
pub struct Client {
    client: reqwest::Client,
}

impl Client {
    /// Construct a new NPM API Client with an optional provided auth token
    pub fn new(auth: Option<String>) -> Result<Client, reqwest::Error> {
        Ok(Client {
            client: client(auth)?,
        })
    }

    /// Fetch information for specified package
    pub async fn package(&self, name: &str) -> Result<Option<Package>, Error> {
        let response = self
            .client
            .get(format!("{NPMJS_REGISTRY_BASE}/{name}"))
            .send()
            .await
            .map_err(|err| Error::Context {
                source: err,
                message: format!("Fetching {name}"),
            })?;

        if response.status().is_success() {
            let json = response
                .json::<Package>()
                .await
                .map_err(|err| Error::Context {
                    source: err,
                    message: format!("Fetching {name}"),
                })?;

            Ok(Some(json))
        } else {
            Ok(None)
        }
    }

    /// Fetch all user packages
    pub async fn packages(&self, user: &str) -> Result<Option<UserPackage>, reqwest::Error> {
        let response = self
            .client
            .get(format!("{NPMJS_REGISTRY_BASE}/-/user/{user}/package"))
            .send()
            .await?;

        if response.status().is_success() {
            Ok(Some(response.json::<UserPackage>().await?))
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
