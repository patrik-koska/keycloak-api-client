mod token;
mod users;
mod userid;
use uuid::Uuid;

fn main() {
    let uuid = Uuid::new_v4();
    let base_url = String::from("http://localhost:34555");
    let client_id = String::from("admin-cli");
    let client_secret = String::from("65d02276-8f96-40ba-a32d-a0bd2ee5665b");
    let admin_user = String::from("admin");
    let admin_password = String::from("Passw0rd");
    let grant_type = String::from("password");

    let username = String::from("patrik");
    let email = String::from("test100@test.com");
    let firstname = String::from("firstname");
    let lastname = String::from("lastname");
    let emailverified = false;
    let userid = userid::gen_uuid(&uuid);


    let client = reqwest::blocking::Client::new();

    let token = token::get_token(&base_url, &client_id, &client_secret, &admin_user, &admin_password, &grant_type, &client);

    users::list_users(&base_url, &token, &client);

    users::create_users(&base_url, &token, &client, username, email, firstname, lastname, emailverified, userid);

    users::list_users(&base_url, &token, &client)
    

}
