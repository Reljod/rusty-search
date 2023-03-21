use anyhow::{Context, Result};
use clap::Parser;
use dotenv::dotenv;
use hyper::Client;
use hyper_tls::HttpsConnector;
use serde::{Deserialize, Serialize};
use std::env;
use url::Url;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The query to search for on Google
    query: String,

    /// The number of results to return
    #[arg(short, long, default_value = "1")]
    num_count: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Response {
    items: Option<Vec<Item>>,
    search_information: Option<SearchInformation>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SearchInformation {
    search_time: f64,
    total_results: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Item {
    title: String,
    link: String,
    snippet: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Rusty Search\n");

    dotenv().ok();

    let meta_template = "Search time: <search_time>; Total results: <total_results>\n";
    let search_template = "[<index>] <link>\n<snippet>\n";

    let args = Cli::parse();

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let mut url = Url::parse("https://www.googleapis.com/customsearch/v1")?;

    let key = env::var("GOOGLE_SEARCH_API_KEY").expect("GOOGLE_SEARCH_API_KEY must be set");
    let cx = env::var("GOOGLE_SEARCH_CX").expect("GOOGLE_SEARCH_CX must be set");

    url.query_pairs_mut().append_pair("key", &key);
    url.query_pairs_mut().append_pair("cx", &cx);
    url.query_pairs_mut().append_pair("q", args.query.as_str());

    let uri = url.as_str().parse()?;

    let resp = client
        .get(uri)
        .await
        .with_context(|| "Failed to get response")?;

    match resp.status() {
        hyper::StatusCode::OK => (),
        _ => {
            return Err(
                anyhow::anyhow!("Failed to get response, status code: {}", resp.status()).into(),
            );
        }
    }

    let buf = hyper::body::to_bytes(resp)
        .await
        .with_context(|| "Failed to get body from response")?;
    let response: Response =
        serde_json::from_slice(&buf).with_context(|| "Failed to parse body to JSON")?;

    if response.search_information.is_some() {
        let search_information = response.search_information.unwrap();

        let mut meta = meta_template.replace(
            "<search_time>",
            search_information.search_time.to_string().as_str(),
        );
        meta = meta.replace("<total_results>", search_information.total_results.as_str());
        println!("{}", meta);
    }

    if response.items.is_none() {
        println!("[0] No results found");
        return Ok(());
    }

    for (ind, item) in response
        .items
        .unwrap()
        .iter()
        .take(args.num_count.unwrap())
        .enumerate()
    {
        // get items index count
        let mut search = search_template.replace("<index>", (ind + 1).to_string().as_str());
        search = search.replace("<link>", item.link.as_str());
        search = search.replace("<snippet>", item.snippet.as_str());
        println!("{}", search);
    }

    Ok(())
}
