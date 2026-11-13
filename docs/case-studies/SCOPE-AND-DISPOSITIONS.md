# Enforcement scope and recorded dispositions

This document records the triage of feed advisories that the current Eden HTTP remediation profile cannot act on, and why. It follows the portfolio's assessment rule: state the exact stage reached rather than relabeling a gap as coverage. As of 2026-11-13 the qualified profile is the 1.103.0 single-node HTTP/1 native MVP — a request/response interlay whose declarative policies and native modules see the HTTP method, target, headers, and body of traffic they front.

## Why these are not remediable at this boundary

**Client-side execution (category A).** The exploitation primitive runs inside the victim's browser, document renderer, or endpoint application. The CVE-specific malicious bytes travel inside otherwise-legitimate content (a web page, a PDF, an email attachment); Eden fronts a server, not the victim's process, and refusing all such content is a blanket block rather than a CVE-specific control. These records name the affected component so a customer can confirm their patch state; they are not claims of Eden coverage.

**Wire-protocol gaps (category B).** The vulnerable traffic is not HTTP/1 on a fronted port: DNS/DoH, SMB, iSCSI, SSH, SIP, TDS, PostgreSQL/MongoDB wire protocols, HTTP/2 framing below the interlay's line-based view, VPN/TLS handshake internals, or appliance-internal protocols (FGFM, OFTP, ICA, LSA, HA, syslog). The native MVP has no adapter for these protocols; several are tracked in the protocol-adapter backlog, and each becomes actionable when its adapter ships.

**Local execution and host libraries (category C).** The primitive executes inside the host kernel, a setuid binary, a hypervisor transition, or a linked library after bytes reach the process by some non-HTTP means. No HTTP request carries the primitive across an interlay Eden could front for the affected component.

## Category A — client-side execution

| Advisory | Component |
| --- | --- |
| CVE-2018-4200, CVE-2023-41974, CVE-2024-27834, CVE-2025-43520, CVE-2026-65400, CVE-2026-120808 | Apple platform components |
| CVE-2018-4878 | Adobe Flash Player |
| CVE-2018-4990, CVE-2020-9715, CVE-2021-21017, CVE-2021-28550, CVE-2023-21608, CVE-2023-26369, CVE-2026-34621 | Adobe Acrobat / Reader |
| CVE-2026-21509 | Microsoft Office trust checks (local) |
| CVE-2025-6554, CVE-2025-6558, CVE-2025-10585, CVE-2025-13223, CVE-2025-14174, CVE-2025-2783, CVE-2025-4609, CVE-2025-5419, CVE-2026-2441, CVE-2026-3910, CVE-2026-85046, CVE-2026-87491 | Google Chrome (V8, ANGLE, Skia, Mojo, FileSystem) |
| CVE-2026-118306, CVE-2026-119261, CVE-2026-119263, CVE-2026-120931, CVE-2026-121030, CVE-2026-121034, CVE-2026-121035, CVE-2026-121037, CVE-2026-121060, CVE-2026-121071, CVE-2026-121073, CVE-2026-121074, CVE-2026-121076, CVE-2026-121077, CVE-2026-121086, CVE-2026-121089, CVE-2026-121104, CVE-2026-121114, CVE-2026-121126, CVE-2026-121131, CVE-2026-121148, CVE-2026-121156, CVE-2026-121157, CVE-2026-121185, CVE-2026-121189, CVE-2026-121200, CVE-2026-121202, CVE-2026-121214, CVE-2026-121229, CVE-2026-121234, CVE-2026-121241, CVE-2026-121246, CVE-2026-121247, CVE-2026-121249, CVE-2026-121250, CVE-2026-121257, CVE-2026-125347, CVE-2026-125350, CVE-2026-126842, CVE-2026-126846, CVE-2026-126859, CVE-2026-126872, CVE-2026-126885, CVE-2026-126909, CVE-2026-126912 | Google Chrome 157.x family |
| CVE-2025-15556 | Notepad++ WinGUp updater |
| CVE-2025-24054 | Windows NTLM hash disclosure spoofing |

## Category B — wire-protocol gaps

| Advisory | Protocol / surface |
| --- | --- |
| CVE-2020-1350 | Windows DNS (SIGRED) |
| CVE-2026-107407, CVE-2026-107408, CVE-2026-107409, CVE-2026-107410, CVE-2026-120868 | Citrix NetScaler SMPP / DNS / DoH / Push / packet-processing |
| CVE-2026-120864 | NetScaler ICA proxy |
| CVE-2026-126253 | FortiOS LSA broadcast |
| CVE-2026-126254 | FortiOS iBGP attributes |
| CVE-2026-126259 | FortiOS HA FGSP packets |
| CVE-2026-126261 | FortiManager FGFM |
| CVE-2026-126262 | FortiAnalyzer OFTP SQL injection |
| CVE-2019-19492 | FreeSWITCH event socket |
| CVE-2026-116779 | FreeSWITCH inbound SIP MESSAGE |
| CVE-2026-4408 | Samba samr (SMB) |
| CVE-2026-99153, CVE-2026-99154 | libiscsi PDU handling |
| CVE-2025-32433 | Erlang/OTP SSH |
| CVE-2019-9513, CVE-2026-33871, CVE-2026-119618 | HTTP/2 framing (ping loop, Netty, Apache mod_http2) |
| CVE-2026-116945 | Microsoft SQL Server TDS heap overflow |
| CVE-2026-118383 | MongoDB replica-set wire protocol |
| CVE-2020-11651 | SaltStack ZeroMQ message bus |
| CVE-2020-3952 | VMware vmdir (LDAP) |
| CVE-2021-33912, CVE-2021-33913 | libspf2 SPF parsing (DNS TXT) |
| CVE-2023-0286 | X.509 X.400 type confusion (TLS handshake) |
| CVE-2024-4741 | OpenSSL SSL_free_buffers (TLS stack) |
| CVE-2026-85102 | Check Point VPN certificate validation |
| CVE-2026-59310 | VMware vCenter Syslog service (UDP) |

## Category C — local execution and host libraries

| Advisory | Component |
| --- | --- |
| CVE-2021-4034 | polkit pkexec (local) |
| CVE-2025-32463 | sudo nsswitch (local) |
| CVE-2023-28252 | Windows CLFS driver (local) |
| CVE-2024-1086 | Linux netfilter nf_tables (local) |
| CVE-2026-46300 | Linux kernel skbuff (local) |
| CVE-2026-43284 | Linux kernel xfrm ESP (local) |
| CVE-2026-72422 | ksmbd concurrent NEGOTIATE (local) |
| CVE-2026-80587 | mptcp suboptions (local) |
| CVE-2026-89633 | CIFS client coalesce (local) |
| CVE-2026-98432 | ipv4 ireq_opt (local) |
| CVE-2026-111357 | Ivanti dsinvoked local broker escalation |
| CVE-2025-22224, CVE-2025-22226 | VMware ESXi guest-to-host / OOB read |
| CVE-2026-41709 | VMware ESX log evasion |
| CVE-2019-17543 | LZ4 heap overflow (library) |
| CVE-2025-9230 | RFC 3211 KEK unwrap (library) |
| CVE-2024-9143 | OpenPGP GF(2^m) parameters (library) |
| CVE-2026-112397, CVE-2026-112399, CVE-2026-112400, CVE-2026-112402, CVE-2026-112404, CVE-2026-112405, CVE-2026-112894, CVE-2026-112896 | Pillow image decoding (library) |
| CVE-2026-110856, CVE-2026-110859 | GitPython (library) |
| CVE-2026-118665 | GraphQL Tools (library) |
| CVE-2026-118782, CVE-2026-119191, CVE-2026-119196, CVE-2026-119283 | Forge / pyasn1 / joserfc (libraries) |
| CVE-2026-114102, CVE-2026-114143 | adm-zip / toml-node (libraries) |

## Recording status

Stage reached for every entry above: **triaged against the 1.103.0 HTTP/1 interlay; no fixture, policy, or module was recorded, and none is claimed.** Entries move out of this document only when a live case study exists (see the [case study index](README.md)). Protocol entries additionally track the adapter backlog; when a wire adapter ships and is qualified, the corresponding advisories return to the actionable queue.
