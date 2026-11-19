# Ruby2Shell Case Conversion Overflow Guard

**Affected product observed — the one-byte out-of-bounds write reproduced on the real heap of every affected Ruby series, and the trigger refused at the boundary.** See the [case study](../../docs/case-studies/GHSA_34m4_m9fh_gqp5.md) for the observed results and limits.

## Advisory

- **GHSA:** GHSA-34m4-m9fh-gqp5 ([verialabs.com](https://verialabs.com/ruby2shell/) · [GitHub](https://github.com/ruby/ruby/security/advisories/GHSA-34m4-m9fh-gqp5); **CVE requested, not yet assigned**)
- **Affected component:** Ruby string case conversion (`rb_str_casemap` → the single-byte `case_map` in ISO-8859-x and Windows-125x encodings)
- **Affected versions:** ruby 3.3.13 and earlier, 3.4.0–3.4.12, 4.0.0–4.0.9; 3.2 and older EOL (back to 2.4.0)
- **Fixed versions:** 3.3.14, 3.4.13, 4.0.10 ([ruby/ruby@d582f8f](https://github.com/ruby/ruby/commit/d582f8f13d700d8e39714d3a8f11eb4bd2bc69f0))
- **Fixture component:** gabriel-r2shell-app (Ruby 3.4.12 WEBrick fixture, fork-isolated heap audit)
- **Summary:** refuse request parts carrying 21+ sharp-s (`0xDF`) bytes outside valid UTF-8 — the case-conversion overflow shape

## Patch contents

| File | Purpose |
| --- | --- |
| `http-request-filter/Cargo.toml` | module name + SemVer |
| `http-request-filter/src/lib.rs` | the `no_std` Rust remediation module |
| `http-request-filter/patch.toml` | Eden compatibility, SDK commit, build profile |
| `http-request-filter/deployment.json` | portable interlay configuration |
| `http-request-filter/api.md` | copy-paste import, activation, verification |

## Module identity

- Registry package: `ruby2shell-case-conversion-overflow-guard`
- Rust library: `ruby2shell_case_conversion_overflow_guard`
- Registry version: `1.0.0`
- Current source SHA-256: `d74ff3358cd70a35df8a12b2dbd59cda693428c083203d032082534913c9ec0f`
- Original source snapshot: [Rust source](https://github.com/eden-dev-inc/cve/blob/main/cve/GHSA-34m4-m9fh-gqp5/http-request-filter/src/lib.rs) and [build manifest](https://github.com/eden-dev-inc/cve/blob/main/cve/GHSA-34m4-m9fh-gqp5/http-request-filter/patch.toml)
- Recorded test binary SHA-256: `92cd8a6da164943118566a741d586da3c35477bddd22f2b532d10f678fd05c87`
- Recorded test operation: `96cb58fd-fd09-45ad-8417-fd7bc1f1fac0`
- Test date: 2026-11-19 UTC; first publicly published 2026-11-19T07:30:00Z

## Recorded lab evidence (2026-11-19 UTC)

- Deployment: Eden Sphinx 0.9.0 (engine release `eden-product-release:1.103.0-single`, image `sha256:809293e1f0b525ac04698d1fa8d7ffdc66cbee1ff690b082ba26b4d050f08416`)

| Arm | n | mean | p50 | p95 |
| --- | --- | --- | --- | --- |
| Tier 1 — declarative policy, blocked (403) | 15 | 0.4 ms | 0.4 ms | 0.5 ms |
| Tier 2 — module, benign passes (200) | 20 | 13.7 ms | 10.8 ms | 21.4 ms |
| Tier 2 — module, attack refused | 15 | 0.5 ms | 0.5 ms | 0.6 ms |
| Baseline — unprotected interlay, benign (200) | 20 | 53.0 ms | 53.0 ms | 68.0 ms |

The baseline arm measures the fixture's fork-isolated audit oracle (a forked audit child per request), not a like-for-like module-overhead comparison.

## Fixture comparison

| Probe | Baseline | Tier 1 | Tier 2 (module) |
| --- | --- | --- | --- |
| benign_ascii | 200, no effect | 403 | 200, no effect |
| benign_utf8_sharp | 200, no effect | 403 | 200, no effect |
| benign_below_threshold (20 sharp-s) | 200, no effect | 403 | 200, no effect |
| attack_query_pct (21 sharp-s) | 200, **real heap corruption (oob=83 = 0x53)** | 403 | 403, refused |
| attack_body_raw | 200, **real heap corruption** | 403 | 403, refused |
| attack_body_len28 | 200, **real heap corruption** | 403 | 403, refused |
| attack_body_pct | 200, **real heap corruption** | 403 | 403, refused |
| attack_header_value | 200; neutralized in flight (see case study) | 403 | 403, refused |

## Limits

Finite lab route; the framework leg (Sinatra classic on puma and WEBrick) is an honest negative — see the case study. The threshold rule also refuses binary-encoded values that Ruby would not expand. Sibling advisories in the same Ruby release (GHSA-87fm case-conversion integer overflow; five Regexp-engine GHSAs) are not covered by this module — see the annex. No production claim.
