use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub status: u16,
    pub message: String,
    pub data: Option<T>,
}
