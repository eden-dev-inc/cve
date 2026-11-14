# Eden Sphinx CVE patches

Browse virtual patches for published CVEs, with Rust module source, deployment
configuration, API examples and case studies that explain the observed
behavior and where coverage ends.

- [CVE patch bundles](cve/README.md)
- [Case study catalogue](docs/case-studies/README.md)
- [Additional case-study module sources](docs/case-studies/modules/README.md)
- [Contribute a patch or improvement](CONTRIBUTING.md)
- [Implement, test and apply a patch with Eden Sphinx](docs/sphinx-remediation.md)

Get started with the [Log4j JNDI Lookup Guard](cve/CVE-2021-44228/README.md).
Its bundle links the implementation, deployment declaration, API example and
the evidence for its test scope.

## Skills

- [CVE packaging](skills/cve-packaging/SKILL.md) explains the repository layout,
  patch titles, versions and what belongs in a bundle.
- [Eden Sphinx remediation](skills/eden-sphinx-remediation/SKILL.md) covers
  authoring, testing, packaging and applying a virtual patch.

The [contributor check](CONTRIBUTING.md#run-the-public-check) runs in a fresh
fork. Native builds require the compatible Eden SDK identified in the bundle.
