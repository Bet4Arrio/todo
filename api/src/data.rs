#[derive(PartialEq, Debug, Clone)]
pub struct Task {
    pub id: usize,
    pub name: String,
    pub description: Option<String>,
    pub completed: bool,
    // pub created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(id: usize, name: String, description: Option<String>, completed: bool) -> Self {
        Self {
            id,
            name,
            description,
            completed,
        }
    }
}
