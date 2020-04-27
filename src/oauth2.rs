// this file is to get the gmail token

use serde::Deserialize;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;
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

pub fn authorization() -> Result<String, Box<dyn Error>> {
    let user = read_authorize_info()?;
    let url = generate_permission_url(&user.client_id);
    println!("To authorize token, visit the url");
    println!("{:?}", url);
    let access_token: String = read!("access_token: {}");
    println!("access_token: {}", access_token);
    Ok(access_token)
} 


pub fn generate_permission_url(client_id: &str) -> String {
    let mut params = HashMap::new();
    params.insert("client_id", client_id);
    params.insert("redirect_uri", "urn:ietf:wg:oauth:2.0:oob");
    params.insert("scope", "https://mail.google.com");
    params.insert("response_type", "code");
    let account_URL = "https://accounts.google.com/o/oauth2/auth";
    let request = format!("{}?{}", account_URL, format_url_params(params));
    request
}

pub fn format_url_params(params: HashMap<&str, &str>) -> String {
    let mut param_list = Vec::new();
    for (key, value) in &params {
        param_list.push(format!("{}={}", key, value)); 
    }
    param_list.join("&")
}
