use std::future::Future;

use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct AnalizeRequest {
    pub content: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArticleEntity {
    pub name: String,
    #[serde(rename = "type")]
    pub entity_type: String,
    pub salience: f32,
}

pub fn get_entities<'a>(
    api_key: &'a String,
    request: &'a AnalizeRequest,
) -> impl Future<Output = Result<Vec<ArticleEntity>, String>> + 'a {
    async move {
        let entities_request = AnalizeEntitiesRequest {
            document: Document {
                content: request.content.clone(),
                content_type: "PLAIN_TEXT".to_string(),
                language: "en".to_string(),
                reference_web_url: request.url.clone(),
                boilderplate_handling: "NONE".to_string(),
            },
            encoding_type: "UTF8".to_string(),
        };

        let response = Client::new()
            .post("https://language.googleapis.com/v1beta2/documents:analyzeEntities")
            .query(&[("key", api_key)])
            .json(&entities_request)
            .send()
            .await;

        return match response {
            Ok(response) => response
                .json::<AnalizedEntitiesResponse>()
                .await
                .map(|response| response.entities)
                .map_err(|err| err.to_string()),
            Err(err) => Err(err.to_string()),
        };
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct AnalizeEntitiesRequest {
    document: Document,
    #[serde(rename = "encodingType")]
    encoding_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Document {
    content: String,
    language: String,
    #[serde(rename = "type")]
    content_type: String,
    #[serde(rename = "referenceWebUrl")]
    reference_web_url: String,
    #[serde(rename = "boilderplateHandling")]
    boilderplate_handling: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AnalizedEntitiesResponse {
    entities: Vec<ArticleEntity>,
}
