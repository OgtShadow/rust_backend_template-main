#[derive(serde::Deserialize)]
pub struct Error {
    pub code: u16,
    pub message: String,
}
