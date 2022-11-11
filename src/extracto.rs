use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{env::var, future::Future};

#[derive(Serialize, Deserialize, Debug)]
pub struct ArticleMetadata {
    pub author: Option<String>,
    /// the parsed and cleaned article text
    pub content: String,
    /// a short about the article
    pub description: Option<String>,
    pub image: Option<String>,
    /// list of alternative links
    pub links: Vec<String>,
    pub published: Option<String>,
    /// original publisher
    pub source: Option<String>,
    pub title: Option<String>,
    /// time to read in second, 0 = unknown
    pub ttr: Option<i32>,
    pub url: Option<String>,
}

#[derive(Debug)]
pub struct ExtractoCredentials {
    pub url: String,
    pub api_key: String,
}

#[derive(Debug)]
pub struct ArticleError {
    pub error: String,
}

pub fn get_article_metadata<'a>(
    creds: &'a ExtractoCredentials,
    article_url: &'a str,
) -> impl Future<Output = Result<ArticleMetadata, ArticleError>> + 'a {
    async move {
        let response = Client::new()
            .get(&creds.url)
            .query(&[("url", article_url)])
            .bearer_auth(&creds.api_key)
            .send()
            .await;

        return match response {
            Ok(response) => response
                .json::<ArticleMetadata>()
                .await
                .map_err(|err| ArticleError {
                    error: err.to_string(),
                }),
            Err(err) => Err(ArticleError {
                error: err.to_string(),
            }),
        };
    }
}

#[derive(Debug)]
pub struct ExtractoCredentialsError {
    pub error: String,
}

pub fn get_creds() -> Result<ExtractoCredentials, ExtractoCredentialsError> {
    let url = var("EXTRACTO_URL").map_err(|err| ExtractoCredentialsError {
        error: err.to_string(),
    })?;

    let api_key = var("EXTRACTO_API_KEY").map_err(|err| ExtractoCredentialsError {
        error: err.to_string(),
    })?;

    Ok(ExtractoCredentials { url, api_key })
}
