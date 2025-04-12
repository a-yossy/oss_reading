use dotenv::dotenv;
use dotenv_reading::examples::as_ref::{MyStruct, hello};
use std::env;

fn main() {
    for item in dotenv::dotenv_iter().unwrap() {
        let (key, value) = item.unwrap();
        println!("{}: {}", key, value);
    }
}
