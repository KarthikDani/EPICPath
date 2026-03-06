---
id: orders
title: "Orders"
category: core
tags: ["orders", "order-entry", "cpoe", "fundamentals"]
related: ["epic-overview", "beaker", "results", "inbasket"]
roles: ["clinician", "nurse", "lab-tech"]
difficulty: beginner
---

# Orders in EPIC

Orders are the backbone of clinical workflow in EPIC. When a physician wants a lab test, imaging study, medication, or procedure, they create an order. EPIC then routes that order to the right department for fulfillment.

## Types of Orders

- **Laboratory Orders** -- Blood work, urinalysis, cultures, pathology specimens
- **Radiology Orders** -- X-rays, CT scans, MRIs, ultrasounds
- **Medication Orders** -- Prescriptions, IV medications, PRN meds
- **Procedure Orders** -- Surgeries, biopsies, consults
- **Nursing Orders** -- Vital sign frequency, diet, activity level

## CPOE (Computerized Provider Order Entry)

CPOE is the system that allows providers to enter orders electronically. Key features:

- **Order Sets** -- Pre-built groups of common orders (e.g., "Post-op Day 1" or "Sepsis Workup")
- **Clinical Decision Support (CDS)** -- Alerts that fire when orders might conflict (drug interactions, duplicate orders, allergy alerts)
- **Order Routing** -- EPIC automatically sends orders to the right lab, pharmacy, or department based on configuration

## Order Lifecycle

1. **Placed** -- Provider enters the order
2. **Signed** -- Provider authorizes the order (may require cosignature for trainees)
3. **Sent** -- Order is transmitted to the fulfilling department
4. **In Progress** -- Specimen collected, test running, etc.
5. **Completed** -- Results are entered and verified
6. **Resulted** -- Results are available for the ordering provider to review

## Common Pitfalls

- Ordering the wrong test because of similar names (e.g., "TSH" vs "Free T4")
- Not selecting the right order mode (inpatient vs outpatient)
- Missing required fields like diagnosis codes (ICD-10) for outpatient orders
- Duplicate orders from multiple providers on the same patient
