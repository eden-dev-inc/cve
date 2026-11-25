# Contribute to Eden Sphinx CVE patches

Fork [eden-dev-inc/cve](https://github.com/eden-dev-inc/cve), then clone your
fork and branch from its current `main`.

```sh
git clone https://github.com/YOUR-HANDLE/cve.git
cd cve
git switch -c improve-log4j-jndi-lookup-guard
```

Start at the [bundle index](cve/README.md) or search for the CVE ID. A bundle's
README links its code, deployment example, API and case study. Edit source in
`cve/<CVE-ID>/<adapter>/src/lib.rs` when a bundle exists. Source linked as
historical evidence stays tied to its original commit and digests.

For a new CVE, use the [case study and bundle template](docs/case-studies/TEMPLATE.md).
Add the bundle, both catalogue links and a reservation in
[`cve/patch-titles.json`](cve/patch-titles.json) in the same PR. Use the advisory ID
for the folder and a unique descriptive patch title for the package. Keep the
title free of project branding and release numbers. Carry the release separately
in the registry's SemVer field and the bundle metadata.

## Run the public check

Run this from the repository root with Python 3.11 or later.

```sh
python3 scripts/check-bundles.py CVE-2021-44228
```

Omit the CVE ID to check every bundle. The check reads bundle files and verifies
required files, title ownership, naming and version consistency, and catalogue
links. GitHub Actions runs it for CVEs changed in a pull request, and checks
the full catalogue when shared validation files change.

For changes to patch behavior, include repeatable benign and advisory-shaped
probes and their expected application effects. Use the
[Sphinx walkthrough](docs/sphinx-remediation.md) for the native build and staging
check. Include the exact SDK/host version and observed results when available.
A fresh checkout can run the public check without an Eden account. If you lack
the compatible SDK or staging access, say which checks are pending so a
maintainer can reproduce them before release.

## Submit a pull request

Push your branch to your fork and open a pull request against `eden-dev-inc/cve:main`.
Include the CVE ID, what the change prevents, which legitimate traffic should
still work, test commands and results, and any remaining coverage limits.
Use sanitized fixtures and logs. For a code change, keep the deployment example
and report aligned with the proposed behavior while preserving historical evidence.

## Maintainer handoff

GitHub is the contribution entry point. For an accepted change, carry the merged
GitHub commit and its CVE-specific diff into the corresponding GitLab publishing
source before the next publication of that CVE. Record the GitHub PR and commit
in that change. Apply the accepted bundle, tests and report changes to the
existing source and resolve any overlapping authoring changes there.

Before publishing a new wave, compare the outgoing files for each affected CVE
against current GitHub `main`. Preserve accepted changes and review any
divergence before updating an existing bundle or case study. For example,
from the GitLab checkout, compare the outgoing version of these paths with the
latest public revision:

```sh
git fetch --no-tags https://github.com/eden-dev-inc/cve.git main
git diff FETCH_HEAD -- cve/CVE-2021-44228 docs/case-studies/CVE_2021_44228.md
```

The diff shows what publication would change relative to GitHub. Validate any
changed behavior in staging, then use the registry's release process for changed
package bytes. After publication, verify the source diff and the registry's
source commit, version and archive digest.
Follow the [signed .epatch release workflow](docs/sphinx-epatch.md) for proof
and source verification, and record the immutable Registry result in the
[migration inventory](docs/epatch-migration.csv).
