/// This file handle Quotes API from https://api.quotable.io/
///
/// Example Code
/// ```rust
/// fn fetch_quote() -> Result<(), reqwest::Error> {
///    let quote = QuoteAPI::fetch().await?;
///    dbg!(quote);
/// }
/// ```
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Quote {
    #[serde(rename = "_id")]
    pub id: String,

    #[serde(rename = "content")]
    pub content: String,

    #[serde(rename = "author")]
    pub author: String,

    #[serde(rename = "tags")]
    pub tags: Vec<String>,

    #[serde(rename = "authorSlug")]
    pub author_slug: String,

    #[serde(rename = "length")]
    pub length: i64,

    #[serde(rename = "dateAdded")]
    pub date_added: String,

    #[serde(rename = "dateModified")]
    pub date_modified: String,
}

/// This struct is just way to abstract away the Quote API
pub struct QuoteAPI;

impl QuoteAPI {
    /// Quote API Endpoint
    const BASE_URL: &str = "https://api.quotable.io/";

    pub async fn fetch() -> Result<Quote, reqwest::Error> {
        Ok(reqwest::get(format!("{}/random", Self::BASE_URL))
            .await?
            .json::<Quote>()
            .await?)
    }
}
