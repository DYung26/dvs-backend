#[derive(Debug, Serialize, Deserialize)]
pub struct Vote {
    pub id: i32,
    pub poll_id: i32,
    pub voter_id: i32,
    pub candidate_id: i32,
    pub timestamp: String,
}
