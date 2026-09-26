---
status: accepted; mechanism superseded by ADR-0023
date: 2026-09-25
decision-makers: Joey
---

# Registry-driven validation with severity policy and user rules

## Context and Problem Statement

The prototype's validator was 13 hand-coded rules, several enforcing invented
keyword sets and wrong P-group meanings, while its registry metadata
(required tags, value types, deprecation) went unused. Validation profiles and
per-rule severities existed in the API but did nothing. Users need to validate
against a named edition and to relax or tighten rules for their own files.

## Decision Drivers

* Rules come from the standard's own usage-attribute fields, not from memory
* The edition being validated against is explicit
* Users control what is fatal

## Considered Options

* **One generic pass per attribute driven by the registry** (pattern →
  edition validity → Allowed/Required when → Range → value type), plus
  hand-written rules only for cross-document checks (index contiguity "with no
  missing values" per §9.5.1 a, link resolution, key uniqueness)
* Hand-written rules per attribute (the prototype)

## Decision Outcome

Chosen option: **registry-driven validation**. Findings carry a stable rule
identifier, severity, location, and Chapter 9 citation. A severity policy can
raise, lower, or disable any rule; users can add rules through a public trait;
a "Chapter 10 recorder" profile applies the R/R Ch 10 Status rules. A value
longer than a *recommended* maximum length is a warning by default (owner
decision, 2026-09-25).

### Consequences

* Good: validation, views, and links cannot disagree about what an attribute
  means.
* Good: "our files violate X on purpose" is a one-line policy setting.
* Bad: validation quality depends on the registry transcription (ADR-0004).
