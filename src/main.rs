mod token;
mod users;

fn main() {
    let base_url = String::from("http://localhost:34555");
    let client_id = String::from("admin-cli");
    let client_secret = String::from("65d02276-8f96-40ba-a32d-a0bd2ee5665b");
    let admin_user = String::from("admin");
    let admin_password = String::from("Passw0rd");
    let grant_type = String::from("password");

    let client = reqwest::blocking::Client::new();

    let token = token::get_token(&base_url, &client_id, &client_secret, &admin_user, &admin_password, &grant_type, &client);

    users::list_users(&base_url, &token, &client);

}