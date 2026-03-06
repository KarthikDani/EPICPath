use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizQuestion {
    pub id: String,
    pub text: String,
    #[serde(rename = "type")]
    pub question_type: String,
    pub options: Vec<String>,
    pub correct: usize,
    pub explanation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quiz {
    pub id: String,
    pub title: String,
    pub description: String,
    pub concepts: Vec<String>,
    pub difficulty: String,
    pub questions: Vec<QuizQuestion>,
}
