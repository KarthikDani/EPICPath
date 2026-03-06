use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowStep {
    pub number: u32,
    pub title: String,
    pub description: String,
    pub action: String,
    #[serde(default)]
    pub tips: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    pub id: String,
    pub title: String,
    pub module: String,
    pub roles: Vec<String>,
    pub difficulty: String,
    pub estimated_minutes: u32,
    pub tags: Vec<String>,
    #[serde(default)]
    pub prerequisites: Vec<String>,
    pub steps: Vec<WorkflowStep>,
}
