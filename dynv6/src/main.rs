use anyhow::anyhow;
use clap::Parser;
use reqwest::StatusCode;
use std::{process::Command, time::Duration};

fn main() {
    let mut dynv6 = Dynv6::from(Args::parse());
    dynv6.info_log();
    dynv6.update();
}

/// Simple program to update dynv6 ipv6 address!
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
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

struct Dynv6 {
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

impl Dynv6 {
    fn info_log(&self) {
        println!("zone: {} token: {}", self.zone, self.token);
    }

    /// update dynv6 ipv6 address!
    fn update(&mut self) {
        let interval = Duration::from_secs(self.interval);
        loop {
            if let Ok(ipv6_list) = Self::get_ipv6_list() {
                if ipv6_list.len() > 0 {
                    let ipv6 = ipv6_list[0].as_str();
                    self.update_ip(ipv6);
                }
            }
            std::thread::sleep(interval);
        }
    }

    fn update_ip(&mut self, ipv6: &str) {
        if ipv6 != self.record_ipv6 {
            // zone: &str, token: &str, ipv6: &str
            let url = format!(
                "https://dynv6.com/api/update?zone={}&token={}&ipv6={}",
                self.zone, self.token, ipv6
            );
            if let Ok(response) = reqwest::blocking::get(url) {
                // println!("{:#?}", response);
                if response.status() == StatusCode::OK {
                    println!("update ipv6 success: {ipv6}");
                    self.record_ipv6 = ipv6.to_string();
                }
            } else {
                println!("update dynv6 failed!");
            }
        }
    }

    /// 通过ipconfig命令获取系统ipv6地址
    /// min_size: 返回ipv6数量
    #[cfg(target_os = "windows")]
    fn get_ipv6_list() -> anyhow::Result<Vec<String>> {
        use encoding_rs::GBK;
        let output = Command::new("ipconfig.exe").output()?;
        if output.status.success() {
            let decoded = GBK.decode(&output.stdout);
            let output_str = decoded.0.into_owned();
            let lines = output_str.split("\r\n");
            Ok(lines
                .filter(|line| line.trim().starts_with("IPv6"))
                .filter_map(|line| {
                    let line_split = line.split(": ").collect::<Vec<&str>>();
                    let ipv6 = line_split[1].to_string();
                    if line_split.len() == 2 && !ipv6.starts_with("f") {
                        Some(ipv6)
                    } else {
                        None
                    }
                })
                .collect())
        } else {
            Err(anyhow!("Command execution error!"))
        }
    }

    #[cfg(target_os = "macos")]
    fn get_ipv6_list() -> anyhow::Result<Vec<String>> {
        let output = Command::new("ifconfig").output()?;
        if output.status.success() {
            let output_str = String::from_utf8(output.stdout)?;
            let lines = output_str.split("\n");
            Ok(lines
                .filter(|line| line.trim().starts_with("inet6"))
                .filter_map(|line| {
                    let line_split = line.split(" ").collect::<Vec<&str>>();
                    let ipv6 = line_split[1].to_string();
                    if ipv6.starts_with("2") {
                        Some(ipv6)
                    } else {
                        None
                    }
                })
                .collect())
        } else {
            Err(anyhow!("Command execution error!"))
        }
    }

    /// 通过ip addr获取linux ipv6
    #[cfg(target_os = "linux")]
    fn get_ipv6_list() -> anyhow::Result<Vec<String>> {
        let output = Command::new("ip").arg("addr").output()?;
        if output.status.success() {
            let output_str = String::from_utf8(output.stdout)?;
            let lines = output_str.split("\n");
            Ok(lines
                .filter(|line| line.trim().starts_with("inet6"))
                .filter_map(|line| {
                    let line_split = line.trim().split(" ").collect::<Vec<&str>>();
                    let ipv6 = line_split[1].to_string();
                    if ipv6.starts_with("2") {
                        Some(ipv6)
                    } else {
                        None
                    }
                })
                .collect())
        } else {
            Err(anyhow!("Command execution error!"))
        }
    }
}
