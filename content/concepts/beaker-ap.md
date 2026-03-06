---
id: beaker-ap
title: "Beaker AP (Anatomic Pathology)"
category: modules
tags: ["pathology", "beaker", "anatomic-pathology", "surgical-pathology", "cytology"]
related: ["beaker", "orders", "results", "smartphrases"]
roles: ["pathologist", "lab-tech"]
difficulty: intermediate
---

# Beaker AP (Anatomic Pathology)

Beaker AP is the anatomic pathology component of EPIC's Beaker module. It supports the full workflow from specimen receipt through final diagnosis.

## Core Workflow

### 1. Accessioning
- Specimen arrives in the lab with a requisition
- Accessioner scans or enters the order and assigns a case number (e.g., S25-12345)
- Labels are printed for cassettes and slides
- Case appears on the pathologist's worklist

### 2. Grossing
- Gross description is entered (dictated or typed)
- Number of cassettes/blocks is recorded
- Tissue is submitted for processing
- SmartPhrases are commonly used for standardized gross templates

### 3. Slide Preparation
- Histology processes tissue into slides
- Slides are linked to the case in Beaker
- Pathologist receives slides (physical or digital via whole-slide imaging)

### 4. Microscopic Examination & Diagnosis
- Pathologist reviews slides
- Enters microscopic description and final diagnosis
- For cancer cases, completes a synoptic report (CAP protocol)
- May order special stains, IHC, or molecular tests as add-ons

### 5. Sign-Out (Verification)
- Pathologist electronically signs the report
- Report becomes available in Chart Review for the ordering clinician
- InBasket notification is sent

### 6. Amendments & Addenda
- **Addendum**: Additional information added after sign-out (e.g., IHC results)
- **Amendment**: Correction to the original report (requires reason documentation)

## Pathologist Worklist

The AP worklist is the pathologist's main dashboard showing:
- Pending cases awaiting sign-out
- Cases with pending ancillary studies
- Amendment/addendum requests
- Cases assigned to you vs. unassigned

## Synoptic Reporting

For cancer cases, EPIC integrates CAP (College of American Pathologists) cancer protocols:
- Structured data entry for tumor type, size, margins, grade, stage, etc.
- Ensures completeness -- required fields must be filled
- Outputs both a structured report and a narrative summary

## Common Beaker AP Terminology

| Term | Meaning |
|------|---------|
| Case | A single pathology accession (may have multiple specimens/parts) |
| Part | A distinct specimen within a case (e.g., Part A: left breast, Part B: sentinel node) |
| Block | A tissue cassette/block from a part |
| Slide | A glass slide cut from a block |
| Accession Number | Unique identifier for the case (e.g., S25-12345) |
| Sign-Out | The act of finalizing and signing a pathology report |
