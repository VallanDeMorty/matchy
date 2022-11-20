use std::future::Future;

use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Bookmark {
    #[serde(rename = "_id")]
    id: i32,
    #[serde(rename = "collection.$id")]
    collection_id: i32,
    created: DateTime<Utc>,
    domain: String,
    description: String,
    #[serde(rename = "lastUpdate")]
    last_update: DateTime<Utc>,
    link: String,
    tags: Vec<String>,
    title: String,
    #[serde(rename = "type")]
    bookmark_type: String,
    #[serde(rename = "user.$id")]
    user_id: i32,
}

#[derive(Debug)]
pub struct GetBookmarkError {
    pub error: String,
    pub r#type: GetBookmarkErrorType,
}

#[derive(Debug)]
pub enum GetBookmarkErrorType {
    NotFound,
    ConnectionError,
    ParsingError,
}

pub fn get_bookmark<'a>(
    token: &'a String,
    bookmark_id: i32,
) -> impl Future<Output = Result<Bookmark, GetBookmarkError>> + 'a {
    async move {
        let response = Client::new()
            .get(&format!(
                "https://api.raindrop.io/rest/v1/raindrop/{}",
                bookmark_id
            ))
            .bearer_auth(token)
            .send()
            .await;

        return match response {
            Ok(response) => response
                .json::<Bookmark>()
                .await
                .map_err(|err| GetBookmarkError {
                    error: err.to_string(),
                    r#type: GetBookmarkErrorType::ParsingError,
                }),
            Err(err) => Err(GetBookmarkError {
                error: err.to_string(),
                r#type: GetBookmarkErrorType::ConnectionError,
            }),
        };
    }
}

pub enum BookmarkType {
    Link,
    Article,
    Image,
    Video,
    Document,
    Audio,
}

pub fn parse_bookmark_type(bookmark: &Bookmark) -> Option<BookmarkType> {
    match bookmark.bookmark_type.as_str() {
        "article" => Some(BookmarkType::Article),
        "video" => Some(BookmarkType::Video),
        "image" => Some(BookmarkType::Image),
        "audio" => Some(BookmarkType::Audio),
        "link" => Some(BookmarkType::Link),
        "document" => Some(BookmarkType::Document),
        _ => None,
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct AddTagsRequest {
    tags: Vec<String>,
}

#[derive(Debug)]
pub struct AddTagsError {
    pub error: String,
    pub r#type: AddTagsErrorType,
}

#[derive(Debug)]
pub enum AddTagsErrorType {
    BookmarkIsNotFound,
    ConnectionError,
    ParsingError,
}

pub fn add_tags_to_bookmark<'a>(
    token: &'a String,
    bookmark: &'a Bookmark,
    tags: &'a Vec<String>,
) -> impl Future<Output = Option<AddTagsError>> + 'a {
    async move {
        let response = Client::new()
            .post(&format!(
                "https://api.raindrop.io/rest/v1/raindrop/{}",
                bookmark.id
            ))
            .bearer_auth(token)
            .json(&AddTagsRequest { tags: tags.clone() })
            .send()
            .await;

        return match response {
            Ok(response) => match response.status().is_success() {
                true => None,
                false => {
                    let status = response.status();

                    Some(AddTagsError {
                        error: response.text().await.unwrap(),
                        r#type: match status.as_u16() {
                            404 => AddTagsErrorType::BookmarkIsNotFound,
                            _ => AddTagsErrorType::ConnectionError,
                        },
                    })
                }
            },
            Err(err) => Some(AddTagsError {
                error: err.to_string(),
                r#type: AddTagsErrorType::ConnectionError,
            }),
        };
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Collection {
    id: i32,
    title: String,
}

pub fn get_collection<'a>(
    token: &'a String,
    collection_id: i32,
) -> impl Future<Output = Result<Collection, String>> + 'a {
    async move {
        let response = Client::new()
            .get(&format!(
                "https://api.raindrop.io/rest/v1/collection/{}",
                collection_id
            ))
            .bearer_auth(token)
            .send()
            .await;

        return match response {
            Ok(response) => response
                .json::<Collection>()
                .await
                .map_err(|err| err.to_string()),
            Err(err) => Err(err.to_string()),
        };
    }
}

pub fn get_tags<'a>(token: &'a String) -> impl Future<Output = Result<Vec<String>, String>> + 'a {
    async move {
        let response = Client::new()
            .get("https://api.raindrop.io/rest/v1/tags")
            .bearer_auth(token)
            .send()
            .await;

        return match response {
            Ok(response) => response
                .json::<Vec<String>>()
                .await
                .map_err(|err| err.to_string()),
            Err(err) => Err(err.to_string()),
        };
    }
}
