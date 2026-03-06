use maud::{html, Markup, DOCTYPE};

use crate::progress::UserProgress;

pub fn page_with_theme(
    title: &str,
    active: &str,
    is_dark: bool,
    progress: &UserProgress,
    content: Markup,
) -> Markup {
    let body_class = if is_dark { "dark" } else { "" };
    let toggle_label = if is_dark { "\u{263C}" } else { "\u{263E}" };
    let (rank_name, _) = progress.rank();
    let rank_pct = progress.rank_progress_percent();

    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="UTF-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                title { (format!("{} | EpicPath", title)) }
                link rel="preconnect" href="https://fonts.googleapis.com";
                link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="";
                link rel="stylesheet" href="/static/css/style.css";
            }
            body class=(body_class) {
                div class="app" {
                    nav class="sidebar" {
                        div class="sidebar-header" {
                            a href="/" {
                                h1 { "EpicPath" }
                            }
                        }
                        div class="nav-section" {
                            (nav_item("/", "Home", active))
                            (nav_item("/concepts", "Concepts", active))
                            (nav_item("/workflows", "Workflows", active))
                            (nav_item("/paths", "Paths", active))
                            (nav_item("/quizzes", "Quizzes", active))
                        }
                        a href="/progress" class="xp-pill" title=(format!("{} XP — {}", progress.xp, rank_name)) {
                            span class="xp-rank" { (rank_name) }
                            span class="xp-bar-wrap" {
                                span class="xp-bar-fill" style=(format!("width:{}%", rank_pct)) {}
                            }
                            span class="xp-number" { (format!("{} XP", progress.xp)) }
                        }
                        form action="/toggle-theme" method="post" class="theme-toggle-form" {
                            button type="submit" class="theme-toggle-btn" title="Toggle dark mode" {
                                (toggle_label)
                            }
                        }
                    }
                    main class="main" {
                        (content)
                    }
                }
            }
        }
    }
}

fn nav_item(href: &str, label: &str, active: &str) -> Markup {
    let class = if label == active
        || (active == "Learning Paths" && label == "Paths")
        || (active == "Progress" && label == "Progress")
    {
        "nav-item active"
    } else {
        "nav-item"
    };
    html! {
        a href=(href) class=(class) { (label) }
    }
}

pub fn difficulty_tag(d: &str) -> Markup {
    html! {
        span class=(format!("tag tag-{}", d)) { (d) }
    }
}

pub fn role_tags(roles: &[String]) -> Markup {
    html! {
        @for role in roles {
            span class="tag tag-role" { (role) }
        }
    }
}
