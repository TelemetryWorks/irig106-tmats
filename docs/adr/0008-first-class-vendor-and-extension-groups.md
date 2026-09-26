---
status: accepted; extended to the H group by 0027
date: 2026-09-25
decision-makers: Joey
---

# Vendor (V) and extension (X) groups are first-class

## Context and Problem Statement

Chapter 9 provides two extension mechanisms of its own. V-group attributes
(`V-x\acr\attribute-string`, §9.5.13) carry vendor-specific data tied to a
data source. X-group attributes
(`X-x\ORGANIZATION\ORIGCODE\EXTENSION_CODE-i-j-m-n`, §9.5.14) extend an
existing attribute; §9.5.14 asks that an editor "notice the association and
preserve it even if the editor doesn't know what the code means", updating the
extension's indices if the original is renumbered. The prototype knew neither
group.

## Decision Drivers

* Real files use vendor and extension attributes
* The standard defines how extensions attach to other attributes
* Edits must not break those attachments

## Considered Options

* **Parse V and X code names structurally, link each X attribute to the
  attribute it extends, and keep the link through edits**
* Treat V and X as opaque unknown attributes

## Decision Outcome

Chosen option: **first-class V and X support**. V attributes are grouped by
data source and vendor acronym. X attributes are linked to their original by
group, code, and leading indices; renumbering the original renumbers its
extensions. Users can register definitions for their own V and X codes in the
registry overlay (ADR-0004).

### Consequences

* Good: the library follows the standard's own extension model.
* Good: extended attributes survive editing.
* Bad: the X-group grammar (organization, original code with its own
  backslashes, extra indices) needs careful parsing and tests.
