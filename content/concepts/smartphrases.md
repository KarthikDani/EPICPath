---
id: smartphrases
title: "SmartPhrases & SmartTools"
category: tools
tags: ["smartphrases", "smarttools", "documentation", "templates", "efficiency"]
related: ["epic-overview", "beaker"]
roles: ["all"]
difficulty: intermediate
---

# SmartPhrases & SmartTools

SmartTools are EPIC's text expansion and automation features that speed up clinical documentation. SmartPhrases are the most commonly used SmartTool.

## SmartPhrases

A SmartPhrase is a reusable text template triggered by typing a dot (.) followed by the phrase name.

### How They Work
- Type `.dischargeinstructions` in a note and it expands to your full discharge template
- Type `.normalpath` and it expands to a normal pathology report template
- They can include dynamic content that auto-fills from the patient's chart

### Types of Content in SmartPhrases
- **Static text** -- Fixed boilerplate that doesn't change
- **SmartLinks** -- Pull live data from the chart (e.g., `@NAME@` inserts the patient's name)
- **SmartLists** -- Drop-down menus for picking options (e.g., tumor grade: well / moderately / poorly differentiated)
- **Wild Cards** -- Blank fields you fill in manually (marked with `***`)

### Creating SmartPhrases
1. Go to the SmartPhrase Manager (usually under Tools menu)
2. Create a new phrase with a memorable name
3. Build your template using text, SmartLinks, and SmartLists
4. Share it with your department or keep it personal

## Other SmartTools

- **SmartLinks** (`@LINK@`) -- Pull specific data from the chart into your text
- **SmartLists** (`{LIST:...}`) -- Create selectable drop-down options
- **SmartText** -- Longer, structured templates (similar to SmartPhrases but managed differently)

## Pathology SmartPhrase Examples

- `.grossskin` -- Gross description template for skin biopsies
- `.microcolon` -- Microscopic template for colon biopsies
- `.synbreast` -- Synoptic report template for breast cancer
- `.normalappendix` -- Normal appendix sign-out template
