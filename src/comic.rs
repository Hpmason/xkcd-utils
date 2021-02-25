use crate::error::XKCDError;
use rand::{
    Rng,
    SeedableRng,
    rngs::StdRng
};

use serde::{Serialize, Deserialize};
use async_trait::async_trait;

use crate::{search::find_comic, xkcd::XKCD};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Comic {
    month: String,
    pub num: u32,
    link: String,
    year: String,
    news: String,
    safe_title: String,
    pub transcript: String,
    pub alt: String,
    pub img: String,
    pub title: String,
    day: String,
}

impl Comic {
    pub async fn get_random() -> Result<Self, XKCDError> {
        let home_page = Self::get_home().await?;
        let mut rng = StdRng::from_entropy();
        let comic = Self::get_page(rng.gen_range(0..home_page.num)).await?;
        Ok(comic)
    }
    
    pub async fn search(s: &str) -> Result<Self, XKCDError>{
        let url = find_comic(s).await?;
        let comic = Self::get(format!("{}/{}", url, Self::SUFFIX)).await?;
        Ok(comic)
    }
}

#[async_trait]
impl XKCD<Self> for Comic {
    const BASE_URL: &'static str = "https://www.xkcd.com";
    const SUFFIX: &'static str = "info.0.json";

    async fn get<S: ToString + Send>(url: S) -> Result<Self, XKCDError> {
        let url = url.to_string();
        println!("Getting {}", url);
        let comic = reqwest::get(&url).await
            .unwrap()
            .json::<Comic>()
            .await?;
        Ok(comic)
    }

    async fn get_home() -> Result<Self, XKCDError> {
        let url = format!("{}/{}", Self::BASE_URL, Self::SUFFIX);
        let comic = Self::get(url).await?;
        Ok(comic)
    }

    async fn get_page(page_id: u32) -> Result<Self, XKCDError> {
        let url = format!("{}/{}/{}", Self::BASE_URL, page_id, Self::SUFFIX);
        let comic = Self::get(url).await?;
        Ok(comic)
    }
}


