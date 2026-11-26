# Publish a signed .epatch with the Eden CLI

A Registry release contains the original Runtime TAR, a signed Lean proof,
the patch's source and reproducible build inputs, and a signed manifest that
binds the exact bytes. The patch-only profile uses a separately distributed
compiled ABI SDK. Replay needs only the patch bundle and the trusted kit.
Eden implementation source and protected application source stay private.

## 1. Select a patch and verify its inputs

Choose an advisory from the [migration inventory](epatch-migration.csv). Each
row identifies the original source directory by Git commit and tree SHA. Record
the new release source commit separately after reviewing the migrated package.
Identify the exact software and advisory records for `targets.json`.

Use a Runtime TAR already qualified for the exact V1 ABI and ELF, its signed
`proof.json`, and the corresponding Lean solution export. The signed member
inventory covers the raw `proof/solution-leanexport-v3.1.0.ndjson` or the
compressed pair `proof/solution-leanexport-v3.1.0.ndjson.gz` and
`proof/solution-leanexport-v3.1.0.gzip.json`. It also covers
`proof/abi-sdk-v1.sha256` in `build-inputs`. The SDK pin contains exactly 64
lowercase hexadecimal characters followed by one line feed. Use the digest
from the independent kit for the release's ABI and theorem profile. Use the
canonical build-input layout supplied with the qualification handoff, with
only redistributable patch sources, Lean inputs and build files.

The Runtime manifest, proof and bundle statement must agree on the release
version and `org/<organization UUID>` namespace. For this public catalogue the
qualified manifest sets `visibility` to `public` and `tenant_id` to null.
The CLI's `--public` flag confirms that audience at publication.

Install Eden CLI 2.7.0 or later on Linux x86-64 GNU for patch-only replay
with compressed proof exports. Replay needs Python 3.12 or later and a root-owned
`/usr/bin/bwrap` with unprivileged user and network namespaces and
`--disable-userns` support. Obtain the approved verifier kit and the P-256
proof public key in SPKI DER from your release authority. Verify the kit
against its independently distributed SHA-256. The trusted kit supplies
`replay-kit/trust.json`, the pinned compiler and ABI SDK, and the approved
theorem reference. [CLI release notes](https://eden.dev/changelog/cli)

```sh
curl -fsSL https://eden.dev/eden/install.sh | sh
eden --version
```

## 2. Assemble and replay the release

Use the exact `build-inputs.tar` and signed `bundle.json` supplied by the
proof authority. If you received a build-input directory, use the CLI to
prepare the TAR and send the resulting canonical statement to the approved
proof signer. If the signer returns a detached DER signature, seal it with
the trusted public key.

```sh
eden patch bundle prepare runtime.tar \
  --proof proof.json --build-inputs build-inputs --output epatch-inputs
eden patch bundle seal --statement epatch-inputs/bundle-statement.json \
  --signature bundle-signature.der --proof-public-key proof-public-key.der \
  --output bundle.json
```

Use the supplied or prepared TAR at `epatch-inputs/build-inputs.tar`.
Pack the exact inputs as `release.epatch` and replay it offline. Use new paths
for the generated outputs, including the preparation and verification
directories.

```sh
eden patch bundle pack runtime.tar --proof proof.json \
  --build-inputs epatch-inputs/build-inputs.tar --bundle-signature bundle.json \
  --proof-public-key proof-public-key.der --output release.epatch
eden patch bundle replay release.epatch \
  --proof-public-key proof-public-key.der \
  --trust replay-kit/trust.json --output verified-local
```

Replay checks the authorized theorem and rebuilds the exact ELF. The proof
covers the patch's stated behavior within the declared ABI and model. Sphinx
applies its own Runtime admission checks to the original Runtime TAR.

## 3. Publish as Eden Development Inc.

Approve the terminal in your browser and select **Eden Development Inc.**
Confirm the expected organization UUID and namespace. You can reuse this
browser-approved session for additional qualified releases while it is active.
Use the enrolled Ed25519 publisher key UUID and an owner-only local key file.

```sh
eden patch login
eden patch whoami --public
```

Set `SOURCE_COMMIT_URL` to the full GitHub commit URL for the exact
redistributable patch source. Set `PACKAGE_NAME` to the Runtime manifest's
name. Use the approved public archive as `publisher-source.tar`. Its SHA-256
and the source commit must match `proof/source-closure.json` in the build
inputs. Create the detached publisher statement with the CLI if a matching
statement was not supplied.

```sh
eden patch sign runtime.tar --source publisher-source.tar \
  --key-id "$PUBLISHER_KEY_ID" --key "$PUBLISHER_KEY_FILE" \
  --source-commit "$SOURCE_COMMIT_URL" --output publisher-statement.json
eden patch publish runtime.tar --package "$PACKAGE_NAME" --public \
  --targets targets.json --source-commit "$SOURCE_COMMIT_URL" \
  --proof proof.json --build-inputs epatch-inputs/build-inputs.tar \
  --bundle-signature bundle.json --publisher-statement publisher-statement.json \
  --publisher-source publisher-source.tar
```

For an interrupted upload, retry with the same inputs and arguments. A change
to a published package or its source needs a new version and matching
qualification.

## 4. Verify the Registry copy and record the release

Set `REGISTRY_VERSION_UUID` to `release.version_id` in the publication response.
The CLI downloads the exact signed bundle, checks its proof and rebuilds the
ELF in one operation.

```sh
eden patch pull "$REGISTRY_VERSION_UUID" --bundle \
  --proof-public-key proof-public-key.der \
  --verify replay-kit/trust.json --output verified-registry
cmp release.epatch verified-registry/release.epatch
```

Check the publication response's organization attribution and countersignature
separately from the CLI replay receipt. Resolve the countersignature's
`key_id` against the independently trusted P-256 organization key. Verify
the DER ECDSA signature over the SHA-256 of its canonical JSON statement.
Compare its release identity and digests against the original Runtime TAR
and the detached publisher statement and source archive. Record the exact
version, publisher namespace and digests of the Runtime TAR, compiled ELF,
proof, build inputs, compiled ABI SDK, whole bundle and
`verified-registry/receipt.json` in the inventory. Set the row to `published`
after verifying the organization receipt and completing the exact Registry
readback and replay.
Use `blocked` for a known missing input and record it in the qualification
note. `pending` indicates that release qualification is still open.
