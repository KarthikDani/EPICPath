use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathModule {
    pub id: String,
    pub title: String,
    pub description: String,
    pub concepts: Vec<String>,
    pub workflows: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningPath {
    pub id: String,
    pub title: String,
    pub description: String,
    pub role: String,
    pub difficulty: String,
    pub estimated_hours: u32,
    pub modules: Vec<PathModule>,
}
