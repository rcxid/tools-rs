use encoding_rs::GBK;
use std::{error::Error, process::Command};

fn main() {
    if cfg!(target_os = "windows") {
        if let Ok(ipv6_list) = get_windows_ipv6(1) {
            for ipv6 in ipv6_list {
                println!("{}", ipv6);
            }
        }
    } else if cfg!(target_os = "linux") {
        todo!()
    } else {
        println!("unsupport operate system!")
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
