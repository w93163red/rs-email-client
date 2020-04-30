// this file is to get the gmail token

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
    params.insert("client_id", string_encode(client_id));
    params.insert(
        "redirect_uri",
        string_encode("urn:ietf:wg:oauth:2.0:oob:auto"),
    );
    params.insert("client_secret", string_encode(client_secret));
    params.insert("code", string_encode(authorize_code));
    params.insert("grant_type", string_encode("authorization_code"));
    let account_URL = "https://oauth2.googleapis.com/token";
    let res = reqwest::Client::new()
        .post(account_URL)
        .form(&params)
        .send()
        .await?;
    println!("{:#?}", res.json::<HashMap<String, String>>().await?);
    Ok(String::new())
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
