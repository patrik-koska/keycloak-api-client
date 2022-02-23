use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Userdata {
    enabled: bool,
    id: String,
    username: String,
    email: String,
    firstName: String,
    lastName: String,
    emailVerified: bool
}

pub fn list_users(base_url: &String, token: &String, client: &reqwest::blocking::Client) {
    let endpoint = format!("{}/auth/admin/realms/myrealm/users", &base_url);

    let res = client
        .get(endpoint)
        .bearer_auth(&token)
        .send()
        .expect("Could not get users")
        .text()
        .expect("Could not get response as text");

    let userdatas: Vec<Userdata> = serde_json::from_str(&res).expect("Could not deserialize to struct");

    for data in userdatas.iter() {
        println!("username: {}, id: {}, email: {}", data.username, data.id, data.email);
    }

}

pub fn create_users(base_url: &String, token: &String, client: &reqwest::blocking::Client, username: String, 
    email: String, firstname: String, lastname: String, emailverified: bool, userid: String) {

    let endpoint = format!("{}/auth/admin/realms/myrealm/users", &base_url);

    let user = Userdata {
        id: userid,
        enabled: true,
        username: username,
        email: email,
        firstName: firstname,
        lastName: lastname,
        emailVerified: emailverified
    };

    let res = client
    .post(endpoint)
    .bearer_auth(&token)
    .json(&user)
    .send()
    .expect("Could not make post request to create users");

    println!("User creation ended with status: {}", res.status())

}

#[allow(dead_code)]
fn get_users_from_file(){}