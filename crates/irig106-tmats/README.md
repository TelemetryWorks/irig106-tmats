# irig106-tmats

Library scaffold for the documentation-first TMATS redesign. The only current
API is the workspace version constant. Parser, writer, registry, time-view and
checksum APIs are not implemented. The retired implementation is available at
[prototype-0](https://github.com/TelemetryWorks/irig106-tmats/tree/prototype-0).

The `std` feature is default-on and required; disabling it is unsupported,
not a no_std promise. The library and CLI retain version 0.1.0 in lockstep.
See the [release contract](https://github.com/TelemetryWorks/irig106-tmats/blob/main/docs/RELEASE-0.1.md).
