#[derive(Debug, Serialize)]
pub struct ResponseError {
    pub code: u32,
}

#[derive(Debug, Deserialize)]
pub struct RequestRegister {
    pub account: String,
    pub password: String,
    pub phone_number: String,
}

#[derive(Debug, Serialize)]
pub struct ResponseRegister {
    pub code: u32,
    pub uuid: String,
    pub account: String,
    pub password: String,
    pub nickname: String,
    pub gender: i8,
    pub phone_number: String,
    pub head_image: String,
}