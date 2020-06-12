use std::io::prelude::*;
use std::fs::File;
use std::ops::Index;
use serde_qs;
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestRegister {
    pub account: String,
    pub password: String,
    pub phone_number: String,
}

fn main() {
    let mut buffer = File::create("siege.txt").unwrap();
    for i in 0 .. 10000 {
        let account = "account".to_owned() + &i.to_string();
        let req = RequestRegister {
            account: account,
            password: String::from("fdfdfsd"),
            phone_number: String::from("213123213")
        };
    
        let req_str = serde_qs::to_string(&req).unwrap();
        buffer.write(b"https://127.0.0.1:8080/user/register POST ").unwrap();
        buffer.write(req_str.index(0..req_str.len()).as_bytes()).unwrap();
        buffer.write(b"\r\n").unwrap();
    }
}