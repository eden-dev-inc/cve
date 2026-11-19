# Ruby2Shell Case Conversion Overflow Guard deployment

Use registry package `ruby2shell-case-conversion-overflow-guard` and select a version separately. The Rust library target is `ruby2shell_case_conversion_overflow_guard`. Verify the digest of the exact archive you import.

This example targets the Ruby fixture described in the [case study](../../../docs/case-studies/GHSA_34m4_m9fh_gqp5.md).
Evidence status is **Affected product observed (ruby 3.2.10 EOL, 3.3.13, 3.4.12 and 4.0.9 — the one-byte out-of-bounds write reproduced on the real heap; fixed 3.3.14, 3.4.13 and 4.0.10 clean)**. See [deployment prerequisites](../../../docs/deployment.md)
for SDK, build, and native authority requirements.

## Declarative policy

Set `INTERLAY` to an existing HTTP interlay UUID in your authenticated Eden
CLI context. Configure the interlay's upstream for your application. Extract
the example declaration and review its route and request constraints.

```sh
jq -e '.tier1' deployment.json > remediation.json
```

To apply it, place the declaration in `settings.remediation` in a reviewed
`interlay-update.json`. Preserve the interlay's other settings.

```sh
eden api request PATCH "/api/v1/interlays/$INTERLAY" \
  --body-file interlay-update.json --content-type application/json --confirm
```

## Import a native package

Build the canonical Eden patch archive with a compatible SDK. Set
`PATCH_ARCHIVE` to that archive and `PACKAGE_SHA256` to its expected archive
digest from the package producer. Use the application product and version
observed on the host when preparing its deployment configuration.

```sh
printf '%s  %s\n' "$PACKAGE_SHA256" "$PATCH_ARCHIVE" | sha256sum --check &&
  eden api request PUT "/api/v1/interlays/$INTERLAY/patch-packages/$PACKAGE_SHA256" \
    --body-file "$PATCH_ARCHIVE" \
    --content-type application/vnd.eden.patch-package.v2+tar --confirm
```

Import returns an exact bound configuration and requires explicit activation.
Use that configuration unchanged when inspecting and signing the candidate.
The independent host principal approves the inspected bindings. Then the
publisher signs the typed statement, admits and activates the operation, and
reads its receipt. The endpoint for each prepared authority action is:

```sh
eden api request POST "/api/v1/interlays/$INTERLAY/native-authority" \
  --body-file authority-action.json --content-type application/json --confirm
```

Verify `serving_now` and acknowledgment from the expected shards. Revocation
retires the operation. A required protection can fail closed until restored.

## Comparison

The comparison below records the fixture expectations from the source bundle.
Check the case study for observed results, known bypasses, and failed imports.

| Probe | Request | Baseline (no remediation) | Tier 1 | Tier 2 (module) |
| --- | --- | --- | --- | --- |
| benign_ascii | `GET /convert?q=hello+world&enc=ISO-8859-1` | 200, no effect | 403 | 200, no effect |
| benign_utf8_sharp | `GET /convert?q=%C3%9F…&enc=UTF-8` (valid UTF-8 sharp s) | 200, no effect | 403 | 200, no effect |
| benign_below_threshold | `GET /convert?q=%DF…&enc=ISO-8859-1` (20 sharp-s bytes) | 200, no effect | 403 | 200, no effect |
| attack_query_pct | `GET /convert?q=%DF…&enc=ISO-8859-1` (21 sharp-s bytes) | 200, **real heap corruption (oob byte 0x53 at the case-map allocation boundary)** | 403 | 403, refused |
| attack_body_raw | `POST /convert` `q=<0xDF>×21&enc=ISO-8859-1` | 200, **real heap corruption** | 403 | 403, refused |
| attack_body_len28 | `POST /convert` `q=aaaaaaa<0xDF>×21&enc=ISO-8859-1` | 200, **real heap corruption** | 403 | 403, refused |
| attack_body_pct | `POST /convert` `q=%DF…&enc=ISO-8859-1` | 200, **real heap corruption** | 403 | 403, refused |
| attack_header_value | `GET /convert?enc=ISO-8859-1` + `X-Gabriel-Query: <0xDF>×21` | 200; **neutralized in flight** (see case study) | 403 | 403, refused |

After revocation the interlay failed closed (503) rather than re-exposing the conversion route, and a replacement operation restored the refusal.
