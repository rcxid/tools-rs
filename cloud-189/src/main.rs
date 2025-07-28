mod auth;
mod client;
mod const_val;
mod util;

use anyhow::Result;
use reqwest::Client;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    // let cloud = CloudAuthClient::try_new()?;
    // let res = cloud
    //     .login_by_password("", "")
    //     .await?;

    let s = "abcdefghijklmnopqrstuvwxyz";
    for i in s.chars() {
        for j in s.chars() {
            println!("{}", format!("{}{}", i, j));
        }
    }

    Ok(())
}
