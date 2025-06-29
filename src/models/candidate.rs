#[derive(Debug, Serialize, Deserialize)]
pub struct Candidate {
    pub id: i32,
    pub user_id: i32,
    pub election_id: i32,
    pub manifesto: String,
    pub approved: bool,
}
