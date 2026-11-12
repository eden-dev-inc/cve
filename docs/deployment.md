# Build and deploy a CVE module

Each `Cargo.toml` records the module version and expects a compatible Eden SDK
source checkout in the adjacent `eden-gateway-remediation` directory. Use the
SDK revision, target, compiler version, and native build profile in
`patch.toml`. The repository contains module source and API examples. The
compatible SDK and built patch archives come from an Eden build environment.

Use the matching SDK's package producer to create the schema-2 patch archive
and record its SHA-256. Native packaging takes the actual module binary and
its generated artifact manifest. The package manifest also records the
deployment declaration and source compatibility. The API uploads the archive
bytes by their archive digest. This digest is distinct from the Rust source
and module binary digests.

For native activation, use an authenticated Eden CLI context and a compatible
host with configured independent approvers. The management API contract for
your installed build is at `/api-docs/openapi.json` on the Eden service.
Enroll the publisher key, attest the actual host software inventory, import
the package, inspect the exact returned configuration, obtain independent host
approval, sign the typed activation statement with the SDK, then admit and
activate the operation.

## Verify the result

Read the operation receipt and check `serving_now` and shard acknowledgment.
Compare behavior on the affected application version, including a reproducible
unprotected baseline, the target effect under protection, and legitimate
requests that should still pass. The case studies document the boundary reached
by each experiment. Some cases used synthetic fixtures or recorded bypasses,
failed imports, and unresolved CVE associations.
