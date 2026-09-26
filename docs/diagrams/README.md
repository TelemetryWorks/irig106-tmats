# Diagrams

Hand-authored SVG diagrams of the architecture, embedded in
`docs/ARCHITECTURE.md`. Each one shows one mechanism; the caption beside it in
`ARCHITECTURE.md` states the claim.

| File | Shows |
|------|-------|
| `system-context.svg` | The crate in the ecosystem: CLI, library, lockstep workspace, consumers, shared types |
| `legacy-vs-new.svg` | The `idmptmat` / `irig106lib` pipeline against ours, with each defect (D1–D7) at the stage that causes it |
| `data-flow.svg` | Scanner → Document; Document and registry meet in the read-through layer (link graph, effective-value resolver, condition evaluator); views and the four-pass validator read through it; suggested edits → caller → patch list → writer |
| `derived-parameters.svg` | Appendix 9-E: function and formula styles → binder and Table E-6 parser → description, derivation graph, validation; evaluation in `irig106-decode`; the test-only reference evaluator |
| `registry-pipeline.svg` | Extractor → source layer → author and independent reviewer → interpretation layer → generator, with the four CI checks and the re-review loop |
| `document-model.svg` | One buffer, spans, keys, index, and the byte range the `G\SHA` digest covers |
| `link-graph.svg` | The Chapter 9 §9.5.1 b ties between groups, labelled with the value that carries each |
| `edits-and-checksum.svg` | An edit as a patch, byte-faithful output, and `G\SHA` reported stale and stamped only on request |

## Conventions

- Plain SVG with an embedded `<style>`: light colors by default and a
  `prefers-color-scheme: dark` block, so the diagrams follow the reader's
  GitHub theme. No scripts, no external fonts or images.
- `role="img"` and an `aria-label` stating the same claim as the caption.
- Label arrows with what flows along them; amber marks what the user or
  caller controls; red marks a defect or a loss; blue marks the stages this
  crate adds.
- When a design decision changes a diagram, update the SVG and its caption in
  the same commit.
