use error::Error;
use http;
use resources::{Address, Card};
use params::Metadata;

#[derive(Serialize)]
pub struct OwnerParams<'a> {
    #[serde(skip_serializing_if = "Option::is_none")] pub address: Option<Address>,
    #[serde(skip_serializing_if = "Option::is_none")] pub email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")] pub name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")] pub phone: Option<&'a str>,
}

#[derive(Serialize)]
pub struct RedirectParams<'a> {
    return_url: &'a str,
}

#[derive(Default, Serialize)]
pub struct SourceParams<'a> {
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<&'static str>,

    #[serde(skip_serializing_if = "Option::is_none")] pub amount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")] pub currency: Option<&'a str>, // eg. "usd"
    #[serde(skip_serializing_if = "Option::is_none")] pub flow: Option<&'a str>, // (redirect, receiver, code_verification, none)
    #[serde(skip_serializing_if = "Option::is_none")] pub metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")] pub owner: Option<OwnerParams<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")] pub redirect: Option<RedirectParams<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")] pub token: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")] pub usage: Option<&'a str>, // (reusable, single-use)
}

#[derive(Debug, Deserialize)]
#[serde(tag = "object")]
pub enum Source {
    // BitcoinReceiver(...),

    #[serde(rename = "card")]
    Card(Card),
}

impl Source {
    pub fn create(params: SourceParams, key: &str) -> Result<Source, Error> {
        return http::post("/sources", key, params);
    }

    pub fn get(source_id: &str, key: &str) -> Result<Source, Error> {
        return http::get(&format!("/sources/{}", source_id), key);
    }

    pub fn update(source_id: &str, params: SourceParams, key: &str) -> Result<Source, Error> {
        return http::post(&format!("/source/{}", source_id), key, params);
    }
}
