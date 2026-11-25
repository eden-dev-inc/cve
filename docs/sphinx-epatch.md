# Publish a signed .epatch with the Eden CLI

The Registry release includes a signed file manifest, the exact Runtime TAR,
the signed Lean proof envelope, and the build inputs used to reproduce the
binary. The build inputs include Rust and Lean source, dependency and toolchain
pins, and the build and replay instructions.

The [migration inventory](epatch-migration.csv) covers all 300 advisory
directories at [the pinned source revision](https://github.com/eden-dev-inc/cve/tree/62797e6726bdf2deeb73c74a8d9e3abb21c74e27/cve).
A row stays pending until its public source and proof inputs are qualified and
the signed Registry release can be downloaded and verified. The Registry
version UUID identifies that immutable release. Package or source changes
require a new version and matching qualification.

## Prepare release inputs

Use a qualified Linux Runtime TAR and its signed proof. The checked-out public
source revision must match the Rust implementation and build inputs covered by
the proof. Replay Lean and rebuild the exact binary in the recorded environment.
Review the evidence and source for distribution before sending the canonical
bundle statement to the approved proof signer.

Install the Eden CLI on Linux and approve the publishing terminal in a browser.
Select your authorized workspace. Use the publisher context to check the
namespace and audience before signing.

```sh
curl -fsSL https://eden.dev/eden/install.sh | sh
eden --version
eden patch login
eden patch whoami --public
eden patch bundle prepare runtime.tar \
  --proof proof.json --build-inputs build-inputs --output epatch-inputs
```

Obtain the detached signature over `epatch-inputs/bundle-statement.json` from the
approved proof signer. Obtain the proof signer's public key from its trusted
distribution channel. Seal and verify the exact inputs before assembling the
bundle.

```sh
eden patch bundle seal --statement epatch-inputs/bundle-statement.json \
  --signature bundle-signature.der --proof-public-key proof-public-key.der \
  --output bundle.json

eden patch bundle pack runtime.tar --proof proof.json \
  --build-inputs epatch-inputs/build-inputs.tar --bundle-signature bundle.json \
  --proof-public-key proof-public-key.der --output release.epatch
```

## Sign and publish the release

Use the enrolled publisher key UUID and the owner-only local key file. The
source archive must contain the corresponding release source. Set
`SOURCE_COMMIT_URL` to the full public GitHub commit URL and `PACKAGE_NAME` to
the name in the Runtime manifest. Use the Registry software and advisory IDs
for the verified release coverage in `targets.json`.

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

Record the immutable version UUID, organization attribution, and returned
digest. Download the same version and compare it to the verified local bundle.
Update the inventory only after the comparison succeeds.

```sh
eden patch pull "$REGISTRY_VERSION_UUID" --bundle \
  --proof-public-key proof-public-key.der --output downloaded.epatch
cmp release.epatch downloaded.epatch
```
