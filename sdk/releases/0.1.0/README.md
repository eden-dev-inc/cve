# Eden Patch SDK 0.1.0

Build native HTTP Proof V1 patches for Eden Sphinx on Linux x86-64 GNU. The SDK includes the Apache-2.0 license, a Rust 1.93.1 toolchain pin, and two benign examples. Local authoring and proof checking are available without an Eden account.

## Set up the SDK

[Download the source archive](eden-epatch-sdk-0.1.0.tar.gz) and its [SHA-256 checksum](eden-epatch-sdk-0.1.0.tar.gz.sha256) into the same directory:

```sh
sha256sum --check eden-epatch-sdk-0.1.0.tar.gz.sha256
tar -xzf eden-epatch-sdk-0.1.0.tar.gz
cd eden-epatch-sdk-0.1.0
cargo check --offline
```

Follow `examples/continue-when-complete/README.md` inside the archive to compile a module and check its Lean policy model. The qualified method-and-path example reproduces the pinned SDK library and the exact ELF used for a benign Sphinx admission check:

```sh
./build-pinned-sdk.sh --qualified-example
```

That rebuild uses Bubblewrap, Linux user namespaces, and a GNU C linker.

## Prepare a package with the Eden CLI

Install or update the [Eden CLI](https://eden.dev/eden/install.sh). CLI 2.8.0 uses a local SDK checkout for compilation and packages the compiled output. For a native module project, put the schema-2 `epatch.toml` and `README.md` at the project root and the compiled artifact at `patches/patch.bin`. From the project directory, run:

```sh
eden patch build . --output runtime.tar
```

For Proof V1, package preparation requires a qualified build and matching signed proof. Given the proof envelope and the directory of exact build and replay inputs, prepare the files for bundle signing:

```sh
eden patch bundle prepare runtime.tar --proof proof.json --build-inputs build-inputs --output prepared
```

For Registry publication, run `eden patch login` and `eden patch whoami` to confirm the intended publisher. Use `eden patch publish --help` for the existing signing and upload inputs. The supported proof profile remains subject to Sphinx approval for admission.

## Verify a signed release

Use the proof authority's independently obtained public key to verify the downloaded `.epatch`:

```sh
eden patch bundle verify release.epatch --proof-public-key proof-authority.der
```

Use `eden patch bundle replay --help` for the separate Lean theorem check and exact binary rebuild.
