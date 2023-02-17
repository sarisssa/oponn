#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    pub number: usize,
    pub line_one: String,
    pub line_two: String,
    pub city: String,
}
