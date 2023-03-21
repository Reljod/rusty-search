use clap::Parser;
use dotenv::dotenv;
use hyper::Client;
use hyper_tls::HttpsConnector;
use serde::{Deserialize, Serialize};
use std::env;
use url::Url;

#[derive(Parser)]
struct Cli {
    query: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    context: Context,
    items: Vec<Item>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Context {
    title: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Item {
    title: String,
    link: String,
    snippet: String,
}

#[tokio::main]
async fn main() {
    println!("Rusty Search\n");

    dotenv().ok();

    let args = Cli::parse();

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let mut url = Url::parse("https://www.googleapis.com/customsearch/v1").unwrap();

    let key = env::var("GOOGLE_SEARCH_API_KEY").expect("GOOGLE_SEARCH_API_KEY must be set");
    let cx = env::var("GOOGLE_SEARCH_CX").expect("GOOGLE_SEARCH_CX must be set");

    url.query_pairs_mut().append_pair("key", &key);
    url.query_pairs_mut().append_pair("cx", &cx);
    url.query_pairs_mut().append_pair("q", args.query.as_str());

    let uri = url.as_str().parse().unwrap();

    let resp = client.get(uri).await.unwrap();
    let buf = hyper::body::to_bytes(resp).await.unwrap();
    let response: Response = serde_json::from_slice(&buf).unwrap();

    for item in response.items {
        println!("Title: {}", item.title);
        println!("Link: {}", item.link);
        println!("Snippet: {}", item.snippet);
        println!("");
    }
}
