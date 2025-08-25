use anyhow::Result;
use common::time::timestamp_ms;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};

pub struct BossCrawler {
    client: Client,
    headers: HeaderMap,
    params: BossCrawlerParams,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BossCrawlerParams {
    page: u64,
    page_size: u64,
    city: String,
    query: String,
    #[serde(rename = "_")]
    timestamp: u64,
}

impl BossCrawler {
    pub fn try_new(headers: HeaderMap, params: BossCrawlerParams) -> Result<Self> {
        let mut default_headers = HeaderMap::new();
        default_headers.insert(
            header::REFERER,
            HeaderValue::from_str("https://www.zhipin.com/")?,
        );
        Ok(Self {
            client: reqwest::ClientBuilder::new()
                .default_headers(default_headers)
                .build()?,
            headers,
            params,
        })
    }

    pub async fn job_list(&self) -> Result<()> {
        let res = self
            .client
            .get("https://www.zhipin.com/wapi/zpgeek/search/joblist.json")
            .query(&self.params)
            .headers(self.headers.clone())
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        println!("{}", res.to_string());
        Ok(())
    }
}

impl BossCrawlerParams {
    pub fn new(
        page: u64,
        page_size: u64,
        city: impl Into<String>,
        query: impl Into<String>,
    ) -> Self {
        Self {
            page,
            page_size,
            city: city.into(),
            query: query.into(),
            timestamp: timestamp_ms(),
        }
    }

    pub fn next_page(&mut self) {
        self.page += 1;
    }

    pub fn update_timestamp(&mut self) {
        self.timestamp = timestamp_ms();
    }
}
