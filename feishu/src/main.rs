use anyhow::{anyhow, Result};
use reqwest::header::HeaderValue;
use reqwest::{header, Client};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<()> {
    let app_id = "cli_a8347de63ff0d01c";
    let app_secret = "O23e0JIRalQYnqNEWRJyGdgaumDjdunO";
    // let t = "R822w62bdiIDNRkVCYZcBc9gnud";

    let res = Client::new()
        // https://open.feishu.cn/open-apis/auth/v3//internal
        .post("https://open.feishu.cn/open-apis/auth/v3/app_access_token/internal")
        .json(&json!({
            "app_id": app_id,
            "app_secret": app_secret
        }))
        .send()
        .await?
        .json::<Value>()
        .await?;

    let token = res
        .get("app_access_token")
        .ok_or(anyhow!("get token failed!"))?
        .as_str()
        .ok_or(anyhow!("type error"))?;

    let token = format!("Bearer {token}");

    println!("{token}");

    let res = Client::new()
        .get("https://open.feishu.cn/open-apis/sheets/v2/spreadsheets/R822w62bdiIDNRkVCYZcBc9gnud/values/Sheet1")
        .header(header::CONTENT_TYPE, HeaderValue::from_static("application/json; charset=utf-8"))
        .header(header::AUTHORIZATION, HeaderValue::from_str(token.as_str()).unwrap())
        .send()
        .await?
        .json::<Value>()
        .await?;

    println!("{}", res);

    Ok(())
}
