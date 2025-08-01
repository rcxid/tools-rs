use anyhow::anyhow;
use async_trait::async_trait;
use std::process::Command;
use std::time::Duration;
use tokio::time;

pub mod cloudflare;
pub mod dynv6;

/// ip地址类型
pub enum IpAddressType {
    Ipv4,
    Ipv6,
}

#[async_trait]
pub trait DdnsClient {
    /// 支持的ddns类型
    fn support_type(&self) -> Vec<IpAddressType>;
    /// 更新间隔：单位秒
    fn interval_secs(&self) -> u64;

    async fn update(&mut self) -> anyhow::Result<()>;

    async fn run(&mut self) {
        let period = Duration::from_secs(self.interval_secs());
        let mut interval = time::interval(period);
        loop {
            interval.tick().await;
            if let Err(err) = self.update().await {
                println!("{}", err);
            }
        }
    }
}

/// 通过ipconfig命令获取系统ipv6地址
/// min_size: 返回ipv6数量
#[cfg(target_os = "windows")]
pub fn get_ipv6_list() -> anyhow::Result<Vec<String>> {
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
pub fn get_ipv6_list() -> anyhow::Result<Vec<String>> {
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
pub fn get_ipv6_list() -> anyhow::Result<Vec<String>> {
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
