#[derive(Debug, Deserialize)]
pub struct RequestRegister {
    pub account: String,
    pub password: String,
    pub phone_number: String,
}

#[derive(Debug, Serialize)]
pub struct ResponseRegister {
    pub code: u32,
    pub account: String,
    pub password: String,
}