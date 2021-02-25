
use std::{collections::HashMap};

use serde::{Serialize, Deserialize};
use async_trait::async_trait;

use crate::{error::XKCDError, xkcd::XKCD};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ExplainXKCD {
    pub num: u32,
    pub title: String,
    pub explanation: String,
    pub permalink: String,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Redirect {
    from: String,
    to: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct ExplainParse {
    // format: "num: page_title"
    title: String,
    // refers to the 
    pageid: u32,
    wikitext: HashMap<String, String>,
    redirects: Vec<Redirect>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ExplainRequest {
    parse: ExplainParse,
}

impl ExplainParse {
    fn into_explain_xkcd(self) -> ExplainXKCD {
        let num = self.redirects[0].from.parse().unwrap();
        let mut split = self.title.split(":");
        // Clear id number
        split.next();
        ExplainXKCD {
            num: num,
            title: split.next().unwrap().to_string(),
            explanation: self.wikitext.get("*").unwrap().to_owned(),
            permalink: format!("https://www.explainxkcd.com/wiki/index.php/{}", num),
        }
    }
}


#[async_trait]
impl XKCD<Self> for ExplainXKCD {
    const BASE_URL: &'static str = "https://explainxkcd.com/wiki/api.php?action=parse&origin=*&redirects=1&prop=wikitext&sectiontitle=Explanation&format=json&page=";
    const SUFFIX: &'static str = "";

    async fn get<S: ToString + Send>(url: S) -> Result<Self, XKCDError> {
        let url = url.to_string();
        let ex_parse = reqwest::get(&url)
            .await?
            .json::<ExplainRequest>()
            .await?
            .parse;

        Ok(ex_parse.into_explain_xkcd())
    }
    async fn get_home() -> Result<Self, XKCDError> {
        let url = Self::BASE_URL;
        let ex = Self::get(url).await?;
        Ok(ex)
    }

    async fn get_page(page_id: u32) -> Result<Self, XKCDError> {
        let url = format!("{}{}", Self::BASE_URL, page_id);
        let ex = Self::get(url).await?;
        Ok(ex)
    }
}

