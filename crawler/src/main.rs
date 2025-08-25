use crate::boss::{BossCrawler, BossCrawlerParams};
use anyhow::Result;
use reqwest::header;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

mod boss;

#[tokio::main]
async fn main() -> Result<()> {
    let params = BossCrawlerParams::new(1, 15, "101210100", "数据仓库");
    let mut headers = HeaderMap::new();
    headers.insert(header::USER_AGENT, HeaderValue::from_str("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/138.0.0.0 Safari/537.36")?);
    headers.insert(header::COOKIE, HeaderValue::from_str("lastCity=101280100; ab_guid=34d91d20-3d8a-49af-9cb8-217a7aaed7be; wt2=DPKqyEYT-gNyHLp0Hi3uGVAaVqVrujTVr6C2n0RAKSATerXaMr8zG2Bsc15D1Uz0VN5GR45gMY_rdhQDPNHaAAA~~; wbg=0; zp_at=zmClJe0enzzcQ_xshtg8ZnCfwjr0DnXtcOD0awTuCUQ~; __g=-; __l=l=%2Fwww.zhipin.com%2Fweb%2Fgeek%2Frecommend&r=&g=&s=3&friend_source=0; Hm_lvt_194df3105ad7148dcf2b98a91b5e727a=1754015562,1754276321; Hm_lpvt_194df3105ad7148dcf2b98a91b5e727a=1754276321; HMACCOUNT=E58E3BE7182714A7; bst=V2R9MkF-f72F5iVtRuyhgbLiu47Drewys~|R9MkF-f72F5iVtRuyhgbLiu47DrWzC4~; __c=1754276295; __a=27917383.1742636838.1754015542.1754276295.40.7.3.36; __zp_stoken__=12aefw4JDewQ2BANfVQxvSgN%2BwrjCtVNUaMKlYcKpdG1QVmhdYMKawqdawrbClMK8wq1Fwqllwp%2FCvMOzwqzCj8KXwr3ClcK%2BRsO%2Fwr3CgljCm8SYw6XDs8K7xJfCgcK3wp0xJQcCAgENc3Z2dXkMDQIBDQYDAwQQEAkJCgY1K8O3wroyNzE%2BOCpJUFAGRlpaR1lKBFRIRTw2UlENBTYtKjg8MsOAwrfCs8OtwrnCt8K0w6HCs8KywrMPODQyN8Kyey8jCgPCsXYGwrJOBFkGwrfClwTDjFrCpxLDjcK9wporMTHCuzsyFT49N0AyMTQ3MiUywrzChMOHX8KoHcOLwrLCgSE8FzoyMTY1MjIxODdAJjE1NSgyOCw7ARAJAwshPcK0wpTCssOcMjE%3D")?);
    headers.insert(HeaderName::from_static(":authority"), HeaderValue::from_str("www.zhipin.com")?);
    headers.insert(HeaderName::from_static(":method"), HeaderValue::from_str("GET")?);
    headers.insert(HeaderName::from_static(":scheme"), HeaderValue::from_str("https")?);
    headers.insert(header::ACCEPT, HeaderValue::from_str("application/json, text/plain, */*")?);
    headers.insert(header::ACCEPT_ENCODING, HeaderValue::from_str("gzip, deflate, br, zstd")?);
    headers.insert(header::ACCEPT_LANGUAGE, HeaderValue::from_str("zh-CN,zh;q=0.9")?);
    // headers.insert(HeaderName::from_static(""), HeaderValue::from_str("")?);
    let crawler = BossCrawler::try_new(headers, params)?;
    crawler.job_list().await?;
    Ok(())
}
