use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Userdata {
    id: String,
    username: String,
    email: String
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