use crate::{get_ipv6_list, DdnsClient, IpAddressType};
use async_trait::async_trait;
use clap::Parser;
use reqwest::StatusCode;

/// Simple program to update dynv6 ipv6 address!
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Your dynv6 zone name.
    #[arg(short, long)]
    zone: String,
    /// An HTTP token for this zone.
    #[arg(short, long)]
    token: String,
    #[arg(short, long, default_value_t = 60)]
    /// Update interval second
    interval: u64,
}

pub struct Dynv6 {
    /// Your dynv6 zone name.
    zone: String,
    /// An HTTP token for this zone.
    token: String,
    /// Update interval secs
    interval: u64,
    /// ipv6 update before
    record_ipv6: String,
}

impl From<Args> for Dynv6 {
    fn from(value: Args) -> Self {
        Dynv6 {
            zone: value.zone,
            token: value.token,
            interval: value.interval,
            record_ipv6: String::new(),
        }
    }
}

#[async_trait]
impl DdnsClient for Dynv6 {
    fn support_type(&self) -> Vec<IpAddressType> {
        vec![IpAddressType::Ipv6]
    }

    fn interval_secs(&self) -> u64 {
        self.interval
    }

    async fn update(&mut self) -> anyhow::Result<()> {
        let res = get_ipv6_list()?;
        if res.len() > 0 {
            let ipv6 = res[0].as_str();
            self.update_ip(ipv6).await?;
        }
        Ok(())
    }
}

impl Dynv6 {
    pub fn info_log(&self) {
        println!("zone: {} token: {}", self.zone, self.token);
    }

    async fn update_ip(&mut self, ipv6: &str) -> anyhow::Result<()> {
        if ipv6 != self.record_ipv6 {
            // zone: &str, token: &str, ipv6: &str
            let url = format!(
                "https://dynv6.com/api/update?zone={}&token={}&ipv6={}",
                self.zone, self.token, ipv6
            );
            let response = reqwest::get(url).await?;
            if response.status() == StatusCode::OK {
                self.record_ipv6 = ipv6.to_string();
            }
        }
        Ok(())
    }
}
