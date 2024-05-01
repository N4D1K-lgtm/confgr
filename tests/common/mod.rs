#![allow(dead_code)]

pub fn get_var(key: &str) -> String {
    let msg = format!("{} must be set in .env", key);
    std::env::var(key).expect(&msg)
}
