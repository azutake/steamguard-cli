use std::env::{self};
use simple_steam_totp::{generate};

fn main() {
    let args: Vec<String> = env::args().collect();
    let totp = match generate(&args[1]) {
        Ok(code) => code,
        Err(e) => {
            panic!("Failed {}", e);
        }
    };

    println!("{}", totp);
}
