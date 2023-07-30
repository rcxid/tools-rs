use clap::Parser;
use encoding_rs::GBK;
use reqwest::StatusCode;
use std::{error::Error, process::Command, time::Duration};

fn main() {
    let args = Args::parse();
    let zone = args.zone.as_str();
    let token = args.token.as_str();
    println!("zone: {} token: {}", zone, token);
    let interval = Duration::from_secs(60);
    let mut record_ipv6 = String::new();
    loop {
        if let Ok(ipv6_list) = get_system_ipv6(1) {
            if ipv6_list.len() == 1 {
                let ipv6 = ipv6_list[0].as_str();
                if ipv6 != record_ipv6 {
                    if let Some(update_ipv6) = update_dynv6(zone, token, ipv6) {
                        record_ipv6 = update_ipv6;
                    }
                }
            }
        }
        std::thread::sleep(interval);
    }
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
}

/// update dynv6 ipv6 address!
fn update_dynv6(zone: &str, token: &str, ipv6: &str) -> Option<String> {
    let mut update_ipv6 = None;
    let url = format!("https://dynv6.com/api/update?zone={zone}&token={token}&ipv6={ipv6}");
    if let Ok(response) = reqwest::blocking::get(url) {
        // println!("{:#?}", response);
        if response.status() == StatusCode::OK {
            println!("update ipv6 success: {ipv6}");
            update_ipv6 = Some(ipv6.to_string());
        }
    } else {
        println!("update dynv6 failed!");
    }
    update_ipv6
}

/// 获取系统ipv6地址
fn get_system_ipv6(min_size: usize) -> Result<Vec<String>, Box<dyn Error>> {
    if cfg!(target_os = "windows") {
        get_windows_ipv6(min_size)
    } else if cfg!(target_os = "linux") {
        todo!()
    } else {
        panic!("unsupport operate system!");
    }
}

/// 通过ipconfig命令获取系统ipv6地址
/// min_size: 返回ipv6数量
fn get_windows_ipv6(min_size: usize) -> Result<Vec<String>, Box<dyn Error>> {
    let mut result = Vec::new();
    let output = Command::new("ipconfig.exe").output()?;
    if output.status.success() {
        let decoded = GBK.decode(&output.stdout);
        let output_str = decoded.0.into_owned();
        let lines = output_str.split("\r\n");
        for line in lines {
            if line.trim().starts_with("IPv6") {
                let b = line.split(": ").collect::<Vec<&str>>();
                if b.len() == 2 {
                    result.push(b[1].to_string());
                }
            }
        }
    }
    Ok(result
        .into_iter()
        .enumerate()
        .filter_map(
            |(index, value)| {
                if index < min_size {
                    Some(value)
                } else {
                    None
                }
            },
        )
        .collect())
}
