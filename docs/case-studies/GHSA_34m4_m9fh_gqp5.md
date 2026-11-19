# Ruby2Shell: one-byte heap overflow in Ruby case conversion (GHSA-34m4-m9fh-gqp5; CVE pending)

An unauthenticated remote caller can submit a crafted string to an affected Ruby's case conversion (`upcase`, `swapcase`, `downcase(:fold)`, `casecmp?`), write exactly one byte past a heap allocation, and — per the vendor's withheld chains — reach remote code execution in default configurations of common Ruby web frameworks. The vendor's proof of concept is pending publication; this report records what was independently observed. Upgrade Ruby to a fixed release.

[Vendor advisory](https://verialabs.com/ruby2shell/) · [GitHub advisory](https://github.com/ruby/ruby/security/advisories/GHSA-34m4-m9fh-gqp5) · [Module source](modules/GHSA-34m4-m9fh-gqp5.rs)

**Patch title.** Ruby2Shell Case Conversion Overflow Guard (`ruby2shell-case-conversion-overflow-guard` in the registry). The [original source snapshot](https://github.com/eden-dev-inc/cve/blob/main/cve/GHSA-34m4-m9fh-gqp5/http-request-filter/src/lib.rs) preserves the source available for the historical report.

| Field | Value |
| --- | --- |
| Affected component and versions | Ruby string case conversion — ruby 3.3.13 and earlier, 3.4.0–3.4.12, 4.0.0–4.0.9; 3.2 and older EOL (back to 2.4.0) |
| Fixed release | 3.3.14, 3.4.13, 4.0.10 ([ruby/ruby@d582f8f](https://github.com/ruby/ruby/commit/d582f8f13d700d8e39714d3a8f11eb4bd2bc69f0)) |
| Severity | Critical (vendor-rated; CVSS pending with the CVE) |
| Evidence status | Affected product observed (the one-byte OOB write reproduced on the real heap of every affected series, through the vendor-documented allocation shape; fixed releases observed clean) |
| Test environment | Lab fixture and affected builds documented in the evidence below |
| Eden release and module version | Sphinx 0.9.0 · module 1.0.0 |
| Tested at | 2026-11-19 UTC |
| First publicly published | 2026-11-19T07:30:00Z |

[Vendor advisory](https://verialabs.com/ruby2shell/) · [Module source](modules/GHSA-34m4-m9fh-gqp5.rs) · [Bundle](../../cve/GHSA-34m4-m9fh-gqp5/README.md)

**Result.** The crafted shape **corrupted the real Ruby heap** in the baseline — the fork-isolated audit oracle observed the byte `S` (`0x53`) written exactly one byte past the case-map allocation on every affected series, with the allocator itself aborting on integrity errors in repeated runs. The native module refused every marked request with HTTP 403 **before release** — no trigger value reached the conversion while the module served — and preserved HTTP 200 for every benign probe, including valid-UTF-8 sharp s and the below-threshold control. After revocation the interlay failed closed (503) rather than re-exposing the conversion route.

## The vulnerability

Ruby allocates the case-mapping buffer in `rb_str_casemap` (`string.c`) as `offsetof(mapping_buffer, space) + capa`, with `capa = len × n + CASE_MAPPING_ADDITIONAL_LENGTH` (`+20`, on the theory that 20 bytes absorb any single-character expansion). The single-byte `case_map` in the ISO-8859-x and Windows-125x encoding sources maps the sharp s (`0xDF`) to **two** bytes (`SS` on upcase) but the loop guard checked room for only **one** byte (`while (*pp < end && to < to_end)`). A legacy-encoded string with **21** or more `0xDF` bytes consumes the +20 headroom and writes the final `S` exactly one byte past the allocation — onto the size field of the next heap chunk (`0x53` on `upcase`/`swapcase`; `0x73` on `downcase(:fold)`/`casecmp?`, which fold to lowercase).

The bug family is confirmed against the upstream fix ([ruby/ruby@d582f8f](https://github.com/ruby/ruby/commit/d582f8f13d700d8e39714d3a8f11eb4bd2bc69f0)), which tightens the loop guard to `to_end - to > 1` in exactly fourteen encodings: ISO-8859-1, -2, -3, -4, -9, -10, -13, -14, -15, -16 and Windows-1250, -1252, -1254, -1257. UTF-8 strings take a different code path and are not affected. CVE identifiers were requested by the vendor but not assigned at publication; the advisory is canonically GHSA-34m4-m9fh-gqp5.

## Primitive verification on real affected Rubies (2026-11-19 UTC)

Each public Docker Hub Ruby image was run on the verification host with an `LD_PRELOAD` malloc tracer and a Fiddle-based audit: the tracer records the case-map allocation (`24 + (len × 1 + 20)` bytes), and the audit reads the byte at `ptr + alloc_size` — exactly one byte past the allocation — after the conversion. The trigger is `0xDF × 21` in `ISO-8859-1` (28-byte class form for the allocation boundary); controls are valid UTF-8 `ß` and `ASCII-8BIT`, plus a no-trigger groom. Verdicts are observed bytes and observed process fate, nothing else.

| Ruby image | Status | Observed at the allocation boundary | Process fate |
| --- | --- | --- | --- |
| ruby:3.2.10 | EOL, vulnerable code present | `0x53` — **OOB write observed** | completed |
| ruby:3.3.13 | affected | `0x53` — **OOB write observed** | completed |
| ruby:3.4.12 | affected | `0x53` — **OOB write observed** | several runs aborted in glibc integrity checks (`munmap_chunk(): invalid pointer`) |
| ruby:4.0.9 | affected | `0x53` — **OOB write observed** | completed |
| ruby:3.3.14 | **fixed** | next-chunk size byte intact | clean |
| ruby:3.4.13 | **fixed** | next-chunk size byte intact | clean |
| ruby:4.0.10 | **fixed** | next-chunk size byte intact | clean |

The fixed builds also show the fix working in the allocation trace: the tightened two-byte room check defers the final sharp s to a **second** mapping pass, so a second, 46-byte pass buffer appears where affected builds allocate a single 72-byte buffer for the 28-byte trigger.

The **encoding matrix** matches the upstream fix exactly: the OOB write reproduces in all fourteen patched encodings (ISO-8859-1, -2, -3, -4, -9, -10, -13, -14, -15, -16; Windows-1250, -1252, -1254, -1257) and in **no other** legacy single-byte encoding — ISO-8859-5, -6, -7, -8 and Windows-1251, -1253, -1255, -1256, -1258 do not map `0xDF` to a two-byte expansion and stay clean, as do UTF-8 and `ASCII-8BIT`. The **method matrix**: `upcase` and `swapcase` write `0x53`; `downcase(:fold)` and `casecmp?` write `0x73`.

## Framework reachability (honest negative)

The vendor states the bug is reachable in the default configuration of Rails and Sinatra (classic style), with the PoC withheld pending upgrades. This campaign traced the default conversion paths in real gem trees on the affected image:

- Sinatra **classic style** (`Sinatra::Application`) enables `Rack::MethodOverride` by default (modular `Sinatra::Base` does not — `set :method_override, true` at `base.rb:2087`), and `Rack::MethodOverride` upcases the attacker-controlled `_method` form parameter (`method.to_s.upcase`).
- `Rack::Protection` (default-on in classic) downcases the `Host` header in `HostAuthorization`.

However, in this lab's harness (default Sinatra classic under puma and under WEBrick, raw-socket request lines), every env string that reaches those conversion calls arrives as `ASCII-8BIT` or UTF-8 — encodings that do not take the expanding `case_map` path — so **the corruption was not reproduced through the frameworks** here. The vendor's PoC (due within a week) will name the actual default vector; the fixture leg below reproduces the primitive in a controlled shape instead. Recorded as an honest limit, not as proof the vendor claim is wrong.

## The Eden patch

**Tier 1 — declarative policy.** The recorded policy denies every method on the `/convert` route family and its descendants. This is a coarse rule: it also blocks the benign conversion probes on the same route (recorded false positives).

```json
{
  "version": 1,
  "profile": "http-structure-v1",
  "policy": {
    "version": 1,
    "rules": [
      {
        "scope": {
          "methods": [],
          "path": "/convert",
          "descendants": true
        },
        "require": {
          "kind": "deny"
        }
      }
    ]
  }
}
```

**Tier 2 — native Rust module.** The module streams every request part (request line and headers, then body) through two allocation-free counters: the raw byte count of `0xDF` bytes that are not valid UTF-8 lead pairs, and the same count over the percent-decoded stream. A request part carrying **21 or more** such bytes is refused (`0x53`-shaped `upcase`/`swapcase` triggers and `0x73`-shaped `fold` triggers alike). Valid UTF-8 never contains a lone `0xDF` (every `0xDF` lead is followed by a continuation byte), so UTF-8 traffic never trips the rule — verified by the `benign_utf8_sharp` probe carrying 21 valid `ß`. Both raw and percent-encoded forms are covered because the fixture decodes `%DF` to the raw byte before the conversion.

```rust
const SHARP: u8 = 0xDF;
const THRESHOLD: u32 = 21;

fn is_hex(b: u8) -> bool {
    (b >= b'0' && b <= b'9') || (b >= b'a' && b <= b'f') || (b >= b'A' && b <= b'F')
}

fn hex_val(b: u8) -> u8 {
    match b {
        b'0'..=b'9' => b - b'0',
        b'a'..=b'f' => b - b'a' + 10,
        _ => b - b'A' + 10,
    }
}

fn is_continuation(b: u8) -> bool {
    (0x80..=0xBF).contains(&b)
}

/// Count sharp-s bytes that are not valid UTF-8 lead pairs, in the raw bytes.
fn lone_sharp_raw(src: &[u8]) -> u32 {
    let mut count = 0u32;
    let mut i = 0usize;
    let n = src.len();
    while i < n {
        if src[i] == SHARP {
            if i + 1 < n && is_continuation(src[i + 1]) {
                // valid UTF-8 two-byte sequence: never a sharp s
                i += 2;
            } else {
                count += 1;
                i += 1;
            }
        } else {
            i += 1;
        }
    }
    count
}

/// Pull the next percent-decoded byte from `src`, advancing `i`. Returns
/// None at end of input. A malformed escape yields the literal bytes.
fn next_decoded(src: &[u8], i: &mut usize) -> Option<u8> {
    let n = src.len();
    if *i >= n {
        return None;
    }
    let b = src[*i];
    if b == b'%' && *i + 2 < n && is_hex(src[*i + 1]) && is_hex(src[*i + 2]) {
        let byte = (hex_val(src[*i + 1]) << 4) | hex_val(src[*i + 2]);
        *i += 3;
        Some(byte)
    } else {
        *i += 1;
        Some(b)
    }
}

/// Count sharp-s bytes that are not valid UTF-8 lead pairs, in the
/// percent-decoded stream. Runs in O(n) with one byte of lookahead.
fn lone_sharp_decoded(src: &[u8]) -> u32 {
    let mut count = 0u32;
    let mut i = 0usize;
    let mut pending: Option<u8> = None;
    loop {
        let b = match pending.take() {
            Some(b) => b,
            None => match next_decoded(src, &mut i) {
                Some(b) => b,
                None => break,
            },
        };
        if b != SHARP {
            continue;
        }
        // one decoded byte of lookahead decides lead-pair vs lone sharp s
        let next = match pending.take().or_else(|| next_decoded(src, &mut i)) {
            Some(b) => b,
            None => {
                count += 1;
                break;
            }
        };
        if is_continuation(next) {
            // valid UTF-8 pair in the decoded stream: not a sharp s
        } else {
            count += 1;
            pending = Some(next);
        }
    }
    count
}
```

(Complete module: [modules/GHSA-34m4-m9fh-gqp5.rs](modules/GHSA-34m4-m9fh-gqp5.rs).)

## Implement the remediation

1. **Prepare the target.** Confirm the Ruby version against the advisory (`ruby -v`). Use a compatible Eden HTTP interlay and an authenticated operator with Configure permission.
2. **Apply the declarative policy.** Set `INTERLAY` to the target interlay UUID. Save the interlay's current settings, place the tier-1 policy in `settings.remediation`, and PATCH the full settings with a fresh `Idempotency-Key`. Read back `remediation_status.state == ready`.
3. **Import and activate the native module.** Build the schema-2 archive from the published source with the compatible SDK, verify its digest, and PUT it to the interlay's patch-packages endpoint. Import requires explicit activation: enroll the publisher key, attest the host inventory, inspect the returned configuration, obtain independent host approval, sign the typed statement, admit, and activate. Read the receipt until `serving_now` is true and all shards are acknowledged.
4. **Verify, revoke, and restore.** Replay the benign and marked probes and compare the audit oracle. Revoke and confirm the route fails closed (503); re-approve, re-admit and reactivate a replacement operation to restore protection.

**Vendor update.** The durable fix is Ruby 3.3.14, 3.4.13 or 4.0.10. Ruby 3.2 and older are EOL and will not receive the fix.

## Fixture and probes

The fixture is a real Ruby 3.4.12 WEBrick application behind the Eden interlay, exposing `GET/POST /convert?q=<value>&enc=<encoding>` (and an `X-Gabriel-Query` header form). Every conversion runs in a fork-isolated audit child with `GC.disable` on a drained, deterministic 80-byte-alloc class heap: the audit records the malloc-traced case-map allocation, reads the byte one past it after the conversion, and returns the observation. The probe set:

| Probe | Shape | Baseline | Tier 1 | Tier 2 (module) |
| --- | --- | --- | --- | --- |
| benign_ascii | `q=hello+world`, ISO-8859-1 | 200, `oob=-1` (no alloc of the class) | 403 | 200, no effect |
| benign_utf8_sharp | 21 valid UTF-8 `ß` (`%C3%9F`), UTF-8 | 200, `oob=0` — **no write** | 403 | 200, no effect |
| benign_below_threshold | 20 sharp-s (`0xDF`) bytes | 200, `oob=0` — **no write** | 403 | 200, no effect |
| attack_query_pct | 21 sharp-s bytes, percent-encoded in the query | 200, `oob=83` (`0x53`) — **OOB write** | 403 | 403, refused |
| attack_body_raw | 21 raw `0xDF` in a form body | 200, `oob=83` — **OOB write** | 403 | 403, refused |
| attack_body_len28 | 7 `a` + 21 `0xDF` (the 28-byte allocation class) | 200, `oob=83` — **OOB write** | 403 | 403, refused |
| attack_body_pct | 21 `%DF` escapes in a form body | 200, `oob=83` — **OOB write** | 403 | 403, refused |
| attack_header_value | 21 `0xDF` in `X-Gabriel-Query` | 200 — **neutralized in flight** (below) | 403 | 403, refused |

**In-flight neutralization finding (header vector).** Eden's forward path percent-encodes header **values**: the fixture's audit observed `in_len=63` — the literal ASCII text `%DF%DF…`, not the raw bytes — so no conversion corruption occurs at the fixture through an interlay for this header shape. The module still refuses the original raw head shape (the tier-2 403), which covers deployments where the header reaches the runtime untransformed. Recorded honestly: the baseline corruption deltas for the header probe are zero **through Eden**, and the refusal is not evidence of a heap save for that specific vector — it is boundary defense for the original shape.

## Signed module authority

Import returned HTTP 201 with explicit activation required and signature verification deferred. Read-only inspection was refused (403) for the reader principal, admission of a tampered statement was rejected (400), and admission of an invalid signature was refused (403). The activation receipt reported the module serving on all 32 expected shards. After revocation the marked probe received HTTP 503 (fail-closed); re-approval and reactivation restored the refusal.

- Source SHA-256 `d74ff3358cd70a35df8a12b2dbd59cda693428c083203d032082534913c9ec0f`
- Recorded test binary SHA-256 `92cd8a6da164943118566a741d586da3c35477bddd22f2b532d10f678fd05c87`
- Recorded activation operation `96cb58fd-fd09-45ad-8417-fd7bc1f1fac0`

## Performance

Instrument: client wall-clock through the interlay (includes the network hop), warmup discarded, mean/p50/p95 in milliseconds.

| Arm | n | mean | p50 | p95 |
| --- | --- | --- | --- | --- |
| Tier 1 — declarative policy, blocked (403) | 15 | 0.4 | 0.4 | 0.5 |
| Tier 2 — module, benign passes (200) | 20 | 13.7 | 10.8 | 21.4 |
| Tier 2 — module, attack refused | 15 | 0.5 | 0.5 | 0.6 |
| Baseline — unprotected interlay, benign (200) | 20 | 53.0 | 53.0 | 68.0 |

The baseline arm measures the fixture's fork-isolated audit oracle (one audit child per request), so it is not a like-for-like module-overhead comparison; the refusal and module arms are the comparable pair.

## Evidence and artifacts

- [Vendor advisory](https://verialabs.com/ruby2shell/) · [GitHub advisory](https://github.com/ruby/ruby/security/advisories/GHSA-34m4-m9fh-gqp5)
- [Upstream fix ruby/ruby@d582f8f](https://github.com/ruby/ruby/commit/d582f8f13d700d8e39714d3a8f11eb4bd2bc69f0)
- [Published module source](modules/GHSA-34m4-m9fh-gqp5.rs)
- [Bundle](https://github.com/eden-dev-inc/cve/blob/main/cve/GHSA-34m4-m9fh-gqp5/README.md)
- Sibling advisories from the same Ruby release (unpatched here): GHSA-87fm-7w84-j2px (integer overflow in case conversion), GHSA-7x2q-68qv-6p8x, GHSA-wpqw-5qxg-m68r, GHSA-3p8q-wmhx-vm2w, GHSA-7whj-cfh7-xgm4, GHSA-pvgc-g7gj-pv6h (Regexp engine) — see [the annex](SCOPE-AND-DISPOSITIONS.md)
