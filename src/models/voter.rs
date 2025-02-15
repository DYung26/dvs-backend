#[derive(Debug, Serialize, Deserialize)]
pub struct Voter {
    pub id: i32,
    pub user_id: i32,
    pub election_id: i32,
}
