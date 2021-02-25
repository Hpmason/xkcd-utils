use std::{env};

// use serde::{Deserialize, Serialize};
use serde_json::Value;

use lazy_static::lazy_static;

use crate::error::*;

const BASE_URL: &str = "https://www.googleapis.com/customsearch/v1/siterestrict?";
const CX: &str = "dddff420d41a056fa";

lazy_static! {
    static ref KEY: String = env::var("GOOGLE_SEARCH_KEY")
        .expect("Expected a key in the environment");
}


pub async fn find_comic(s: &str) -> Result<String, XKCDError> {
    let url_wo_key = format!("{}num=1&cx={}&q={}", BASE_URL, CX, s);
    let url = format!("{}&key={}", url_wo_key, *KEY);
    println!("Search URL: {}", url);
    let res = reqwest::get(&url).await
        .unwrap();
    let search = res.json::<Value>()
        .await?;

    println!("Found: {:?}", search["items"][0]["link"]);
    let link = search["items"][0]["link"]
        .as_str();
    if let Some(link) = link {
        let link = link.to_string();
        return Ok(link);
    }
    Err(XKCDError::SearchError(s.to_string()))
    // Ok(search.items[0].link.to_owned())
    
}
