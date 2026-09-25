# irig106-tmats-cli

`tmats` — view, extract, and check **TMATS** (IRIG 106 Chapter 9) from
Chapter 10 recordings and standalone TMATS files. It is the Rust successor to
irig106.org's `idmptmat`, built on the
[`irig106-tmats`](https://crates.io/crates/irig106-tmats) library and always
released at the same version as it.

**Status: under development.** The planned commands — `show` (raw, tree, and
channel-summary views), `extract`, `checksum` (`G\SHA` and the irig106.org
flex signature), `verify`, `stamp`, and later `validate` and `diff` — are
described in the
[design documents](https://github.com/TelemetryWorks/irig106-tmats/tree/main/docs).

```bash
cargo install irig106-tmats-cli   # installs the `tmats` command
tmats --version
```

For the complete IRIG 106 command-line tool, see `irig106-cli`.
