pub mod models;
pub mod utils;
pub mod uutp;

use uutp::Uutp;

fn main() {
    let response = Uutp::get("https://httpbin.org/ip").unwrap();
    println!("Response: {}", response);
}
