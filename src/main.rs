mod db;
mod error_handler;
use dotenv::dotenv;
mod companies;

fn main() {
    dotenv().ok();
    println!("Hello, world!");
    db::init();
}
