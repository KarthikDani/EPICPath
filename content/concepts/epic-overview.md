---
id: epic-overview
title: "EPIC Overview"
category: core
tags: ["fundamentals", "ehr", "introduction"]
related: ["mychart", "hyperspace", "epic-modules"]
roles: ["all"]
difficulty: beginner
---

# What is EPIC?

EPIC is the most widely used Electronic Health Record (EHR) system in the United States. It is developed by Epic Systems Corporation, based in Verona, Wisconsin. EPIC is used by large health systems, academic medical centers, and community hospitals to manage patient records, clinical workflows, billing, and more.

## Key Things to Know

- EPIC is **not a single application** -- it is a suite of interconnected modules, each handling a different part of healthcare (lab, radiology, pharmacy, surgery, billing, etc.)
- The main user interface is called **Hyperspace** (the installed desktop app) or **Hyperdrive** (the newer web-based version)
- Patients interact with EPIC through **MyChart**, a patient portal
- EPIC uses a proprietary database called **Chronicles** built on the MUMPS/Cache platform
- Each hospital configures EPIC differently, so workflows can vary significantly between institutions

## Core Architecture

EPIC's architecture revolves around a few key ideas:

1. **Master Files** -- Central configuration tables that define how the system behaves (order types, user roles, department settings)
2. **Records** -- Patient records, encounter records, order records, etc.
3. **Workflows** -- Sequences of actions that users perform (ordering a lab, signing a note, discharging a patient)
4. **SmartTools** -- Reusable building blocks like SmartPhrases, SmartLinks, and SmartLists that speed up documentation

## Why EPIC Feels Overwhelming

- Thousands of buttons, menus, and configuration options
- Heavy use of jargon and abbreviations (ADT, FYI flags, InBasket, etc.)
- Different roles see completely different interfaces
- Workflows depend on how YOUR hospital configured EPIC
- Training materials are often locked behind institutional access
