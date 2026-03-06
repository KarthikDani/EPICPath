use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;

use pulldown_cmark::{Options, Parser, html};

use crate::models::{Concept, LearningPath, Quiz, Workflow};
use crate::models::concept::ConceptFrontmatter;

pub struct ContentStore {
    pub concepts: HashMap<String, Concept>,
    pub workflows: HashMap<String, Workflow>,
    pub paths: HashMap<String, LearningPath>,
    pub quizzes: HashMap<String, Quiz>,
}

impl ContentStore {
    pub fn load(content_dir: &str) -> Arc<Self> {
        let concepts = Self::load_concepts(&format!("{}/concepts", content_dir));
        let workflows = Self::load_yaml_dir::<Workflow>(&format!("{}/workflows", content_dir));
        let paths = Self::load_yaml_dir::<LearningPath>(&format!("{}/paths", content_dir));
        let quizzes = Self::load_yaml_dir::<Quiz>(&format!("{}/quizzes", content_dir));

        println!(
            "Loaded {} concepts, {} workflows, {} paths, {} quizzes",
            concepts.len(),
            workflows.len(),
            paths.len(),
            quizzes.len()
        );

        Arc::new(Self {
            concepts,
            workflows,
            paths,
            quizzes,
        })
    }

    fn load_concepts(dir: &str) -> HashMap<String, Concept> {
        let mut map = HashMap::new();
        let dir_path = Path::new(dir);

        if !dir_path.exists() {
            return map;
        }

        for entry in fs::read_dir(dir_path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("md") {
                continue;
            }

            let content = fs::read_to_string(&path).unwrap();
            if let Some(concept) = Self::parse_concept(&content) {
                map.insert(concept.meta.id.clone(), concept);
            }
        }

        map
    }

    fn parse_concept(content: &str) -> Option<Concept> {
        // Split frontmatter from body
        let parts: Vec<&str> = content.splitn(3, "---").collect();
        if parts.len() < 3 {
            return None;
        }

        let frontmatter_str = parts[1].trim();
        let body = parts[2].trim();

        let meta: ConceptFrontmatter = serde_yaml::from_str(frontmatter_str).ok()?;

        // Convert markdown body to HTML
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_STRIKETHROUGH);
        let parser = Parser::new_ext(body, options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);

        let read_time_minutes = crate::progress::estimate_read_time(body);

        Some(Concept {
            meta,
            body_markdown: body.to_string(),
            body_html: html_output,
            read_time_minutes,
        })
    }

    fn load_yaml_dir<T>(dir: &str) -> HashMap<String, T>
    where
        T: serde::de::DeserializeOwned + HasId,
    {
        let mut map = HashMap::new();
        let dir_path = Path::new(dir);

        if !dir_path.exists() {
            return map;
        }

        for entry in fs::read_dir(dir_path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("yaml") {
                continue;
            }

            let content = fs::read_to_string(&path).unwrap();
            if let Ok(item) = serde_yaml::from_str::<T>(&content) {
                map.insert(item.id().to_string(), item);
            }
        }

        map
    }
}

pub trait HasId {
    fn id(&self) -> &str;
}

impl HasId for Workflow {
    fn id(&self) -> &str {
        &self.id
    }
}

impl HasId for LearningPath {
    fn id(&self) -> &str {
        &self.id
    }
}

impl HasId for Quiz {
    fn id(&self) -> &str {
        &self.id
    }
}
