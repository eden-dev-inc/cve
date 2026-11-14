# Implement, test and apply a CVE patch with Eden Sphinx

Eden Sphinx can refuse or transform supported traffic before it reaches an
affected application. A virtual patch applies to the traffic and software in
its declared scope. Follow the vendor's instructions to fix the vulnerable
component itself.

This walkthrough uses the [Log4j JNDI Lookup Guard](../cve/CVE-2021-44228/README.md).
Log4j can evaluate attacker-controlled text from a log message and make an
outbound lookup. The example targets one lab route, `POST /api/log`. Its
structural policy refuses requests with a root JSON `message` member. The
native example preserves the request while redacting the literal JNDI marker.
Other input paths and encodings need separate coverage. Use the
[case study](case-studies/CVE_2021_44228.md) and
[Apache advisory](https://logging.apache.org/security.html) to check the
demonstrated scope and vendor fix.

## 1. Write the protection

1. Pin the affected application version in an isolated environment. Record the
   expected backend effect of an advisory-shaped request, then verify that the
   effect occurs without the patch. Use a harmless marker and a backend or
   fixture you control. Label synthetic fixture results in the case study.
2. Scope the rule to the affected application, HTTP methods, routes and fields.
   Use a declarative rule when a supported structural constraint suffices.
   The Log4j policy blocks legitimate requests containing `message` too.
   Record that restriction before applying it.
3. Use a native module when the rule needs bounded byte inspection or an edit.
   Open the [Log4j Rust module](../cve/CVE-2021-44228/http-request-filter/src/lib.rs).
   For HTTP incremental v3, implement `RemediationModule<Request<'_>>` using
   the SDK's `native_http_v3` input and exported ABI table. Return `NeedMore`
   while incomplete bytes can change the decision. Return `Continue` for an
   out-of-scope or conclusive benign match. Return `Refuse` to block, or
   submit bounded edits to original-frame offsets and return `Edit` when the
   host permits those edits. Keep capability declarations and the runtime
   profile aligned with the behavior.
4. Update [the deployment declaration](../cve/CVE-2021-44228/http-request-filter/deployment.json)
   and [API example](../cve/CVE-2021-44228/http-request-filter/api.md) along
   with the implementation. Use the real upstream and observed application
   product/version in your staging setup.

## 2. Check, build and package

Start from the root of the fork.

```sh
python3 scripts/check-bundles.py CVE-2021-44228
```

This validates the public bundle metadata and links. Native compilation
requires the matching Eden SDK or build environment. The public repository
does not include that SDK. Check the
[bundle's build metadata](../cve/CVE-2021-44228/http-request-filter/patch.toml)
for the SDK commit, Rust version, Linux target and flags. If you only have a
public checkout, include your probes and request native build and staging
verification in the PR.

For this example's closed native HTTP v3 profile, run the following on a Linux
x86-64 build host with Rust 1.93.1 installed. Set `EDEN_SDK_SOURCE` to the
matching authorized Eden source checkout and `BUNDLE` to the bundle directory
in your clone. Use the metadata in the bundle you are actually building.

```sh
EDEN_SDK_SOURCE=/absolute/path/to/matching/eden
BUNDLE="$PWD/cve/CVE-2021-44228/http-request-filter"
BUILD_DIR="$(mktemp -d)"

rustc +1.93.1 --edition=2024 --target x86_64-unknown-linux-gnu \
  --crate-type=rlib --crate-name=eden_gateway_remediation \
  -C opt-level=3 -C panic=abort \
  "$EDEN_SDK_SOURCE/eden-remediation/src/lib.rs" \
  -o "$BUILD_DIR/libeden_gateway_remediation.rlib"

rustc +1.93.1 --edition=2024 --target x86_64-unknown-linux-gnu \
  --crate-type=cdylib --crate-name=log4j_jndi_lookup_guard \
  -C opt-level=3 -C panic=abort -C strip=symbols -C lto=fat \
  -C codegen-units=1 -C force-unwind-tables=no -C link-arg=-nostartfiles \
  --extern "eden_gateway_remediation=$BUILD_DIR/libeden_gateway_remediation.rlib" \
  "$BUNDLE/src/lib.rs" -o "$BUILD_DIR/module.so"

sha256sum "$BUNDLE/src/lib.rs" "$BUILD_DIR/module.so"
```

Use the matching SDK's `NativeArtifactManifest` and `PatchPackageManifest`
to produce `manifest.json` in `BUILD_DIR`. Bind the actual source and binary,
native ABI and capabilities, application applicability, publisher, separate
package version, and the reviewed deployment declaration. The bundle's
`patch.toml` describes the source build. The generated schema-2 manifest
describes the release archive. Create a release `README.md` in `BUILD_DIR`
with scope, limitations and probe outcomes. For the native package, run:

```sh
cargo run --locked --manifest-path "$EDEN_SDK_SOURCE/Cargo.toml" \
  -p eden-gateway-remediation --features native-modules --example package -- \
  "$BUILD_DIR/manifest.json" "$BUILD_DIR/README.md" \
  "$BUILD_DIR/patch.tar" "$BUILD_DIR/module.so"
```

For a `native_policy` package, use the corresponding declarative manifest
and omit the module binary argument. Retain the producer's whole-archive SHA-256
separately from the source and module SHA-256. If the registry is available in
your Eden environment, a released archive can also be downloaded by exact
version. Verify its digest before import. See the
[deployment prerequisites](deployment.md) and
[Eden remediation documentation](https://eden.dev/docs/remediation).

## 3. Apply it on an authorized staging interlay

Pick an existing interlay on a compatible Eden deployment with the relevant
remediation profile enabled. Use an authenticated Eden CLI context and confirm
the installed API at `/api-docs/openapi.json`.

- **Declarative policy.** Save the prior interlay configuration. Copy the
  reviewed Tier 1 declaration to `settings.remediation` in the interlay update.
  Preserve the other settings and PATCH the configuration with the
  [bundle's API example](../cve/CVE-2021-44228/http-request-filter/api.md).
  Read back the effective policy and verify its scope.
- **Native module.** Set `INTERLAY` to the staging interlay UUID, `PATCH_ARCHIVE`
  to the exact tar file, and `PACKAGE_SHA256` to its expected archive digest.
  Verify and import it:

```sh
printf '%s  %s\n' "$PACKAGE_SHA256" "$PATCH_ARCHIVE" | sha256sum --check &&
  eden api request PUT "/api/v1/interlays/$INTERLAY/patch-packages/$PACKAGE_SHA256" \
    --body-file "$PATCH_ARCHIVE" \
    --content-type application/vnd.eden.patch-package.v2+tar --confirm
```

Import stores the package and returns the exact bound configuration. Native
activation also needs an enrolled publisher key and inventory attested by
an independent host principal. Use the unchanged imported configuration
through `POST /api/v1/interlays/{interlay}/native-authority` and the matching
SDK types for approval and signing.

| Order | Authority action | Expected result |
| --- | --- | --- |
| 1 | Publisher inspects the imported configuration | Actual scope, package, host and software bindings |
| 2 | Independent host principal approves those bindings | Host approval ID and revision |
| 3 | Publisher signs the typed `NativeActivationStatement` and submits `admit` | Original operation UUID and prepared receipt |
| 4 | Authorized publisher submits `activate` for that operation | Activation response for the same operation |
| 5 | Read the operation's `receipt` | `serving_now` and acknowledgment from the expected shards |

Follow Eden's [API and package lifecycle](https://eden.dev/docs/remediation/api)
for action bodies and permissions. Registry publication makes an archive
available for delivery. Each deployment has its own activation and verification.

## 4. Prove the effect and exercise rollback

Replay the same probes against the same application before and after activation.

| Probe | Verify in staging |
| --- | --- |
| Ordinary request on the protected route | The application still performs the expected action, except for any declared restrictions. |
| Advisory-shaped request | The unprotected baseline exhibits the fixture signal. Under protection, refusal causes zero prohibited backend effects, or a correction sends only the intended changed bytes and prevents the effect. |
| Off-route request, encoding/case variants, partial input | Record observed bypasses and bounds. Check that incomplete input does not silently release protected traffic. |
| Rollback and restoration | Restore the recorded prior declarative configuration, or exercise native revoke and a fresh authorized restoration in staging. Check receipts and replay the probes. Required native protection can fail closed after revoke. |

For the Log4j fixture, both benign and corrected requests can return HTTP 200.
For a wire-only smoke test, set `STAGING_INTERLAY_URL` to the base URL of the
staging listener connected to your controlled logging fixture. Send these
requests and compare the fixture's captured bodies:

```sh
curl -sS -H 'Content-Type: application/json' \
  --data-binary '{"message":"hello from staging"}' "$STAGING_INTERLAY_URL/api/log"
curl -sS -H 'Content-Type: application/json' \
  --data-binary '{"message":"synthetic ${jndi:fixture-marker}"}' "$STAGING_INTERLAY_URL/api/log"
```

Under the native rule, the first body should be unchanged and the second
should contain `[jndi-redacted]fixture-marker}` in place of the original lookup
text. The declarative rule should refuse both requests. This smoke test checks
message handling. A vulnerability outcome needs the affected application or a
clearly labeled fixture with its own independent lookup or callback signal. Record application
and SDK versions, patch name and version, input variants, observed outcomes
and remaining limitations in the [case study template](case-studies/TEMPLATE.md).
Keep the baseline in the behavioral comparison and omit unprotected baseline
latency from the patch's latency table.

See [Contributing](../CONTRIBUTING.md) to submit the change and carry accepted
improvements into the next release.
