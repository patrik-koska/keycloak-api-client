mod token;
mod users;
mod userid;
use uuid::Uuid;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    base_url: String,
    #[clap(short, long)]
    client_id: String,
    #[clap(short, long)]
    secret: String,
    #[clap(short, long)]
    admin_user: String,
    #[clap(short, long)]
    password: String,
    #[clap(short, long)]
    grant_type: String
}

fn main() {
    let uuid = Uuid::new_v4();

    let args = Cli::parse();

    let userid = userid::gen_uuid(&uuid);

    let client = reqwest::blocking::Client::new();

    let token = token::get_token(&args.base_url, args.client_id, args.secret, args.admin_user, args.password, args.grant_type, &client);

    users::list_users(&args.base_url, &token, &client);
    
}
