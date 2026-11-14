---
name: eden-sphinx-remediation
description: Implement, test and apply an Eden Sphinx CVE mitigation using a scoped declarative policy or native HTTP module, a compatible SDK, and an authorized staging interlay.
---

# Remediate a CVE with Eden Sphinx

Use the [Sphinx walkthrough](../../docs/sphinx-remediation.md) for the supported
profile, build commands, package identity and staging runbook.

1. Start from the vendor advisory. Identify the vulnerable operation, affected
   version and observable backend effect. Reproduce it in an isolated test
   environment and check which routes reach it through Eden.
2. Choose a scoped declarative policy for a structural requirement or a
   compatible native Rust module for bounded inspection or editing. For HTTP
   v3, keep the contract and declared capabilities consistent with the host.
   Use `NeedMore` while incomplete input could change the result.
3. Check the public bundle locally. With the compatible SDK, build the actual
   module, retain build provenance and use the canonical package producer.
   Track source, binary and archive digests separately.
4. Use an authorized staging interlay. Import the exact archive and retain
   the returned configuration. For native code, complete host approval,
   typed publisher signing, admission and activation. Read the operation
   receipt and expected shard acknowledgments.
5. Compare baseline and protected outcomes for benign, advisory-shaped and
   boundary cases. Verify backend effects independently. Exercise rollback
   or revoke and restore. Record what was actually run and the remaining
   bypasses or access gaps.

Use [CVE packaging](../cve-packaging/SKILL.md) for the public bundle and evidence.
