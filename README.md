# irig106-tmats

[![OpenSSF Scorecard](https://api.scorecard.dev/projects/github.com/TelemetryWorks/irig106-tmats/badge)](https://scorecard.dev/viewer/?uri=github.com/TelemetryWorks/irig106-tmats)

**Documentation-first TMATS redesign: workspace skeleton, not a parser yet.**

The incorrect prototype and its tests, build-script registry, benchmarks and
fuzz harness have been retired. Its history remains at
[`prototype-0`](https://github.com/TelemetryWorks/irig106-tmats/tree/prototype-0).
Do not use the skeleton for compliance validation or rewriting telemetry.

The virtual workspace contains:

- [crates/irig106-tmats](crates/irig106-tmats): library skeleton, currently
  exposing only `VERSION`.
- [apps/irig106-tmats-cli](apps/irig106-tmats-cli): reusable help/version CLI
  scaffold and thin `tmats` executable. TMATS commands are not implemented.

Both packages remain **0.1.0**, in lockstep. The root manifest owns their
metadata and the exact-version internal dependency. The library's `std`
feature is default-on and required; disabling it is unsupported, not a
`no_std` promise. No new dependency or release is introduced by this layout.

```bash
cargo run -p irig106-tmats-cli -- --version
cargo test --workspace --all-features
cargo test --workspace --no-default-features --features std
python3 scripts/build-trace-matrix.py --check
```

TMATS is defined by IRIG 106 Chapter 9. The planned lossless library preserves
raw source and uncertainty, with no silent repair. See the
[0.1 release contract](docs/RELEASE-0.1.md), [roadmap](docs/ROADMAP.md),
[architecture](docs/ARCHITECTURE.md), [CLI design](docs/CLI.md),
[requirements trace](docs/TRACE-MATRIX.md) and [project structure](docs/PROJECT_STRUCTURE.md).
Written analyses, source inspection and scaffold CI are not implemented TMATS
acceptance. Parsing, views, checksums and release workflows follow in later
families after their review gates. The documented 0.1 scope is unchanged.

The library/CLI use the [Apache-2.0 license](LICENSE).
