# Supply-chain rollout — 2026-11-15

Scope: irig106-tmats workflows/docs and the assigned Dependabot configuration.
Base main: `41030d54dfbd8a539c2810d023dd0670bfd3dc1f`.
No runtime, manifest, lockfile, release, publishing or repository-settings change.
The existing CI packaging dry run remains a dry run. No
`trusted-publishing.yml` exists at this base; none is added.

## Reviewed references and boundaries

Copied Scorecard and zizmor workflows byte for byte from the
[irig106-types pilot](https://github.com/TelemetryWorks/irig106-types/tree/2899b30e338358eb2a52d12e036ff979bc25bcf5/.github),
including Scorecard's workflow-level `contents: read`. Followed the
[spatial-typestate rollout](https://github.com/TelemetryWorks/spatial-typestate/blob/5b34624acc7ffd1e1d36238f6a08e2e012bdd0fa/docs/evals/2026-11-15-supply-chain-rollout.md).
Compared the [official Scorecard template](https://github.com/ossf/scorecard/blob/main/.github/workflows/scorecard-analysis.yml)
and the pinned Scorecard/zizmor action input contracts.

- Scorecard runs on main pushes and weekly, never on PRs. Its job alone grants
  `security-events: write` for SARIF and `id-token: write` for Scorecard results.
  `publish_results: true` publishes security results, not packages. SARIF retention
  is five days. The README badge links to this repository's public Scorecard view.
- zizmor runs on PRs to main touching workflows, audits the entire workflow
  directory with version 1.32.0, the default regular persona and online audits.
  `advanced-security: false` selects failing audit mode without SARIF write grants.
- Every workflow defaults to `contents: read`; all checkouts disable credential
  persistence. No `pull_request_target` event is present. Existing CI job names,
  commands, matrix, toolchain choices and triggers are preserved. Pinning the
  Rust action's `v1` implementation requires an explicit `toolchain: stable` where
  the moving `stable` ref previously selected it; MSRV remains 1.85.
- Dependabot follows the pilot: weekly, five open PRs per ecosystem entry,
  `chore(deps)` commits, grouped minor/patch updates and separate majors. Cargo
  directories explicitly cover `/`, `/irig106-tmats-cli` and `/fuzz` (the latter
  declares its own workspace). No dependency upgrade or automatic merge is made.

## Direct-action inventory and upstream verification

GitHub's upstream tag APIs resolved every unique repository/version pair below
(including annotated tag peeling). Every direct `uses` across all three workflows
has a full commit SHA and matching version comment. Existing CI keeps checkout
v6; the exact pilot templates use checkout v7.0.1.

| Repository | Version comment | Verified commit |
| --- | --- | --- |
| actions/checkout | v6 | `d23441a48e516b6c34aea4fa41551a30e30af803` |
| actions/checkout | v7.0.1 | `3d3c42e5aac5ba805825da76410c181273ba90b1` |
| dtolnay/rust-toolchain | v1 | `683b588c487a69d01f6c6ddf40f036ff682cc2c8` |
| actions/setup-python | v6 | `ece7cb06caefa5fff74198d8649806c4678c61a1` |
| EmbarkStudios/cargo-deny-action | v2 | `3c6349835b2b7b196a839186cb8b78e02f7b5f25` |
| ossf/scorecard-action | v2.5.0 | `fa7117db83c52d787ae5a89f350999701505d78e` |
| actions/upload-artifact | v7.0.2 | `cf430e030ddbb5b0abf93d22962f4752f3646cd9` |
| github/codeql-action | v4.39.3 | `33d7b85da1a04171e211befe2c739303b9a4c066` |
| zizmorcore/zizmor-action | v0.8.1 | `45d7276c9ca790aabd705e31ca99c6185c3e76dd` |

## Findings and validation

The initial offline workflow audit found persistent checkout credentials, missing
permission defaults and unpinned actions. The changes address these directly.
There are **no inline ignores**, repository audit suppressions, severity filters
or persona changes in this PR.

Executed locally on the aarch64 host:

- actionlint 1.7.12 passes all three workflows. Optional shellcheck/pyflakes
  integrations were disabled because those tools are not installed.
- zizmor 1.32.0 passes offline and online regular-persona audits of all three
  workflows with zero unsuppressed findings. Online auditing used the existing
  CLI session without displaying credentials. The tool reports eight findings
  omitted by its built-in regular-persona behavior, not by repository ignores.
- A supplemental explicit Dependabot audit parses the configuration but reports
  two `dependabot-cooldown` advisories (one per ecosystem entry). The reviewed
  pilot has no cooldown, and this rollout preserves that behavior without ignores.
  This supplemental result is not claimed as clean; the assigned zizmor CI job
  audits workflows, exactly as in the pilot.
- All direct-action pin/tag identities and the pinned Rust, Scorecard and zizmor
  input contracts were checked through upstream APIs. Template identity, complete
  Cargo-manifest coverage, scope, LF and `git diff --check` were checked separately.

These are static workflow checks, not a Rust build/test or hosted-CI PASS claim.
Normal PR checks must evaluate the exact review head. Scorecard runs only after
an authorized merge/main push or schedule: its first score, OIDC issuance, SARIF
upload and badge value remain unverified here. Nothing is merged or published by
this assignment; freeze after reporting the PR and head.
