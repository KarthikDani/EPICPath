# EpicPath

A self-hosted learning platform for mastering EPIC EHR (Electronic Health Record), built entirely in Rust. Designed for pathology residents, lab technicians, and healthcare professionals who want to go from EPIC beginner to EPIC Expert.

**Zero JavaScript.** Server-side rendered HTML via [maud](https://maud.lambda.xyz/), served by [axum](https://github.com/tokio-rs/axum). All content lives in flat files (Markdown + YAML) — no database required.

## Features

### Content Types

- **Concepts** — Markdown articles with YAML frontmatter covering EPIC fundamentals, modules, and tools (Hyperspace, Beaker, InBasket, SmartPhrases, etc.)
- **Workflows** — Step-by-step procedural guides for real clinical tasks (accessioning specimens, signing out cases, reviewing results)
- **Learning Paths** — Curated multi-week curricula that group concepts and workflows into structured modules
- **Quizzes** — Multiple-choice assessments tied to specific concepts, with explanations for each answer

### Gamification

EpicPath tracks your progress and rewards learning through an XP-based rank system:

| XP Threshold | Rank | Description |
|---|---|---|
| 0 | Intern | Just getting started |
| 50 | Observer | Learning the basics |
| 120 | Resident | Building core skills |
| 250 | Fellow | Developing expertise |
| 400 | Attending | Confident practitioner |
| 600 | Specialist | Deep EPIC knowledge |
| 800 | EPIC Expert | Mastery achieved |

**XP rewards:**

| Action | XP |
|---|---|
| Read a concept | +15 |
| Complete a workflow | +25 |
| Quiz — per correct answer | +10 |
| Quiz — perfect score bonus | +30 |

**Badges** unlock as you hit milestones: First Steps, Curious Mind, Scholar, Hands On, Process Master, Test Taker, Perfect Score, Century Club, and EPIC Expert.

All progress is stored in browser cookies — no accounts, no database, no sign-up friction.

### Other Features

- **Full-text search** across all content types
- **Dark mode** toggle that persists across page navigation (cookie-based)
- **Progress dashboard** showing your rank, XP, earned badges, and quiz history
- **Learning path progress bars** showing completion percentage per module
- **Read time estimates** on every concept article
- **Difficulty tags** and **role tags** on all content
- **Related concepts** and **prerequisites** linking content together

## Tech Stack

| Layer | Crate | Role |
|---|---|---|
| HTTP server | `axum 0.8` | Routing, request handling, state management |
| Async runtime | `tokio 1` | Async I/O |
| HTML templating | `maud 0.27` | Compile-time HTML generation (no JS needed) |
| Markdown | `pulldown-cmark 0.12` | Concept articles: Markdown to HTML |
| Content parsing | `serde_yaml 0.9` | YAML frontmatter and content files |
| Serialization | `serde 1` | Data model derive macros |
| Static files | `tower-http 0.6` | Serving CSS from `/static` |

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.75+ recommended, edition 2021)

## Quick Start

```sh
# Clone the repository
git clone <repo-url>
cd RustExperiment

# Build and run
cargo run
```

The server starts at **http://localhost:3333**.

## Project Structure

```
.
├── Cargo.toml
├── content/                    # All learning content (flat files)
│   ├── concepts/               # Markdown articles with YAML frontmatter
│   │   ├── epic-overview.md
│   │   ├── hyperspace.md
│   │   ├── beaker.md
│   │   ├── beaker-ap.md
│   │   ├── orders.md
│   │   ├── results.md
│   │   ├── inbasket.md
│   │   ├── smartphrases.md
│   │   ├── epic-modules.md
│   │   ├── mychart.md
│   │   └── navigator.md
│   ├── workflows/              # Step-by-step procedural guides (YAML)
│   │   ├── accessioning-specimen.yaml
│   │   ├── signing-out-case.yaml
│   │   └── chart-review-results.yaml
│   ├── paths/                  # Curated learning curricula (YAML)
│   │   └── new-pathology-resident.yaml
│   └── quizzes/                # Multiple-choice assessments (YAML)
│       ├── beaker-basics.yaml
│       └── epic-fundamentals.yaml
├── src/
│   ├── main.rs                 # Entry point, route definitions
│   ├── api/
│   │   ├── mod.rs
│   │   └── content.rs          # ContentStore: loads all content at startup
│   ├── models/
│   │   ├── mod.rs
│   │   ├── concept.rs          # Concept with frontmatter + rendered HTML
│   │   ├── workflow.rs         # Workflow with numbered steps and tips
│   │   ├── path.rs             # LearningPath with modules
│   │   └── quiz.rs             # Quiz with multiple-choice questions
│   ├── progress/
│   │   └── mod.rs              # XP, ranks, badges, cookie persistence
│   └── views/
│       ├── mod.rs
│       ├── layout.rs           # Page shell, nav bar, XP pill
│       └── pages.rs            # All page handlers and HTML generation
└── static/
    └── css/
        └── style.css           # Full stylesheet (light + dark themes)
```

## Routes

| Method | Path | Description |
|---|---|---|
| GET | `/` | Home page |
| GET | `/concepts` | List all concepts |
| GET | `/concepts/{id}` | Read a concept article |
| POST | `/concepts/{id}/complete` | Mark concept as read (+15 XP) |
| GET | `/workflows` | List all workflows |
| GET | `/workflows/{id}` | View workflow steps |
| POST | `/workflows/{id}/complete` | Mark workflow complete (+25 XP) |
| GET | `/paths` | List learning paths |
| GET | `/paths/{id}` | View path with module progress |
| GET | `/quizzes` | List all quizzes |
| GET | `/quizzes/{id}` | Take a quiz |
| POST | `/quizzes/{id}/submit` | Submit quiz answers (awards XP) |
| GET | `/search` | Search across all content |
| GET | `/progress` | Progress dashboard (rank, badges, history) |
| POST | `/toggle-theme` | Toggle dark/light mode |

## Adding Content

### Concepts

Create a Markdown file in `content/concepts/` with YAML frontmatter:

```markdown
---
id: your-concept-id
title: "Your Concept Title"
category: core
tags: ["tag1", "tag2"]
related: ["other-concept-id"]
roles: ["pathologist", "lab-tech"]
difficulty: beginner
---

# Your Concept Title

Write your content here using standard Markdown. Tables and
strikethrough are supported.
```

**Fields:**
- `id` — Unique identifier, used in URLs (`/concepts/{id}`)
- `title` — Display name
- `category` — Grouping category (e.g., `core`, `lab`, `clinical`)
- `tags` — Searchable tags
- `related` — IDs of related concepts (rendered as links)
- `roles` — Target audience (`all`, `pathologist`, `lab-tech`, etc.)
- `difficulty` — `beginner`, `intermediate`, or `advanced`

### Workflows

Create a YAML file in `content/workflows/`:

```yaml
id: your-workflow-id
title: "Your Workflow Title"
module: beaker-ap
roles: ["pathologist"]
difficulty: beginner
estimated_minutes: 10
tags: ["tag1", "tag2"]
prerequisites: ["concept-id-1", "concept-id-2"]

steps:
  - number: 1
    title: "Step title"
    description: "What this step involves."
    action: "The specific action to take."
    tips:
      - "Helpful tip for this step"
      - "Another tip"
  - number: 2
    title: "Next step"
    description: "..."
    action: "..."
    tips: []
```

### Learning Paths

Create a YAML file in `content/paths/`:

```yaml
id: your-path-id
title: "Your Learning Path"
description: "A description of what this path covers."
role: pathologist
difficulty: beginner
estimated_hours: 4

modules:
  - id: module-1
    title: "Module Title"
    description: "What this module covers."
    concepts:
      - concept-id-1
      - concept-id-2
    workflows:
      - workflow-id-1
```

The progress bar for each module is automatically calculated based on how many of its concepts and workflows the user has completed.

### Quizzes

Create a YAML file in `content/quizzes/`:

```yaml
id: your-quiz-id
title: "Your Quiz Title"
description: "What this quiz tests."
concepts: ["concept-id-1", "concept-id-2"]
difficulty: beginner

questions:
  - id: q1
    text: "Your question text?"
    type: multiple_choice
    options:
      - "Option A"
      - "Option B"
      - "Option C"
      - "Option D"
    correct: 0
    explanation: "Why Option A is correct."
```

The `correct` field is a zero-based index into the `options` array.

## Configuration

| Environment Variable | Default | Description |
|---|---|---|
| `EPICPATH_CONTENT_DIR` | `content` | Path to the content directory |

The server always binds to `0.0.0.0:3333`.

## How Progress Works

Progress is stored entirely in browser cookies (no server-side state):

| Cookie | Format | Example |
|---|---|---|
| `ep_concepts` | Comma-separated IDs | `epic-overview,hyperspace,beaker` |
| `ep_workflows` | Comma-separated IDs | `accessioning-specimen` |
| `ep_quizzes` | `id:score:total,...` | `beaker-basics:4:5,epic-fundamentals:3:4` |
| `ep_xp` | Integer | `145` |
| `theme` | `dark` or `light` | `dark` |

Cookies persist for 1 year. Clearing your browser cookies resets all progress.

## License

MIT
