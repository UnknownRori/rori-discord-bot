use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
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

pub struct QuoteAPI;

impl QuoteAPI {
    pub async fn fetch() -> Result<Quote, reqwest::Error> {
        Ok(reqwest::get("https://api.quotable.io/random")
            .await?
            .json::<Quote>()
            .await?)
    }
}
