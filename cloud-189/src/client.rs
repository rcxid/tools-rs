use crate::auth::CloudAuthClient;
use crate::const_val::*;
use crate::util;
use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::{header, Client, Method, Request, RequestBuilder};

pub struct CloudClient {
    username: String,
    password: String,
    client: Client,
    auth_client: CloudAuthClient,
}

impl CloudClient {
    pub fn try_new(username: &str, password: &str) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(header::USER_AGENT, HeaderValue::from_static(USER_AGENT));
        headers.insert(
            header::REFERER,
            HeaderValue::from_str(format!("{WEB_URL}/web/main/").as_str())?,
        );
        headers.insert(
            header::ACCEPT,
            HeaderValue::from_static("application/json;charset=UTF-8"),
        );
        let client = Client::builder().default_headers(headers).build()?;
        Ok(Self {
            username: username.to_string(),
            password: password.to_string(),
            client,
            auth_client: CloudAuthClient::try_new()?,
        })
    }

    /// 获取用户网盘存储容量信息
    pub async fn get_user_size_info(&self) -> Result<()> {
        let request = self
            .client
            .get(format!("{WEB_URL}/api/portal/getUserSizeInfo.action"))
            .build()?;

        let res = self
            .client
            .execute(self.before_request(request)?)
            .await?
            .json::<serde_json::Value>()
            .await?;

        println!("{}", res.to_string());
        Ok(())
    }

    fn before_request(&self, request: Request) -> Result<Request> {
        let url = request.url().as_str();
        if url.starts_with(API_URL) {
            self.api_url_header(request, "")
        } else if url.starts_with(WEB_URL) {
            self.web_url_handle(request, "")
        } else {
            Ok(request)
        }
    }

    fn api_url_header(&self, request: Request, access_token: &str) -> Result<Request> {
        let timestamp = util::timestamp().to_string();
        let mut params = if Method::GET == request.method().clone() {
            util::parse_url_params(request.url())
        } else {
            util::parse_json_request_body(&request)
        };
        params.insert("Timestamp".to_string(), timestamp.clone());
        params.insert("AccessToken".to_string(), access_token.to_string());
        let signature = util::get_signature(params);

        let mut header = HeaderMap::new();
        header.insert(
            HeaderName::from_static("sign-type"),
            HeaderValue::from_static("1"),
        );
        header.insert(
            HeaderName::from_static("signature"),
            HeaderValue::from_str(signature.as_str())?,
        );
        header.insert(
            HeaderName::from_static("timestamp"),
            HeaderValue::from_str(timestamp.as_str())?,
        );
        header.insert(
            HeaderName::from_static("accesstoken"),
            HeaderValue::from_str(access_token)?,
        );
        Ok(RequestBuilder::from_parts(self.client.clone(), request)
            .headers(header)
            .build()?)
    }

    fn web_url_handle(&self, request: Request, session_key: &str) -> Result<Request> {
        let header = if request.url().as_str().contains("/open") {
            let mut params = if Method::GET == request.method() {
                util::parse_url_params(request.url())
            } else {
                util::parse_json_request_body(&request)
            };
            let timestamp = util::timestamp().to_string();
            params.insert("Timestamp".to_string(), timestamp.clone());
            params.insert("AppKey".to_string(), "600100422".to_string());
            let signature = util::get_signature(params);
            let mut header = HeaderMap::new();
            header.insert(
                HeaderName::from_static("sign-type"),
                HeaderValue::from_static("1"),
            );
            header.insert(
                HeaderName::from_static("signature"),
                HeaderValue::from_str(signature.as_str())?,
            );
            header.insert(
                HeaderName::from_static("timestamp"),
                HeaderValue::from_str(timestamp.as_str())?,
            );
            header.insert(
                HeaderName::from_static("appkey"),
                HeaderValue::from_static("600100422"),
            );
            header
        } else {
            HeaderMap::new()
        };
        // url_params.insert("sessionKey".to_string(), session_key.to_string());
        // RequestBuilder::from_parts(self.client.clone(), request)
        //     .headers(header)
        //     .build()?
        Ok(self
            .client
            .request(request.method().clone(), "")
            .headers(header)
            .build()?)
    }
}
