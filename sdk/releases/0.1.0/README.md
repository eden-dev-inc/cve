# Eden Patch SDK 0.1.0

Download [the source archive](eden-epatch-sdk-0.1.0.tar.gz) and its [SHA-256 checksum](eden-epatch-sdk-0.1.0.tar.gz.sha256).
The archive includes the Apache-2.0 license, pinned Rust toolchain, README,
native HTTP Proof V1 ABI source, and two benign authoring examples for
Linux x86-64 GNU.

```sh
sha256sum --check eden-epatch-sdk-0.1.0.tar.gz.sha256
tar -xzf eden-epatch-sdk-0.1.0.tar.gz
cd eden-epatch-sdk-0.1.0
cargo check --offline
```

The archive README explains how to build an example and check its Lean model.
The qualified example reproduces the SDK and an exact benign module.
Registry publication uses Eden CLI publisher sign-in and signing.
