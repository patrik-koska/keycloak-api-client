use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct TokenData {
    access_token: String,
    refresh_token: String
}

pub fn get_token(
    base_url: &String, client_id: String, secret: String, 
    user: String, pass: String, grant_type: String, 
    client: &reqwest::blocking::Client
    ) -> String {
    
    let endpoint = format!("{}/auth/realms/master/protocol/openid-connect/token", &base_url);

    let mut params = HashMap::new();
    params.insert("client_id", client_id);
    params.insert("client_secret", secret);
    params.insert("username", user);
    params.insert("password", pass);
    params.insert("grant_type", grant_type);

    let res = client
    .post(endpoint)
    .form(&params)
    .send()
    .expect("Error getting json data")
    .json::<TokenData>()
    .expect("Error serializing json data");

    res.access_token
    
}

#[allow(dead_code)]
pub fn get_refresh_token(
    base_url: &String, client_id: String, secret: String, 
    user: String, pass: String, grant_type: String, 
    client: reqwest::blocking::Client
    ) -> String {
    
    let endpoint = format!("{}/auth/realms/master/protocol/openid-connect/token", &base_url);

    let mut params = HashMap::new();
    params.insert("client_id", client_id);
    params.insert("client_secret", secret);
    params.insert("username", user);
    params.insert("password", pass);
    params.insert("grant_type", grant_type);

    let res = client
    .post(endpoint)
    .form(&params)
    .send()
    .expect("Error getting json data")
    .json::<TokenData>()
    .expect("Error serializing json data");

    res.refresh_token
}