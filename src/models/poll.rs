#[derive(Debug, Serialize, Deserialize)]
pub struct Poll {
    pub id: i32,
    pub created_by: i32,
    pub title: String,
    pub description: Option<String>,
    pub start_date: String,
    pub end_date: String,
    pub status: PollStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PollStatus {
    Draft,
    Open,
    Closed,
}
