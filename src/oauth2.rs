// this file is to get the gmail token
#[allow(non_snake_case)]
#[allow(unused_imports)]
use serde::Deserialize;

use percent_encoding::utf8_percent_encode;
use percent_encoding::AsciiSet;
use percent_encoding::CONTROLS;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use text_io::read;

#[derive(Deserialize, Debug)]
pub struct Auth {
    client_id: String,
    client_secret: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct Resp {
    access_token: String,
    expires_in: i32,
    refresh_token: String,
    scope: String,
    token_type: String,
}

pub fn read_authorize_info() -> Result<Auth, Box<dyn Error>> {
    let file = File::open("./oauth2.txt")?;
    let reader = BufReader::new(file);

    let u = serde_json::from_reader(reader)?;
    println!("{:?}", u);
    Ok(u)
}

pub async fn authorization() -> Result<String, Box<dyn Error>> {
    let user = read_authorize_info()?;
    let url = generate_permission_url(&user.client_id);
    println!("To authorize token, visit the url");
    println!("{:?}", url);
    let authorize_code: String = read!("{}");
    let access_token =
        get_authorize_tokens(&user.client_id, &user.client_secret, &authorize_code).await?;
    Ok(access_token)
}

pub fn generate_permission_url(client_id: &str) -> String {
    let mut params = HashMap::new();
    params.insert("client_id", client_id);
    params.insert("redirect_uri", "urn:ietf:wg:oauth:2.0:oob");
    params.insert("scope", "https://mail.google.com");
    params.insert("response_type", "code");
    let account_URL = "https://accounts.google.com/o/oauth2/auth";
    let params_string = format_url_params(params);
    let request = format!("{}?{}", account_URL, string_encode(&params_string));
    request
}

pub fn format_url_params(params: HashMap<&str, &str>) -> String {
    let mut param_list = Vec::new();
    for (key, value) in &params {
        param_list.push(format!("{}={}", key, value));
    }
    param_list.join("&")
}

pub async fn get_authorize_tokens(
    client_id: &str,
    client_secret: &str,
    authorize_code: &str,
) -> Result<String, Box<dyn Error>> {
    let mut params = HashMap::new();
    params.insert("client_id", client_id);
    params.insert("redirect_uri", "urn:ietf:wg:oauth:2.0:oob");
    params.insert("client_secret", client_secret);
    params.insert("code", authorize_code);
    params.insert("grant_type", "authorization_code");
    let account_url = "https://accounts.google.com/o/oauth2/token";
    let res = reqwest::Client::new()
        .post(account_url)
        .form(&params)
        .send()
        .await?;
    let res_json = res.bytes().await?;
    let resp: Resp = serde_json::from_slice(&res_json).unwrap_or(Resp::default());
    Ok(resp.access_token)
}

pub fn string_encode(s: &str) -> String {
    const FRAGMENT: &AsciiSet = &CONTROLS
        .add(b' ')
        .add(b'"')
        .add(b'<')
        .add(b'>')
        .add(b'`')
        .add(b'~')
        .add(b':')
        .add(b'/');
    let params_string = utf8_percent_encode(&s, FRAGMENT).to_string();
    params_string
}

mod tests {
    use crate::oauth2::Resp;

    #[test]
    fn test_serde_json() {
        let data = b"{\n  \"access_token\": \"123\",\n  \"expires_in\": 3599,\n  \"refresh_token\": \"123\",\n  \"scope\": \"https://mail.google.com/\",\n  \"token_type\": \"Bearer\"\n}";
        let v: Resp = serde_json::from_slice(data).unwrap();
        println!("{:?}", v.access_token);
    }
}
