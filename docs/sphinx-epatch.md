# Publish a signed .epatch with the Eden CLI

The Registry release includes a signed file manifest, the exact Runtime TAR,
the signed Lean proof envelope, and the build inputs used to reproduce the
binary. The build inputs include Rust and Lean source, dependency and toolchain
pins, and the build and replay instructions.

The [migration inventory](epatch-migration.csv) covers all 300 advisory
directories at [the pinned source revision](https://github.com/eden-dev-inc/cve/tree/62797e6726bdf2deeb73c74a8d9e3abb21c74e27/cve).
A row stays pending until its public source and proof inputs are qualified and
the signed Registry release can be downloaded and verified. Record the public
source commit, package version, Registry version UUID, and SHA-256 values for
the Runtime TAR, proof, and .epatch. Use the qualification note to record
unresolved identity or coverage gaps. Package or source changes require a new
version and matching qualification.

## Set up a publishing session

Install the Eden CLI on Linux and sign in. Approve the terminal in your browser,
select the publishing workspace, and check the publisher and namespace before
publishing a batch. Reuse the session while it remains active. Enroll the
workspace's Ed25519 publisher key before its first release, and obtain the
proof issuer's public key through its trusted distribution channel.

```sh
curl -fsSL https://eden.dev/eden/install.sh | sh
eden patch login
eden patch whoami --public
```

## Prepare the signed inputs for a release

Use a qualified Runtime TAR, its signed Lean proof, and the matching build
inputs cleared for public distribution. Check that the public commit contains
the source covered by the proof. The Runtime qualification workflow checks the
behavior of the release candidate. The CLI verifies signatures and artifact
identities during packaging and download.

If the issuer provides the prepared `build-inputs.tar` and signed `bundle.json`,
use those exact files. Otherwise prepare the inputs and send
`epatch-inputs/bundle-statement.json` to the approved proof signer.

```sh
eden patch bundle prepare runtime.tar \
  --proof proof.json --build-inputs build-inputs --output epatch-inputs
```

Use the issuer's signed `bundle.json` when available. If the issuer sends a DER
signature, seal it into the envelope with the trusted public key.

```sh
eden patch bundle seal --statement epatch-inputs/bundle-statement.json \
  --signature bundle-signature.der --proof-public-key proof-public-key.der \
  --output bundle.json
```

Keep the verified `release.epatch` supplied with the qualified release for the
download comparison. If you have the signed inputs and need a local copy,
assemble it with the CLI.

```sh
eden patch bundle pack runtime.tar --proof proof.json \
  --build-inputs epatch-inputs/build-inputs.tar --bundle-signature bundle.json \
  --proof-public-key proof-public-key.der --output release.epatch
```

## Sign, publish, and read back

Set `SOURCE_COMMIT_URL` to the full public GitHub commit URL and `PACKAGE_NAME`
to the Runtime manifest's name. Put the verified Registry software and
advisory IDs in `targets.json`. Use the enrolled publisher key UUID and the
owner-only local key file. Use the same prepared build-input archive as the
publisher source when it contains the reviewed source for this release.

```sh
eden patch sign runtime.tar --source epatch-inputs/build-inputs.tar \
  --key-id "$PUBLISHER_KEY_ID" --key "$PUBLISHER_KEY_FILE" \
  --source-commit "$SOURCE_COMMIT_URL" --output publisher-statement.json
eden patch publish runtime.tar --package "$PACKAGE_NAME" --public \
  --targets targets.json --source-commit "$SOURCE_COMMIT_URL" \
  --proof proof.json --build-inputs epatch-inputs/build-inputs.tar \
  --bundle-signature bundle.json --publisher-statement publisher-statement.json \
  --publisher-source epatch-inputs/build-inputs.tar
```

If the release already has a matching publisher statement, use it for the
publish command. To retry an interrupted upload, rerun the same publish
command with the original files and arguments.

Use the returned Registry version UUID for the pull. Compare the exact
download to the verified local bundle. For organization releases, verify the
returned attribution and countersignature using the organization's trusted
public key. Record the exact version, organization attribution and digests in
the inventory after those checks.

```sh
eden patch pull "$REGISTRY_VERSION_UUID" --bundle \
  --proof-public-key proof-public-key.der --output downloaded.epatch
cmp release.epatch downloaded.epatch
```
