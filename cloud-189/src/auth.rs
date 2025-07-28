use crate::const_val::*;
use crate::util;
use anyhow::{anyhow, Result};
use regex::Regex;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::{header, Client};
use rsa::pkcs8::DecodePublicKey;
use rsa::{Pkcs1v15Encrypt, RsaPublicKey};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct CloudAuthClient {
    client: Client,
}

#[derive(Debug, Serialize, Deserialize)]
struct Encrypt {
    data: EncryptData,
    result: isize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EncryptData {
    pre: String,
    pre_domain: String,
    pub_key: String,
    up_sms_on: String,
}

#[derive(Clone)]
struct AppConf {
    captcha_token: String,
    lt: String,
    param_id: String,
    req_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LoginForm {
    app_key: String,
    account_type: String,
    validate_code: String,
    captcha_token: String,
    dynamic_check: String,
    client_type: String,
    #[serde(rename = "cb_SaveName")]
    cb_save_name: String,
    is_oauth2: bool,
    return_url: String,
    param_id: String,
    user_name: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LoginResponse {
    msg: String,
    result: isize,
    to_url: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenSession {
    #[serde(rename = "res_code")]
    res_code: i64,
    #[serde(rename = "res_message")]
    res_message: String,
    access_token: String,
    family_session_key: String,
    family_session_secret: String,
    refresh_token: String,
    login_name: String,
    session_key: String,
    session_secret: String,
    get_file_diff_span: i64,
    get_user_info_span: i64,
    is_save_name: String,
    keep_alive: i64,
}

impl CloudAuthClient {
    pub fn try_new() -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(header::USER_AGENT, HeaderValue::from_static(USER_AGENT));
        headers.insert(
            header::ACCEPT,
            HeaderValue::from_static("application/json;charset=UTF-8"),
        );
        let client = Client::builder().default_headers(headers).build()?;
        Ok(Self { client })
    }

    async fn get_encrypt(&self) -> Result<Encrypt> {
        Ok(self
            .client
            .post("https://open.e.189.cn/api/logbox/config/encryptConf.do")
            .send()
            .await?
            .json::<Encrypt>()
            .await?)
    }

    async fn get_login_form(&self) -> Result<AppConf> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        let res = self
            .client
            .get("https://cloud.189.cn/api/portal/unifyLoginForPC.action")
            .query(&json!({
                "appId": APP_ID,
                "clientType": CLIENT_TYPE,
                "returnURL": RETURN_URL,
                "timeStamp": timestamp,
            }))
            .send()
            .await?
            .text()
            .await?;

        let res = res.as_str();
        let captcha_re = Regex::new(r"'captchaToken' value='(.+?)'")?;
        let captcha_token = captcha_re
            .captures(res)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str())
            .ok_or(anyhow!("未找到captchaToken"))?;
        let lt_re = Regex::new(r#"lt = "(.+?)""#)?;
        let lt = lt_re
            .captures(res)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str())
            .ok_or(anyhow!("未找到lt"))?;
        let param_id_re = Regex::new(r#"paramId = "(.+?)""#)?;
        let param_id = param_id_re
            .captures(res)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str())
            .ok_or(anyhow!("未找到paramId"))?;
        let req_id_re = Regex::new(r#"reqId = "(.+?)""#)?;
        let req_id = req_id_re
            .captures(res)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str())
            .ok_or(anyhow!("未找到reqId"))?;

        Ok(AppConf {
            captcha_token: captcha_token.to_string(),
            lt: lt.to_string(),
            param_id: param_id.to_string(),
            req_id: req_id.to_string(),
        })
    }

    async fn build_login_form(
        &self,
        encrypt: Encrypt,
        app_conf: AppConf,
        username: &str,
        password: &str,
    ) -> Result<LoginForm> {
        let key_data = format!(
            "-----BEGIN PUBLIC KEY-----\n{}-----END PUBLIC KEY-----",
            str_line_break(encrypt.data.pub_key.trim(), 64)
        );
        let username_encrypt = rsa_encrypt(key_data.as_str(), username)?;
        let password_encrypt = rsa_encrypt(key_data.as_str(), password)?;
        Ok(LoginForm {
            app_key: APP_ID.to_string(),
            account_type: ACCOUNT_TYPE.to_string(),
            validate_code: "".to_string(),
            captcha_token: app_conf.captcha_token,
            dynamic_check: "FALSE".to_string(),
            client_type: "1".to_string(),
            cb_save_name: "3".to_string(),
            is_oauth2: false,
            return_url: RETURN_URL.to_string(),
            param_id: app_conf.param_id,
            user_name: format!("{}{}", encrypt.data.pre, username_encrypt),
            password: format!("{}{}", encrypt.data.pre, password_encrypt),
        })
    }

    /// 通过账号密码登录
    pub async fn login_by_password(&self, username: &str, password: &str) -> Result<TokenSession> {
        let encrypt = self.get_encrypt().await?;
        let app_conf = self.get_login_form().await?;
        let data = self
            .build_login_form(encrypt, app_conf.clone(), username, password)
            .await?;
        let login_response = self
            .client
            .post("https://open.e.189.cn/api/logbox/oauth2/loginSubmit.do")
            .header(header::REFERER, HeaderValue::from_static(AUTH_URL))
            .header(
                HeaderName::from_static("lt"),
                HeaderValue::from_str(app_conf.lt.as_str())?,
            )
            .header(
                HeaderName::from_static("reqid"),
                HeaderValue::from_str(app_conf.req_id.as_str())?,
            )
            .form(&data)
            .send()
            .await?
            .json::<LoginResponse>()
            .await?;
        let res = self
            .get_session_for_pc(Some(login_response.to_url), None)
            .await?;
        Ok(res)
    }

    async fn get_session_for_pc(
        &self,
        url: Option<String>,
        access_token: Option<String>,
    ) -> Result<TokenSession> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        let mut params = json!({
            "appId": APP_ID,
            "clientType": "TELEPC",
            "version": "6.2",
            "channelId": "web_cloud.189.cn",
            "rand": timestamp,
        });
        if let Some(url) = url {
            params["redirectURL"] = serde_json::value::Value::String(url);
        }
        if let Some(token) = access_token {
            params["accessToken"] = serde_json::value::Value::String(token);
        }
        let res = self
            .client
            .post("https://api.cloud.189.cn/getSessionForPC.action")
            .query(&params)
            .send()
            .await?
            .json::<TokenSession>()
            .await?;
        Ok(res)
    }

    /// 通过token登录
    pub async fn _login_by_access_token(&self, access_token: &str) -> Result<TokenSession> {
        self.get_session_for_pc(None, Some(access_token.to_string()))
            .await
    }

    //**
    //    * sso登录
    //    */
    //   async loginBySsoCooike(cookie: string) {
    //     logger.debug('loginBySsoCooike...')
    //     const res = await this.request.get(`$`, {
    //       searchParams: {
    //         : AppID,
    //         : ClientType,
    //         : ,
    //         : Date.now()
    //       }
    //     })
    //     const redirect = await this.request(res.url, {
    //       headers: {
    //         Cookie: `SSON=${cookie}`
    //       }
    //     })
    //     return await this.getSessionForPC({ redirectURL: redirect.url })
    //   }

    async fn login_by_sso_cookie(&self, cookie: &str) -> Result<()> {
        let timestamp = util::timestamp();
        self.client
            .get(format!("{WEB_URL}/api/portal/unifyLoginForPC.action"))
            .query(&json!({
                "appId": APP_ID,
                "clientType": CLIENT_TYPE,
                "returnURL": RETURN_URL,
                "timeStamp": timestamp,
            }))
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        Ok(())
    }

    /// todo
    /// 刷新token
    async fn refresh_token(&self, refresh_token: &str) -> Result<()> {
        let res = self
            .client
            .post(format!("{AUTH_URL}/api/oauth2/refreshToken.do"))
            .form(&json!({
                "clientId": "",
                "refreshToken": refresh_token,
                "grantType": "refresh_token",
                "format": "json",
            }))
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;
        Ok(())
    }
}

fn str_line_break(s: &str, break_size: usize) -> String {
    s.chars()
        .collect::<Vec<_>>()
        .chunks(break_size)
        .map(|chunk| format!("{}\n", chunk.into_iter().collect::<String>()))
        .collect()
}

fn rsa_encrypt(public_key: &str, orig_data: &str) -> Result<String> {
    let rsa_public_key = RsaPublicKey::from_public_key_pem(public_key)?;
    let encrypted = rsa_public_key.encrypt(
        &mut rand::thread_rng(),
        Pkcs1v15Encrypt,
        orig_data.as_bytes(),
    )?;
    Ok(hex::encode_upper(&encrypted))
}
