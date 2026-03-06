use std::sync::Arc;

use axum::extract::{Form, Path, Query, State};
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::Response;
use maud::{html, Markup, PreEscaped};
use serde::Deserialize;

use crate::api::content::ContentStore;
use crate::progress::{UserProgress, BADGES};
use crate::views::layout::{difficulty_tag, page_with_theme, role_tags};

#[derive(Deserialize)]
pub struct FilterParams {
    pub q: Option<String>,
    pub role: Option<String>,
}

fn is_dark(headers: &HeaderMap) -> bool {
    headers
        .get(header::COOKIE)
        .and_then(|v| v.to_str().ok())
        .map(|cookies| cookies.split(';').any(|c| c.trim() == "theme=dark"))
        .unwrap_or(false)
}

fn page(title: &str, active: &str, headers: &HeaderMap, content: Markup) -> Markup {
    let progress = UserProgress::from_headers(headers);
    page_with_theme(title, active, is_dark(headers), &progress, content)
}

fn completed_check(done: bool) -> Markup {
    html! {
        @if done {
            span class="completed-badge" title="Completed" { "\u{2713}" }
        }
    }
}

fn read_time_tag(minutes: u32) -> Markup {
    html! {
        span class="tag tag-default" { (format!("{} min read", minutes)) }
    }
}

fn xp_tag(xp: u32) -> Markup {
    html! {
        span class="tag tag-xp" { (format!("+{} XP", xp)) }
    }
}

// --- Theme Toggle ---

pub async fn toggle_theme(headers: HeaderMap) -> Response {
    let currently_dark = is_dark(&headers);
    let new_value = if currently_dark { "light" } else { "dark" };

    let redirect_to = headers
        .get(header::REFERER)
        .and_then(|v| v.to_str().ok())
        .and_then(|referer| {
            referer
                .find("://")
                .and_then(|i| referer[i + 3..].find('/'))
                .map(|i| {
                    let path_start = referer.find("://").unwrap() + 3 + i;
                    referer[path_start..].to_string()
                })
        })
        .unwrap_or_else(|| "/".to_string());

    Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header(header::LOCATION, redirect_to)
        .header(
            header::SET_COOKIE,
            format!(
                "theme={}; Path=/; Max-Age=31536000; SameSite=Lax",
                new_value
            ),
        )
        .body(axum::body::Body::empty())
        .unwrap()
}

// --- Home ---

pub async fn home(headers: HeaderMap, State(store): State<Arc<ContentStore>>) -> Markup {
    let progress = UserProgress::from_headers(&headers);
    let concept_count = store.concepts.len();
    let workflow_count = store.workflows.len();
    let path_count = store.paths.len();
    let quiz_count = store.quizzes.len();
    let (rank_name, rank_desc) = progress.rank();

    let paths: Vec<_> = store.paths.values().collect();

    page_with_theme("Home", "Home", is_dark(&headers), &progress, html! {
        form action="/search" method="get" class="search-bar" {
            input type="text" name="q" placeholder="Search concepts, workflows, quizzes..." autocomplete="off";
        }

        div class="hero" {
            div class="hero-badge" { "Open Source" }
            h2 { "Master EPIC EHR," br; "one concept at a time." }
            p {
                "A community-driven learning platform for healthcare professionals navigating EPIC. Browse concepts, follow step-by-step workflows, and test your knowledge."
            }
        }

        // Progress summary card
        a href="/progress" class="progress-card" {
            div class="progress-card-left" {
                div class="progress-rank-icon" { (rank_name.chars().next().unwrap_or('I')) }
                div {
                    div class="progress-rank-name" { (rank_name) }
                    div class="progress-rank-desc" { (rank_desc) }
                }
            }
            div class="progress-card-right" {
                div class="progress-stat" {
                    span class="progress-stat-num" { (progress.xp) }
                    span class="progress-stat-label" { "XP" }
                }
                div class="progress-stat" {
                    span class="progress-stat-num" { (progress.completed_concepts.len()) }
                    span class="progress-stat-label" { (format!("/{}", concept_count)) }
                }
                div class="progress-stat" {
                    span class="progress-stat-num" { (progress.completed_workflows.len()) }
                    span class="progress-stat-label" { (format!("/{}", workflow_count)) }
                }
            }
        }

        div class="section-title" { "Overview" }
        div class="stats-grid" {
            a href="/concepts" class="stat-card" {
                div class="stat-number" { (concept_count) }
                div class="stat-label" { "Concepts" }
                div class="stat-desc" { (format!("{} completed", progress.completed_concepts.len())) }
            }
            a href="/workflows" class="stat-card" {
                div class="stat-number" { (workflow_count) }
                div class="stat-label" { "Workflows" }
                div class="stat-desc" { (format!("{} completed", progress.completed_workflows.len())) }
            }
            a href="/paths" class="stat-card" {
                div class="stat-number" { (path_count) }
                div class="stat-label" { "Learning Paths" }
                div class="stat-desc" { "Structured curricula" }
            }
            a href="/quizzes" class="stat-card" {
                div class="stat-number" { (quiz_count) }
                div class="stat-label" { "Quizzes" }
                div class="stat-desc" { (format!("{} taken", progress.quiz_scores.len())) }
            }
        }

        div class="section-title" { "Start Learning" }
        div class="card-grid" {
            @for p in &paths {
                a href=(format!("/paths/{}", p.id)) class="card" {
                    h3 { (p.title) }
                    p { (p.description) }
                    div class="card-tags" {
                        (difficulty_tag(&p.difficulty))
                        span class="tag tag-role" { (p.role) }
                        span class="tag tag-default" { (format!("{}h estimated", p.estimated_hours)) }
                    }
                }
            }
        }
    })
}

// --- Concepts ---

pub async fn concepts_list(
    headers: HeaderMap,
    State(store): State<Arc<ContentStore>>,
    Query(params): Query<FilterParams>,
) -> Markup {
    let progress = UserProgress::from_headers(&headers);
    let mut concepts: Vec<_> = store
        .concepts
        .values()
        .filter(|c| {
            if let Some(ref role) = params.role {
                if !c.meta.roles.contains(role) && !c.meta.roles.contains(&"all".to_string()) {
                    return false;
                }
            }
            if let Some(ref q) = params.q {
                let q_lower = q.to_lowercase();
                return c.meta.title.to_lowercase().contains(&q_lower)
                    || c
                        .meta
                        .tags
                        .iter()
                        .any(|t| t.to_lowercase().contains(&q_lower));
            }
            true
        })
        .collect();

    concepts.sort_by(|a, b| a.meta.title.cmp(&b.meta.title));
    let role_label = params.role.as_deref().unwrap_or("All Roles");
    let completed = progress.completed_concepts.len();

    page("Concepts", "Concepts", &headers, html! {
        form action="/concepts" method="get" class="search-bar" {
            @if let Some(ref role) = params.role {
                input type="hidden" name="role" value=(role);
            }
            input type="text" name="q" placeholder="Search concepts..."
                  value=(params.q.as_deref().unwrap_or("")) autocomplete="off";
        }

        h2 class="page-title" { "Concepts" }
        p class="filter-info" {
            "Showing " strong { (concepts.len()) } " concepts for " strong { (role_label) }
            " \u{2022} " strong { (completed) } " completed"
        }

        div class="card-grid" {
            @for c in &concepts {
                @let done = progress.completed_concepts.contains(&c.meta.id);
                a href=(format!("/concepts/{}", c.meta.id)) class=(if done { "card card-completed" } else { "card" }) {
                    h3 {
                        (completed_check(done))
                        (c.meta.title)
                    }
                    div class="card-tags" {
                        (difficulty_tag(&c.meta.difficulty))
                        (role_tags(&c.meta.roles))
                        (read_time_tag(c.read_time_minutes))
                        (xp_tag(15))
                    }
                }
            }
        }
    })
}

pub async fn concept_detail(
    headers: HeaderMap,
    State(store): State<Arc<ContentStore>>,
    Path(id): Path<String>,
) -> Result<Markup, StatusCode> {
    let concept = store.concepts.get(&id).ok_or(StatusCode::NOT_FOUND)?;
    let progress = UserProgress::from_headers(&headers);
    let done = progress.completed_concepts.contains(&id);

    Ok(page(&concept.meta.title, "Concepts", &headers, html! {
        a href="/concepts" class="back-btn" { "\u{2190} Back to Concepts" }

        div class="content-header" {
            h2 {
                (completed_check(done))
                (concept.meta.title)
            }
            div class="content-meta" {
                (difficulty_tag(&concept.meta.difficulty))
                (role_tags(&concept.meta.roles))
                span class="tag tag-default" { (concept.meta.category) }
                (read_time_tag(concept.read_time_minutes))
            }
        }

        div class="content-body" {
            (PreEscaped(&concept.body_html))
        }

        // Mark complete button
        div class="complete-section" {
            @if done {
                div class="complete-done" {
                    span { "\u{2713}" }
                    " You've completed this concept"
                }
            } @else {
                form action=(format!("/concepts/{}/complete", id)) method="post" {
                    button type="submit" class="complete-btn" {
                        "Mark as Complete  (+15 XP)"
                    }
                }
            }
        }

        @if !concept.meta.related.is_empty() {
            div class="related-section" {
                h3 { "Up Next" }
                div class="path-items" {
                    @for r in &concept.meta.related {
                        @let r_done = progress.completed_concepts.contains(r);
                        a href=(format!("/concepts/{}", r)) class=(if r_done { "path-item path-item-done" } else { "path-item" }) {
                            (completed_check(r_done))
                            (r)
                        }
                    }
                }
            }
        }
    }))
}

pub async fn complete_concept(
    headers: HeaderMap,
    State(_store): State<Arc<ContentStore>>,
    Path(id): Path<String>,
) -> Response {
    let mut progress = UserProgress::from_headers(&headers);
    progress.complete_concept(&id);

    let mut builder = Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header(header::LOCATION, format!("/concepts/{}", id));

    for cookie in progress.to_set_cookies() {
        builder = builder.header(header::SET_COOKIE, cookie);
    }

    builder.body(axum::body::Body::empty()).unwrap()
}

// --- Workflows ---

pub async fn workflows_list(
    headers: HeaderMap,
    State(store): State<Arc<ContentStore>>,
    Query(params): Query<FilterParams>,
) -> Markup {
    let progress = UserProgress::from_headers(&headers);
    let mut workflows: Vec<_> = store
        .workflows
        .values()
        .filter(|w| {
            if let Some(ref role) = params.role {
                if !w.roles.contains(role) {
                    return false;
                }
            }
            true
        })
        .collect();

    workflows.sort_by(|a, b| a.title.cmp(&b.title));

    page("Workflows", "Workflows", &headers, html! {
        h2 class="page-title" { "Workflows" }
        p class="page-subtitle" {
            "Step-by-step guides for common EPIC tasks. "
            strong { (progress.completed_workflows.len()) }
            (format!("/{} completed", workflows.len()))
        }

        div class="card-grid" {
            @for w in &workflows {
                @let done = progress.completed_workflows.contains(&w.id);
                a href=(format!("/workflows/{}", w.id)) class=(if done { "card card-completed" } else { "card" }) {
                    h3 {
                        (completed_check(done))
                        (w.title)
                    }
                    div class="card-tags" {
                        (difficulty_tag(&w.difficulty))
                        (role_tags(&w.roles))
                        span class="tag tag-default" { (format!("{} min", w.estimated_minutes)) }
                        (xp_tag(25))
                    }
                }
            }
        }
    })
}

pub async fn workflow_detail(
    headers: HeaderMap,
    State(store): State<Arc<ContentStore>>,
    Path(id): Path<String>,
) -> Result<Markup, StatusCode> {
    let workflow = store.workflows.get(&id).ok_or(StatusCode::NOT_FOUND)?;
    let progress = UserProgress::from_headers(&headers);
    let done = progress.completed_workflows.contains(&id);

    Ok(page(&workflow.title, "Workflows", &headers, html! {
        a href="/workflows" class="back-btn" { "\u{2190} Back to Workflows" }

        div class="content-header" {
            h2 {
                (completed_check(done))
                (workflow.title)
            }
            div class="content-meta" {
                (difficulty_tag(&workflow.difficulty))
                (role_tags(&workflow.roles))
                span class="tag tag-default" { (format!("{} min", workflow.estimated_minutes)) }
                span class="tag tag-default" { (workflow.module) }
            }
        }

        @if !workflow.prerequisites.is_empty() {
            div class="prereq-box" {
                strong { "Prerequisites: " }
                @for p in &workflow.prerequisites {
                    @let p_done = progress.completed_concepts.contains(p);
                    a href=(format!("/concepts/{}", p)) class=(if p_done { "path-item path-item-done" } else { "path-item" }) {
                        (completed_check(p_done))
                        (p)
                    }
                    " "
                }
            }
        }

        div class="workflow-steps" {
            @for step in &workflow.steps {
                div class="workflow-step" {
                    div class="step-number" { (step.number) }
                    h4 { (step.title) }
                    p class="step-desc" { (step.description) }
                    div class="step-action" { (step.action) }
                    @if !step.tips.is_empty() {
                        ul class="step-tips" {
                            @for tip in &step.tips {
                                li { (tip) }
                            }
                        }
                    }
                }
            }
        }

        div class="complete-section" {
            @if done {
                div class="complete-done" {
                    span { "\u{2713}" }
                    " You've completed this workflow"
                }
            } @else {
                form action=(format!("/workflows/{}/complete", id)) method="post" {
                    button type="submit" class="complete-btn" {
                        "Mark as Complete  (+25 XP)"
                    }
                }
            }
        }
    }))
}

pub async fn complete_workflow(
    headers: HeaderMap,
    State(_store): State<Arc<ContentStore>>,
    Path(id): Path<String>,
) -> Response {
    let mut progress = UserProgress::from_headers(&headers);
    progress.complete_workflow(&id);

    let mut builder = Response::builder()
        .status(StatusCode::SEE_OTHER)
        .header(header::LOCATION, format!("/workflows/{}", id));

    for cookie in progress.to_set_cookies() {
        builder = builder.header(header::SET_COOKIE, cookie);
    }

    builder.body(axum::body::Body::empty()).unwrap()
}

// --- Learning Paths ---

pub async fn paths_list(
    headers: HeaderMap,
    State(store): State<Arc<ContentStore>>,
) -> Markup {
    let mut paths: Vec<_> = store.paths.values().collect();
    paths.sort_by(|a, b| a.title.cmp(&b.title));

    page("Learning Paths", "Learning Paths", &headers, html! {
        h2 class="page-title" { "Learning Paths" }
        p class="page-subtitle" { "Structured curricula tailored to your role. Follow a path from beginner to confident EPIC user." }

        div class="card-grid" {
            @for p in &paths {
                a href=(format!("/paths/{}", p.id)) class="card" {
                    h3 { (p.title) }
                    p { (p.description) }
                    div class="card-tags" {
                        (difficulty_tag(&p.difficulty))
                        span class="tag tag-role" { (p.role) }
                        span class="tag tag-default" { (format!("{}h estimated", p.estimated_hours)) }
                    }
                }
            }
        }
    })
}

pub async fn path_detail(
    headers: HeaderMap,
    State(store): State<Arc<ContentStore>>,
    Path(id): Path<String>,
) -> Result<Markup, StatusCode> {
    let path = store.paths.get(&id).ok_or(StatusCode::NOT_FOUND)?;
    let progress = UserProgress::from_headers(&headers);

    // Count total and completed items in this path
    let mut total_items = 0usize;
    let mut completed_items = 0usize;
    for m in &path.modules {
        for c in &m.concepts {
            total_items += 1;
            if progress.completed_concepts.contains(c) {
                completed_items += 1;
            }
        }
        for w in &m.workflows {
            total_items += 1;
            if progress.completed_workflows.contains(w) {
                completed_items += 1;
            }
        }
    }
    let path_pct = if total_items > 0 {
        (completed_items * 100) / total_items
    } else {
        0
    };

    Ok(page(&path.title, "Learning Paths", &headers, html! {
        a href="/paths" class="back-btn" { "\u{2190} Back to Learning Paths" }

        div class="content-header" {
            h2 { (path.title) }
            div class="content-meta" {
                (difficulty_tag(&path.difficulty))
                span class="tag tag-role" { (path.role) }
                span class="tag tag-default" { (format!("{} hours estimated", path.estimated_hours)) }
            }
        }

        // Path progress bar
        div class="path-progress" {
            div class="path-progress-header" {
                span { (format!("{}/{} completed", completed_items, total_items)) }
                span { (format!("{}%", path_pct)) }
            }
            div class="path-progress-bar" {
                div class="path-progress-fill" style=(format!("width:{}%", path_pct)) {}
            }
        }

        p style="color: var(--warm-400); margin-bottom: 36px; line-height: 1.7; font-size: 15px;" { (path.description) }

        @for (i, m) in path.modules.iter().enumerate() {
            div class="path-module" {
                h3 { (format!("{}. {}", i + 1, m.title)) }
                p { (m.description) }
                div class="path-items" {
                    @for c in &m.concepts {
                        @let c_done = progress.completed_concepts.contains(c);
                        a href=(format!("/concepts/{}", c)) class=(if c_done { "path-item path-item-done" } else { "path-item" }) {
                            (completed_check(c_done))
                            (c)
                        }
                    }
                    @for w in &m.workflows {
                        @let w_done = progress.completed_workflows.contains(w);
                        a href=(format!("/workflows/{}", w)) class=(if w_done { "path-item path-item-done" } else { "path-item" })
                          style="border-color: var(--teal-500);" {
                            (completed_check(w_done))
                            (w)
                        }
                    }
                }
            }
        }
    }))
}

// --- Quizzes ---

pub async fn quizzes_list(
    headers: HeaderMap,
    State(store): State<Arc<ContentStore>>,
) -> Markup {
    let progress = UserProgress::from_headers(&headers);
    let mut quizzes: Vec<_> = store.quizzes.values().collect();
    quizzes.sort_by(|a, b| a.title.cmp(&b.title));

    page("Quizzes", "Quizzes", &headers, html! {
        h2 class="page-title" { "Quizzes" }
        p class="page-subtitle" { "Test your understanding of EPIC concepts. Get instant feedback with detailed explanations." }

        div class="card-grid" {
            @for q in &quizzes {
                @let score = progress.quiz_scores.iter().find(|(id, _, _)| id == &q.id);
                a href=(format!("/quizzes/{}", q.id)) class=(if score.is_some() { "card card-completed" } else { "card" }) {
                    h3 {
                        @if let Some((_, s, t)) = score {
                            span class="completed-badge" title="Taken" { (format!("{}/{}", s, t)) }
                        }
                        (q.title)
                    }
                    p { (q.description) }
                    div class="card-tags" {
                        (difficulty_tag(&q.difficulty))
                        span class="tag tag-default" { (format!("{} questions", q.questions.len())) }
                        (xp_tag(q.questions.len() as u32 * 10))
                    }
                }
            }
        }
    })
}

pub async fn quiz_detail(
    headers: HeaderMap,
    State(store): State<Arc<ContentStore>>,
    Path(id): Path<String>,
) -> Result<Markup, StatusCode> {
    let quiz = store.quizzes.get(&id).ok_or(StatusCode::NOT_FOUND)?;

    Ok(page(&quiz.title, "Quizzes", &headers, html! {
        a href="/quizzes" class="back-btn" { "\u{2190} Back to Quizzes" }

        div class="content-header" {
            h2 { (quiz.title) }
            div class="content-meta" {
                (difficulty_tag(&quiz.difficulty))
                span class="tag tag-default" { (format!("{} questions", quiz.questions.len())) }
                (xp_tag(quiz.questions.len() as u32 * 10))
            }
        }

        p style="color: var(--warm-400); margin-bottom: 28px; font-size: 15px;" { (quiz.description) }

        form action=(format!("/quizzes/{}/submit", quiz.id)) method="post" {
            @for (qi, q) in quiz.questions.iter().enumerate() {
                div class="quiz-question" {
                    h4 { (format!("{}. {}", qi + 1, q.text)) }
                    @for (oi, option) in q.options.iter().enumerate() {
                        label class="quiz-option" {
                            input type="radio" name=(format!("q{}", qi)) value=(oi.to_string());
                            (option)
                        }
                    }
                }
            }

            button type="submit" class="submit-btn" {
                "Submit Answers"
            }
        }
    }))
}

#[derive(Deserialize)]
pub struct QuizSubmission {
    #[serde(flatten)]
    pub answers: std::collections::HashMap<String, String>,
}

pub async fn quiz_submit(
    headers: HeaderMap,
    State(store): State<Arc<ContentStore>>,
    Path(id): Path<String>,
    Form(submission): Form<QuizSubmission>,
) -> Result<Response, StatusCode> {
    let quiz = store.quizzes.get(&id).ok_or(StatusCode::NOT_FOUND)?;
    let mut progress = UserProgress::from_headers(&headers);

    let mut score = 0u32;
    let total = quiz.questions.len() as u32;

    let mut results: Vec<(bool, Option<usize>, usize)> = Vec::new();

    for (qi, question) in quiz.questions.iter().enumerate() {
        let key = format!("q{}", qi);
        let selected = submission
            .answers
            .get(&key)
            .and_then(|v| v.parse::<usize>().ok());

        let correct = selected == Some(question.correct);
        if correct {
            score += 1;
        }
        results.push((correct, selected, question.correct));
    }

    let percentage = if total > 0 {
        (score * 100) / total
    } else {
        0
    };

    progress.record_quiz(&id, score, total);
    let xp_earned = score * 10 + if score == total && total > 0 { 30 } else { 0 };

    let grade = match percentage {
        90..=100 => ("Excellent!", "grade-excellent"),
        70..=89 => ("Good Job!", "grade-good"),
        50..=69 => ("Keep Learning", "grade-ok"),
        _ => ("Try Again", "grade-low"),
    };

    let body = page_with_theme(
        &format!("Results: {}", quiz.title),
        "Quizzes",
        is_dark(&headers),
        &progress,
        html! {
            a href=(format!("/quizzes/{}", id)) class="back-btn" { "\u{2190} Retake Quiz" }

            div class="quiz-result-hero" {
                div class=(format!("quiz-grade {}", grade.1)) { (grade.0) }
                div class="quiz-score-big" {
                    (format!("{}/{}", score, total))
                }
                div class="quiz-score-pct" { (format!("{}%", percentage)) }
                div class="quiz-xp-earned" { (format!("+{} XP earned", xp_earned)) }
            }

            div style="margin-top: 32px;" {
                @for (qi, question) in quiz.questions.iter().enumerate() {
                    @let (is_correct, selected, correct_idx) = results[qi];
                    div class="quiz-question" {
                        h4 {
                            @if is_correct {
                                span style="color: var(--teal-500); margin-right: 8px;" { "\u{2713}" }
                            } @else {
                                span style="color: var(--rose-500); margin-right: 8px;" { "\u{2717}" }
                            }
                            (format!("{}. {}", qi + 1, question.text))
                        }

                        @for (oi, option) in question.options.iter().enumerate() {
                            @let class_name = if oi == correct_idx {
                                "quiz-option correct"
                            } else if Some(oi) == selected && !is_correct {
                                "quiz-option incorrect"
                            } else {
                                "quiz-option"
                            };
                            div class=(class_name) {
                                @if oi == correct_idx {
                                    span style="color: var(--teal-500); font-weight: 600; margin-right: 8px;" { "\u{2713}" }
                                }
                                @if Some(oi) == selected && oi != correct_idx {
                                    span style="color: var(--rose-500); font-weight: 600; margin-right: 8px;" { "\u{2717}" }
                                }
                                (option)
                            }
                        }

                        div class="quiz-explanation visible" {
                            (question.explanation)
                        }
                    }
                }
            }
        },
    );

    let mut builder = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "text/html; charset=utf-8");

    for cookie in progress.to_set_cookies() {
        builder = builder.header(header::SET_COOKIE, cookie);
    }

    Ok(builder
        .body(axum::body::Body::from(body.into_string()))
        .unwrap())
}

// --- Progress Page ---

pub async fn progress_page(
    headers: HeaderMap,
    State(store): State<Arc<ContentStore>>,
) -> Markup {
    let progress = UserProgress::from_headers(&headers);
    let (rank_name, rank_desc) = progress.rank();
    let rank_pct = progress.rank_progress_percent();
    let total_concepts = store.concepts.len();
    let total_workflows = store.workflows.len();
    let earned = progress.earned_badges(total_concepts, total_workflows);

    page_with_theme("Progress", "Progress", is_dark(&headers), &progress, html! {
        h2 class="page-title" { "Your Progress" }

        // Rank card
        div class="rank-card" {
            div class="rank-card-icon" { (rank_name.chars().next().unwrap_or('I')) }
            div class="rank-card-info" {
                div class="rank-card-name" { (rank_name) }
                div class="rank-card-desc" { (rank_desc) }
                div class="rank-bar" {
                    div class="rank-bar-fill" style=(format!("width:{}%", rank_pct)) {}
                }
                @if let Some((next_name, next_xp)) = progress.next_rank() {
                    div class="rank-next" {
                        (format!("{} XP to {}", next_xp - progress.xp, next_name))
                    }
                } @else {
                    div class="rank-next" { "Max rank achieved!" }
                }
            }
            div class="rank-card-xp" {
                div class="rank-xp-num" { (progress.xp) }
                div class="rank-xp-label" { "Total XP" }
            }
        }

        // Stats
        div class="section-title" { "Statistics" }
        div class="stats-grid" {
            div class="stat-card" {
                div class="stat-number" { (progress.completed_concepts.len()) }
                div class="stat-label" { "Concepts Read" }
                div class="stat-desc" { (format!("out of {}", total_concepts)) }
            }
            div class="stat-card" {
                div class="stat-number" { (progress.completed_workflows.len()) }
                div class="stat-label" { "Workflows Done" }
                div class="stat-desc" { (format!("out of {}", total_workflows)) }
            }
            div class="stat-card" {
                div class="stat-number" { (progress.quiz_scores.len()) }
                div class="stat-label" { "Quizzes Taken" }
                div class="stat-desc" {
                    @if let Some(avg) = avg_quiz_score(&progress) {
                        (format!("{}% avg score", avg))
                    } @else {
                        "Take your first quiz!"
                    }
                }
            }
            div class="stat-card" {
                div class="stat-number" { (earned.len()) }
                div class="stat-label" { "Badges Earned" }
                div class="stat-desc" { (format!("out of {}", BADGES.len())) }
            }
        }

        // Badges
        div class="section-title" { "Badges" }
        div class="badges-grid" {
            @for &(id, name, desc) in BADGES {
                @let is_earned = earned.iter().any(|&(eid, _, _)| eid == id);
                div class=(if is_earned { "badge-card badge-earned" } else { "badge-card badge-locked" }) {
                    div class="badge-icon" {
                        @if is_earned { "\u{2605}" } @else { "\u{2606}" }
                    }
                    div class="badge-name" { (name) }
                    div class="badge-desc" { (desc) }
                }
            }
        }

        // Quiz history
        @if !progress.quiz_scores.is_empty() {
            div class="section-title" { "Quiz History" }
            @for (quiz_id, s, t) in &progress.quiz_scores {
                @let pct = if *t > 0 { (s * 100) / t } else { 0 };
                div class="quiz-history-row" {
                    a href=(format!("/quizzes/{}", quiz_id)) { (quiz_id) }
                    div class="quiz-history-bar-wrap" {
                        div class="quiz-history-bar" style=(format!("width:{}%", pct)) {}
                    }
                    span class="quiz-history-score" { (format!("{}/{} ({}%)", s, t, pct)) }
                }
            }
        }
    })
}

fn avg_quiz_score(progress: &UserProgress) -> Option<u32> {
    if progress.quiz_scores.is_empty() {
        return None;
    }
    let total_pct: u32 = progress
        .quiz_scores
        .iter()
        .map(|(_, s, t)| if *t > 0 { (s * 100) / t } else { 0 })
        .sum();
    Some(total_pct / progress.quiz_scores.len() as u32)
}

// --- Search ---

pub async fn search(
    headers: HeaderMap,
    State(store): State<Arc<ContentStore>>,
    Query(params): Query<FilterParams>,
) -> Markup {
    let q = params.q.unwrap_or_default();
    let q_lower = q.to_lowercase();

    let mut results: Vec<(&str, String, String, String)> = Vec::new();

    if q_lower.len() >= 2 {
        for concept in store.concepts.values() {
            if concept.meta.title.to_lowercase().contains(&q_lower)
                || concept
                    .meta
                    .tags
                    .iter()
                    .any(|t| t.to_lowercase().contains(&q_lower))
                || concept.body_markdown.to_lowercase().contains(&q_lower)
            {
                let snippet = extract_snippet(&concept.body_markdown, &q_lower);
                results.push((
                    "concept",
                    concept.meta.id.clone(),
                    concept.meta.title.clone(),
                    snippet,
                ));
            }
        }

        for workflow in store.workflows.values() {
            if workflow.title.to_lowercase().contains(&q_lower)
                || workflow
                    .tags
                    .iter()
                    .any(|t| t.to_lowercase().contains(&q_lower))
            {
                let snippet = workflow
                    .steps
                    .first()
                    .map(|s| s.description.clone())
                    .unwrap_or_default();
                results.push((
                    "workflow",
                    workflow.id.clone(),
                    workflow.title.clone(),
                    snippet,
                ));
            }
        }

        for quiz in store.quizzes.values() {
            if quiz.title.to_lowercase().contains(&q_lower)
                || quiz.description.to_lowercase().contains(&q_lower)
            {
                results.push((
                    "quiz",
                    quiz.id.clone(),
                    quiz.title.clone(),
                    quiz.description.clone(),
                ));
            }
        }
    }

    page("Search", "Home", &headers, html! {
        form action="/search" method="get" class="search-bar" {
            input type="text" name="q" placeholder="Search concepts, workflows, quizzes..."
                  value=(q) autocomplete="off";
        }

        h2 class="page-title" {
            @if q.is_empty() {
                "Search"
            } @else {
                (format!("{} results for \"{}\"", results.len(), q))
            }
        }

        @if results.is_empty() && !q.is_empty() {
            div class="empty-state" { "No results found. Try a different search term." }
        }

        @for (rtype, rid, rtitle, rsnippet) in &results {
            @let href = format!("/{}s/{}", rtype, rid);
            a href=(href) class="card" style="display:block; margin-bottom: 12px;" {
                h3 {
                    (rtitle) " "
                    span class="tag tag-default" { (rtype) }
                }
                p { (rsnippet) }
            }
        }
    })
}

fn extract_snippet(text: &str, query: &str) -> String {
    let lower = text.to_lowercase();
    if let Some(pos) = lower.find(query) {
        let start = pos.saturating_sub(50);
        let end = (pos + query.len() + 100).min(text.len());
        let start = text[..start]
            .char_indices()
            .last()
            .map(|(i, _)| i)
            .unwrap_or(0);
        let end = text[end..]
            .char_indices()
            .next()
            .map(|(i, _)| end + i)
            .unwrap_or(text.len());
        let mut snippet = text[start..end].to_string();
        if start > 0 {
            snippet = format!("...{}", snippet);
        }
        if end < text.len() {
            snippet = format!("{}...", snippet);
        }
        snippet
    } else {
        text.chars().take(150).collect()
    }
}
