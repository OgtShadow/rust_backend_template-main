pub enum ResponseType {
    Success,
    Data,
    Error,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum Response<T> {
    Success(String),
    Data(T),
    Error { code: u16, message: String },
}

#[derive(Debug, serde::Deserialize)]
pub struct MessageIn {
    pub message: String,
    pub sender: String,
}

#[derive(serde::Serialize)]
pub struct MessageOut {
    pub text: String,
    pub date: chrono::DateTime<chrono::Utc>,
}

#[derive(serde::Serialize)]
pub struct WebResponse<T> {
    pub status: String,
    pub result: Response<T>,
}

impl<T> WebResponse<T> {
    pub fn new(
        response_type: ResponseType,
        message: String,
        code: u16,
        data: Option<T>,
    ) -> WebResponse<T> {
        match response_type {
            ResponseType::Success => Self {
                status: "ok".to_owned(),
                result: Response::Success(message),
            },
            ResponseType::Data => match data {
                Some(data) => Self {
                    status: message,
                    result: Response::Data(data),
                },
                None => Self {
                    status: "error".to_owned(),
                    result: Response::Error { code, message },
                },
            },
            ResponseType::Error => Self {
                status: "error".to_owned(),
                result: Response::Error { code, message },
            },
        }
    }
}
