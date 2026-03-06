use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptFrontmatter {
    pub id: String,
    pub title: String,
    pub category: String,
    pub tags: Vec<String>,
    pub related: Vec<String>,
    pub roles: Vec<String>,
    pub difficulty: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Concept {
    #[serde(flatten)]
    pub meta: ConceptFrontmatter,
    pub body_markdown: String,
    pub body_html: String,
    pub read_time_minutes: u32,
}
