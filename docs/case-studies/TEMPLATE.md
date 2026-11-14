# CVE case study and bundle template

Use this template to publish a Project Gabriel case study and its companion Eden remediation materials. [Log4j (CVE-2021-44228)](CVE_2021_44228.md) is the reference for the breadth of the case study and the [versioned bundle](../../cve/CVE-2021-44228/README.md).

Give each patch a unique descriptive title. Use its lowercase hyphenated form as the registry package name and Cargo package name. Use underscores for the Rust library target. Keep the CVE ID in advisory fields and paths. Store the release version in the registry's SemVer field and the bundle's version fields. Record historical package identities and digests with their original evidence.

## File layout

```text
docs/case-studies/CVE_<year>_<number>.md
cve/CVE-<year>-<number>/
  README.md
  <protocol-adapter>/
    Cargo.toml
    patch.toml
    deployment.json
    api.md
    src/lib.rs
```

| File | What to include |
| --- | --- |
| Case study | Vulnerability, enforcement decision, implementation steps, test method and results, coverage, and links to public evidence and source. Use the starter below. |
| Bundle `README.md` | CVE and vendor advisory, evidence status, supported product and Eden versions, links to the case study and bundle files, and the relevant artifact digests. |
| `Cargo.toml` and `src/lib.rs` | The module crate and exact source. Use the canonical package name and SemVer from `patch.toml` and the underscore form for the Rust library. |
| `patch.toml` | Patch title, registry package name, advisory, package SemVer, Eden compatibility, protocol and ABI, SDK revision, target, compiler, and build profile. Follow the [existing manifest](../../cve/CVE-2021-44228/http-request-filter/patch.toml). |
| `deployment.json` | The applicable policy or native declaration, including route and software scope. Use placeholders for deployment-specific values. Label fixture settings. |
| `api.md` | Prerequisites, parameterized API commands, effective-state readback, verification, revocation, and restore. Use [the Log4j API example](../../cve/CVE-2021-44228/http-request-filter/api.md) and [deployment prerequisites](../deployment.md). |

For standalone Rust sources linked from a report, use `docs/case-studies/modules/CVE-YYYY-NNNN.rs`. List the file, case study, and source SHA-256 in the [module index](modules/README.md).

For an assessment, a declarative-only result, or a failed native import, publish the report and link the artifacts that exist. Record the exact stage reached and the configuration that was exercised.

## Case study starter

Open every case study with the practical consequences of exploitation and why the affected deployment needs remediation. Ground those consequences in the advisory or other cited evidence. Describe prerequisites and qualify the impact when it depends on configuration, access, or a specific version.

````markdown
# <Product and vulnerability> (CVE-YYYY-NNNN)

<In two or three sentences, explain what a successful attacker could do, which systems, users, or data are at risk under the affected conditions, and why operators need to remediate. State what remains exposed while the affected system stays unpatched.>

[Vendor advisory](<url>) · [CVE record](<url>) · [Source and API bundle](<relative path>)

**Result.** <State the observed outcome, where Eden enforced it, and the strongest coverage the evidence supports.>

| Field | Value |
| --- | --- |
| Affected component and versions | <Vendor-confirmed range and prerequisites> |
| Fixed release | <Version by release line, or unavailable> |
| Severity | <CVSS score and scoring source, when available> |
| Test environment | <Synthetic fixture or affected product, exact build and configuration> |
| Eden release and module version | <Release, SemVer and revision when applicable> |
| Tested at | <UTC date or timestamp> |
| First publicly published | <UTC timestamp from the verified publication record, once available> |

## Vulnerability and enforcement point

<Describe the attacker-controlled input, entry point, required access or configuration, and vulnerable operation. Explain which traffic or operation Eden can see and where that traffic reaches the affected component.>

## Eden remediation

- **Declarative policy** <Exact scope and decision. Link or show the tested policy. State which legitimate requests it blocks.>
- **Native module** <Exact scope, inspected input, match conditions, action, and behavior for incomplete input or no match. Link the exact source and include a short excerpt if it clarifies the decision. If this tier was unavailable or failed, record the stage and observed outcome.>

## Implement the remediation

<Give the operator the steps in execution order. Put the case-specific commands, API requests, and configuration changes in the report. Link the exact deployment declaration and companion API example for full payloads. Define each placeholder before use and identify the Eden release used to verify the commands.>

1. **Prepare the target.** <Specify the affected product and version, compatible Eden release, required permissions, and target endpoint or interlay. Name the policy or module version and where to get it.>
2. **Apply the protection.** <For a declarative policy, show the exact policy and how to publish it to the target. For a native module, show how to obtain or build the versioned archive, verify its archive digest, import it, complete signed approval and admission, and explicitly activate it. Include the command or API request for each applicable action.>
3. **Confirm enforcement.** <Show the readback command and the expected effective policy digest or activation receipt, including shard acknowledgment where applicable. Give the benign and advisory-shaped verification probes for an isolated environment and the expected gateway and backend outcomes.>
4. **Roll back or restore.** <Show how to restore the prior policy or revoke and restore the native operation. Explain what happens to traffic during the change and how to verify the resulting state.>

<Link the vendor’s fix instructions for the affected release line. If an Eden path was untested, failed, or was unavailable, state the stage reached. Label any example steps that were not run and include the vendor’s documented update or workaround when available.>

## Measured comparison (<YYYY-MM-DD> UTC)

<Describe the benign probe, advisory-shaped probe, relevant bypass variants, and the backend effect or state you measured. Explain how the effect was observed independently of the gateway response. Use the same probes across comparable configurations.>

| Probe | Baseline | Declarative policy | Native module |
| --- | --- | --- | --- |
| Benign | <Status and effect> | <Status and effect> | <Status and effect> |
| Advisory-shaped | <Status and effect> | <Status and effect> | <Status and effect> |
| Variant or negative control | <Status and effect> | <Status and effect> | <Status and effect> |

<Use “not tested” where a check was skipped. If the baseline never produced the expected effect, report the filter’s observed action and leave prevention of that effect unverified.>

## Activation and rollback

<Record authorization checks, effective policy or signed module readback, and acknowledged versus expected shards where applicable. State what happened to the same probes after revoke and after restore. Link the sanitized receipt or evidence summary when available.>

## Performance

<When measured, give the instrument, unit, sample count, mean, p50 and p95 per arm, and describe what the measured interval includes. Report declarative and native module timings in this section. Use the measured comparison table for baseline probe outcomes. Otherwise state that performance was not measured.>

## Test scope

<State which routes, versions, and inputs were exercised, along with any false positives, known bypasses, and untested paths. Record whether the test reached the vulnerable component and reproduced the vendor-described effect. Identify the vendor’s durable fix.>

## Evidence and artifacts

- <Public test summary, evidence identifier, or reproducible recipe with private details removed>
- <Pinned module source, deployment declaration, and API example when available>
- <Source SHA-256, tested binary SHA-256, and archive SHA-256 when available. Label each digest separately.>
````

## Before publishing

- Check that the opening states a concrete consequence and gives operators a reason to remediate. Cite the source for the stated impact.
- Check that an operator can follow the case-specific implementation steps, substitute the documented values, verify enforcement, and roll back or restore the change.
- Match the CVE, module version, scope, and linked source across the report and bundle. Label example declarations and observed results.
- Confirm each coverage claim against the baseline, the protected result, and the legitimate-traffic probes. Record failed imports and bypasses with their observed outcomes.
- Verify links and available digests. Keep credentials, customer data, private addresses, and internal lab paths out of public material.
- Link new reports from the [case study index](README.md) and versioned bundles from the [bundle index](../../cve/README.md). For website publication, pin the public Git revision and record the first verified public timestamp in the catalog. Preserve that timestamp when editing the report.
