fn main() {
    for item in dotenv::dotenv_iter().unwrap() {
        let (key, value) = item.unwrap();
        println!("{}: {}", key, value);
    }
}
