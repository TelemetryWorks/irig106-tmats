#!/usr/bin/env python3
"""CI-only archive checks and tests of the unpublished lockstep package pair."""

import json
from pathlib import Path
import subprocess
import tarfile
import tempfile
import tomllib


def main() -> None:
    root = Path(__file__).resolve().parent.parent
    manifest = tomllib.loads((root / "Cargo.toml").read_text())
    version = manifest["workspace"]["package"]["version"]
    license_text = (root / "LICENSE").read_bytes()
    with tempfile.TemporaryDirectory(prefix="tmats-package-tests-") as temporary:
        destination = Path(temporary)
        for name in ["irig106-tmats", "irig106-tmats-cli"]:
            package = f"{name}-{version}"
            with tarfile.open(root / "target" / "package" / f"{package}.crate") as archive:
                with archive.extractfile(f"{package}/LICENSE") as license_file:
                    assert license_file.read() == license_text, package
                archive.extractall(destination, filter="data")
        # Only this archive test patches the unpublished pair together. It is
        # not crates.io-only release evidence for third-party dependencies.
        library = destination / f"irig106-tmats-{version}"
        cli = destination / f"irig106-tmats-cli-{version}" / "Cargo.toml"
        subprocess.run(
            ["cargo", "test", "--manifest-path", str(cli), "--config",
             f"patch.crates-io.irig106-tmats.path={json.dumps(str(library))}"],
            check=True,
        )


if __name__ == "__main__":
    main()
