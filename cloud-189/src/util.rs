use anyhow::Result;
use reqwest::Request;
use serde_json::Value;
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};
use url::Url;

pub fn timestamp() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
}

/// 对参数进行排序并拼接成字符串
pub fn sort_parameter(data: BTreeMap<String, String>) -> String {
    let entries: Vec<String> = data
        .iter()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect();
    entries.join("&")
}

/// 生成参数的MD5签名
pub fn get_signature(data: BTreeMap<String, String>) -> String {
    let parameter = sort_parameter(data);
    // 计算MD5哈希并转换为十六进制字符串
    let digest = md5::compute(parameter.as_bytes());
    format!("{:x}", digest)
}

pub fn parse_url_params(url: &Url) -> BTreeMap<String, String> {
    url.query_pairs().into_owned().collect()
}

pub fn parse_json_request_body(request: &Request) -> BTreeMap<String, String> {
    request
        .body()
        .and_then(|body| body.as_bytes())
        .and_then(|body_bytes| serde_json::from_slice::<serde_json::Value>(body_bytes).ok())
        .and_then(|json_value| {
            if let Value::Object(map) = json_value {
                let mut result = BTreeMap::new();
                for (key, value) in map {
                    match value {
                        Value::Null => {
                            result.insert(key, String::new());
                        }
                        Value::Bool(b) => {
                            result.insert(key, b.to_string());
                        }
                        Value::Number(n) => {
                            result.insert(key, n.to_string());
                        }
                        Value::String(s) => {
                            result.insert(key, s);
                        }
                        Value::Array(_) => {}
                        Value::Object(_) => {}
                    }
                }
                Some(result)
            } else {
                None
            }
        })
        .unwrap_or(BTreeMap::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn test_sort_and_signature() {
        let params = BTreeMap::new();
        let signature = get_signature(params);
        assert_eq!(signature, "d41d8cd98f00b204e9800998ecf8427e");

        let mut params = BTreeMap::new();
        params.insert("b".to_string(), "2".to_string());
        params.insert("a".to_string(), "1".to_string());
        params.insert("c".to_string(), "3".to_string());

        let sorted = sort_parameter(params.clone());
        assert_eq!(sorted, "a=1&b=2&c=3");

        let signature = get_signature(params);
        assert_eq!(signature, "ce788ff9145c2260534889c454d437b8");
    }

    #[test]
    fn test_parse_url_params() -> Result<()> {
        let url_str = "https://example.com/path?name=Alice&age=30&active=true";
        let params = parse_url_params(&Url::parse(url_str)?);
        let sorted = sort_parameter(params);
        assert_eq!(sorted, "active=true&age=30&name=Alice");
        Ok(())
    }
}
