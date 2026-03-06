use axum::http::{header, HeaderMap};
use std::collections::HashSet;

/// XP rewards for different actions
const XP_READ_CONCEPT: u32 = 15;
const XP_COMPLETE_WORKFLOW: u32 = 25;
const XP_QUIZ_PER_CORRECT: u32 = 10;
const XP_QUIZ_PERFECT: u32 = 30; // bonus for 100%

/// Rank thresholds
const RANKS: &[(u32, &str, &str)] = &[
    (0, "Intern", "Just getting started"),
    (50, "Observer", "Learning the basics"),
    (120, "Resident", "Building core skills"),
    (250, "Fellow", "Developing expertise"),
    (400, "Attending", "Confident practitioner"),
    (600, "Specialist", "Deep EPIC knowledge"),
    (800, "EPIC Expert", "Mastery achieved"),
];

/// Badges/achievements
pub const BADGES: &[(&str, &str, &str)] = &[
    ("first_concept", "First Steps", "Read your first concept"),
    ("five_concepts", "Curious Mind", "Read 5 concepts"),
    ("all_concepts", "Scholar", "Read all concepts"),
    ("first_workflow", "Hands On", "Complete your first workflow"),
    ("all_workflows", "Process Master", "Complete all workflows"),
    ("first_quiz", "Test Taker", "Complete your first quiz"),
    ("perfect_quiz", "Perfect Score", "Get 100% on a quiz"),
    ("century", "Century Club", "Reach 100 XP"),
    ("expert", "EPIC Expert", "Reach 800 XP"),
];

#[derive(Clone, Debug)]
pub struct UserProgress {
    pub completed_concepts: HashSet<String>,
    pub completed_workflows: HashSet<String>,
    pub quiz_scores: Vec<(String, u32, u32)>, // (quiz_id, score, total)
    pub xp: u32,
}

impl UserProgress {
    pub fn from_headers(headers: &HeaderMap) -> Self {
        let cookies = headers
            .get(header::COOKIE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        let mut progress = Self {
            completed_concepts: HashSet::new(),
            completed_workflows: HashSet::new(),
            quiz_scores: Vec::new(),
            xp: 0,
        };

        for cookie in cookies.split(';') {
            let cookie = cookie.trim();
            if let Some(val) = cookie.strip_prefix("ep_concepts=") {
                if !val.is_empty() {
                    progress.completed_concepts = val.split(',').map(|s| s.to_string()).collect();
                }
            } else if let Some(val) = cookie.strip_prefix("ep_workflows=") {
                if !val.is_empty() {
                    progress.completed_workflows = val.split(',').map(|s| s.to_string()).collect();
                }
            } else if let Some(val) = cookie.strip_prefix("ep_quizzes=") {
                // Format: quizid:score:total,quizid:score:total
                if !val.is_empty() {
                    for entry in val.split(',') {
                        let parts: Vec<&str> = entry.split(':').collect();
                        if parts.len() == 3 {
                            if let (Ok(s), Ok(t)) = (parts[1].parse(), parts[2].parse()) {
                                progress.quiz_scores.push((parts[0].to_string(), s, t));
                            }
                        }
                    }
                }
            } else if let Some(val) = cookie.strip_prefix("ep_xp=") {
                progress.xp = val.parse().unwrap_or(0);
            }
        }

        progress
    }

    pub fn rank(&self) -> (&'static str, &'static str) {
        let mut rank = (RANKS[0].1, RANKS[0].2);
        for &(threshold, name, desc) in RANKS {
            if self.xp >= threshold {
                rank = (name, desc);
            }
        }
        rank
    }

    pub fn next_rank(&self) -> Option<(&'static str, u32)> {
        for &(threshold, name, _) in RANKS {
            if threshold > self.xp {
                return Some((name, threshold));
            }
        }
        None
    }

    pub fn rank_progress_percent(&self) -> u32 {
        let mut current_threshold = 0u32;
        let mut next_threshold = None;

        for &(threshold, _, _) in RANKS {
            if threshold <= self.xp {
                current_threshold = threshold;
            } else {
                next_threshold = Some(threshold);
                break;
            }
        }

        match next_threshold {
            Some(next) => {
                let range = next - current_threshold;
                let progress = self.xp - current_threshold;
                if range > 0 {
                    (progress * 100) / range
                } else {
                    100
                }
            }
            None => 100, // max rank
        }
    }

    pub fn earned_badges(&self, total_concepts: usize, total_workflows: usize) -> Vec<(&'static str, &'static str, &'static str)> {
        let mut earned = Vec::new();

        for &(id, name, desc) in BADGES {
            let has = match id {
                "first_concept" => !self.completed_concepts.is_empty(),
                "five_concepts" => self.completed_concepts.len() >= 5,
                "all_concepts" => total_concepts > 0 && self.completed_concepts.len() >= total_concepts,
                "first_workflow" => !self.completed_workflows.is_empty(),
                "all_workflows" => total_workflows > 0 && self.completed_workflows.len() >= total_workflows,
                "first_quiz" => !self.quiz_scores.is_empty(),
                "perfect_quiz" => self.quiz_scores.iter().any(|(_, s, t)| s == t && *t > 0),
                "century" => self.xp >= 100,
                "expert" => self.xp >= 800,
                _ => false,
            };
            if has {
                earned.push((id, name, desc));
            }
        }
        earned
    }

    pub fn to_set_cookies(&self) -> Vec<String> {
        let max_age = "Max-Age=31536000; Path=/; SameSite=Lax";

        let concepts: Vec<&str> = self.completed_concepts.iter().map(|s| s.as_str()).collect();
        let workflows: Vec<&str> = self.completed_workflows.iter().map(|s| s.as_str()).collect();
        let quizzes: Vec<String> = self.quiz_scores.iter().map(|(id, s, t)| format!("{}:{}:{}", id, s, t)).collect();

        vec![
            format!("ep_concepts={}; {}", concepts.join(","), max_age),
            format!("ep_workflows={}; {}", workflows.join(","), max_age),
            format!("ep_quizzes={}; {}", quizzes.join(","), max_age),
            format!("ep_xp={}; {}", self.xp, max_age),
        ]
    }

    pub fn complete_concept(&mut self, id: &str) {
        if self.completed_concepts.insert(id.to_string()) {
            self.xp += XP_READ_CONCEPT;
        }
    }

    pub fn complete_workflow(&mut self, id: &str) {
        if self.completed_workflows.insert(id.to_string()) {
            self.xp += XP_COMPLETE_WORKFLOW;
        }
    }

    pub fn record_quiz(&mut self, quiz_id: &str, score: u32, total: u32) {
        // Remove old score for this quiz if exists
        self.quiz_scores.retain(|(id, _, _)| id != quiz_id);
        self.quiz_scores.push((quiz_id.to_string(), score, total));
        self.xp += score * XP_QUIZ_PER_CORRECT;
        if score == total && total > 0 {
            self.xp += XP_QUIZ_PERFECT;
        }
    }
}

pub fn estimate_read_time(markdown: &str) -> u32 {
    let word_count = markdown.split_whitespace().count();
    let minutes = (word_count as f64 / 200.0).ceil() as u32;
    minutes.max(1)
}
