#[cfg(feature = "logging")]
use log::{debug, log_enabled, Level};
use oauth10a::client::{ClientError, RestClient};
#[cfg(feature = "jsonschemas")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::Client;

// -----------------------------------------------------------------------------
// Myself structure and helpers

#[cfg_attr(feature = "jsonschemas", derive(JsonSchema))]
#[derive(Serialize, PartialEq, Eq, Deserialize, Clone, Debug)]
pub struct Myself {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "phone")]
    pub phone: String,
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "city")]
    pub city: String,
    #[serde(rename = "zipcode")]
    pub zipcode: String,
    #[serde(rename = "country")]
    pub country: String,
    #[serde(rename = "avatar")]
    pub avatar: String,
    #[serde(rename = "creationDate")]
    pub creation_date: u64,
    #[serde(rename = "lang")]
    pub lang: String,
    #[serde(rename = "emailValidated")]
    pub email_validated: bool,
    #[serde(rename = "oauthApps")]
    pub oauth_apps: Vec<String>,
    #[serde(rename = "admin")]
    pub admin: bool,
    #[serde(rename = "canPay")]
    pub can_pay: bool,
    #[serde(rename = "preferredMFA")]
    pub preferred_mfa: String,
    #[serde(rename = "hasPassword")]
    pub has_password: bool,
}

// -----------------------------------------------------------------------------
// Helpers functions

#[cfg_attr(feature = "trace", tracing::instrument)]
/// returns information about the person logged in
pub async fn get(client: &Client) -> Result<Myself, ClientError> {
    let path = format!("{}/v2/self", client.endpoint);

    #[cfg(feature = "logging")]
    if log_enabled!(Level::Debug) {
        debug!(
            "execute a request to get information about the logged in user, path: '{}'",
            &path
        );
    }

    client.get(&path).await
}
