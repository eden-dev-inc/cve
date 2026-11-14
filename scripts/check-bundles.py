#!/usr/bin/env python3
"""Check public CVE bundle metadata without the Eden SDK (Python 3.11+)."""

import argparse
import json
from pathlib import Path
import re
import subprocess
import tomllib

ROOT = Path(__file__).resolve().parent.parent
CVE = r"CVE-\d{4}-\d{4,}"
SHARED_FILES = {
    ".github/workflows/bundle-check.yml",
    "cve/README.md",
    "cve/patch-titles.json",
    "docs/case-studies/README.md",
    "scripts/check-bundles.py",
}
REQUIRED_FILES = ("api.md", "Cargo.toml", "patch.toml", "deployment.json", "src/lib.rs")


def changed_cves(base):
    output = subprocess.check_output(
        ["git", "diff", "--name-only", "--no-renames", "-z", base, "HEAD", "--"],
        cwd=ROOT,
    )
    paths = output.decode().split("\0")
    if SHARED_FILES.intersection(paths):
        return None

    cves = set()
    for path in paths:
        bundle = re.match(rf"cve/({CVE})/", path)
        study = re.fullmatch(rf"docs/case-studies/({CVE.replace('-', '_')})\.md", path)
        if bundle:
            cves.add(bundle[1])
        if study:
            cves.add(study[1].replace("_", "-"))
    return cves


def check_adapter(adapter, cve, reservations):
    for name in REQUIRED_FILES:
        if not (adapter / name).is_file():
            yield f"missing {name}"

    try:
        cargo = tomllib.loads((adapter / "Cargo.toml").read_text())
        patch = tomllib.loads((adapter / "patch.toml").read_text())["patch"]
        deployment = json.loads((adapter / "deployment.json").read_text())
        module, title, version = patch["module"], patch["title"], patch["semver"]
        slug = re.sub(r"[^a-z0-9]+", "-", title.lower()).strip("-")
        if not slug or module != slug:
            yield "patch.module must be the lowercase hyphenated patch.title"
        if reservations.get(module) != cve:
            yield f"{module!r} must be reserved for {cve} in cve/patch-titles.json"
        if cargo["package"]["name"] != module or deployment["module"] != module:
            yield "Cargo package and deployment module must match patch.module"
        if cargo["lib"]["name"] != slug.replace("-", "_"):
            yield "Rust library target must use the patch slug with underscores"
        if cargo["package"]["version"] != version or deployment["semver"] != version:
            yield "Cargo and deployment versions must match patch.semver"
        if patch["advisory"] != cve or deployment["advisory"] != cve:
            yield "patch and deployment advisory must match the CVE folder"
    except FileNotFoundError:
        return  # Missing files were already reported above.
    except (ValueError, KeyError, TypeError, AttributeError) as error:
        yield f"invalid bundle metadata: {error}"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("cve", metavar="CVE-ID", nargs="*")
    parser.add_argument("--base", help="Check bundles changed between this Git ref and HEAD")
    args = parser.parse_args()
    if args.base and args.cve:
        parser.error("pass CVE IDs or --base")
    for cve in args.cve:
        if not re.fullmatch(CVE, cve):
            parser.error(f"invalid CVE ID: {cve}")

    bundle_root = ROOT / "cve"
    studies = ROOT / "docs/case-studies"
    bundles = {
        path.name: path for path in bundle_root.iterdir()
        if re.fullmatch(CVE, path.name) and path.is_dir()
    }
    targets = changed_cves(args.base) if args.base else set(args.cve) or None
    if targets is None:
        targets = set(bundles)
    if args.base:
        targets &= bundles.keys()  # Case studies can exist without a bundle.
    if not targets:
        print("No CVE bundles selected.")
        return 0

    reservations = json.loads((bundle_root / "patch-titles.json").read_text())
    bundle_index = (bundle_root / "README.md").read_text()
    study_index = (studies / "README.md").read_text()
    errors = []
    for cve in sorted(targets):
        if cve not in bundles:
            errors.append(f"{cve}: no bundle in cve/")
            continue
        folder = bundles[cve]
        study = cve.replace("-", "_") + ".md"
        if not (folder / "README.md").is_file():
            errors.append(f"{cve}: missing README.md")
        if f"]({cve}/README.md)" not in bundle_index:
            errors.append(f"{cve}: missing link in cve/README.md")
        if not (studies / study).is_file() or f"]({study})" not in study_index:
            errors.append(f"{cve}: missing case study or link in docs/case-studies/README.md")

        adapters = sorted(
            adapter for adapter in folder.iterdir()
            if adapter.is_dir() and adapter.name.endswith("-filter")
        )
        if not adapters:
            errors.append(f"{cve}: no adapter bundle")
        for adapter in adapters:
            errors.extend(
                f"{adapter.relative_to(ROOT)}: {error}"
                for error in check_adapter(adapter, cve, reservations)
            )

    if errors:
        parser.exit(1, "\n".join(errors) + "\n")
    print(f"Checked {len(targets)} CVE bundle(s).")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
