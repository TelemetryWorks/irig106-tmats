# irig106-tmats-cli

Reusable help/version scaffold and thin `tmats` executable in the virtual
workspace. TMATS commands (`show`, `extract`, `checksum`, `verify`, `stamp`)
are planned, not implemented. The library runner accepts caller-owned args
and output sinks and never exits the host. Current command logic is limited
to the existing help/version/usage paths.

```bash
cargo run -p irig106-tmats-cli -- --version
```

The app inherits its exact library dependency from the root workspace and
shares version 0.1.0. This restructure does not publish either package.
See the [CLI contract](https://github.com/TelemetryWorks/irig106-tmats/blob/main/docs/CLI.md).
