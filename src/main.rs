mod extracto;
mod nlp_api;

use extracto::{get_article_metadata, get_creds};

#[tokio::main]
async fn main() {
    let creds = get_creds().expect("Could not get credentials");
    let article_url = "https://www.bbc.com/news/world-us-canada-54500000";
    let article_metadata = get_article_metadata(&creds, article_url)
        .await
        .expect("Could not get article metadata");

    println!("Article metadata: {:?}", article_metadata);
}
