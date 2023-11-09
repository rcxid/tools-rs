use clap::Parser;
use encoding_rs::GBK;
use reqwest::StatusCode;
use std::{process::Command, time::Duration};
use thiserror::Error;

fn main() {
    let args = Args::parse();
    let zone = args.zone.as_str();
    let token = args.token.as_str();
    println!("zone: {} token: {}", zone, token);
    let interval = Duration::from_secs(60);
    let mut record_ipv6 = String::new();
    loop {
        if let Ok(ipv6_list) = get_system_ipv6() {
            if ipv6_list.len() > 0 {
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
fn get_system_ipv6() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    if cfg!(target_os = "windows") {
        get_windows_ipv6()
    } else if cfg!(target_os = "macos") {
        get_macos_ipv6()
    } else if cfg!(target_os = "linux") {
        get_linux_ipv6()
    } else {
        panic!("unsupport operate system!");
    }
}

/// 通过ipconfig命令获取系统ipv6地址
/// min_size: 返回ipv6数量
fn get_windows_ipv6() -> Result<Vec<String>, Box<dyn std::error::Error>> {
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
        Err(Box::new(AppError::ExecutionError))
    }
}

fn get_macos_ipv6() -> Result<Vec<String>, Box<dyn std::error::Error>> {
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
        Err(Box::new(AppError::ExecutionError))
    }
}

/// 通过ip addr获取linux ipv6
fn get_linux_ipv6() -> Result<Vec<String>, Box<dyn std::error::Error>> {
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
        Err(Box::new(AppError::ExecutionError))
    }
}

#[derive(Error, Debug)]
enum AppError {
    #[error("Command execution error!")]
    ExecutionError,
}
