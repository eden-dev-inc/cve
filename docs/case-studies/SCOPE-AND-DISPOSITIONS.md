# Enforcement scope and recorded dispositions

This document records advisories triaged against the current Eden HTTP remediation profile and the reason each remains outside the recorded case studies. It follows the portfolio's assessment rule: state the exact stage reached rather than relabeling a gap as coverage. As of 2026-11-13 the qualified profile is the 1.103.0 single-node HTTP/1 native MVP — a request/response interlay whose declarative policies and native modules see the HTTP method, target, headers, and body of traffic they front.

## Why these advisories have no recorded remediation

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
| CVE-2026-124194 | Microsoft Outlook spoofing (exploited; victim must interact) |
| CVE-2026-121529 | Fortinet FortiClientMac/Windows origin validation (victim must interact) |
| CVE-2026-70279 | Microsoft Exchange XSS spoofing (user must open a malicious link) |
| CVE-2026-96951 | Windows Shell feature bypass (client) |
| CVE-2026-110936 | Siemens Software Center DLL planting (local; exploited per CISA) |
| CVE-2026-86950 | Out-of-bounds write addressed by bounds checking (client component) |
| CVE-2026-126148 | FortiMail HA join-request command injection (HA packet stream, not frontable HTTP) |
| CVE-2026-106173 | FortiSIEM traversal code execution (vendor documents crafted packets, not HTTP) |
| CVE-2026-114713 | UnrealIRCd IRCv3 label tag injection (server-to-server IRC protocol) |
| CVE-2026-105445 | Docker Hub account-conversion token reassignment (SaaS platform flaw, no frontable boundary) |
| CVE-2026-59346 | VMware Workstation VMXNET3 hypervisor transition |
| CVE-2026-96012 | IBM Db2 Mirror for i |
| CVE-2026-98464, CVE-2026-98490, CVE-2026-98548, CVE-2026-98549, CVE-2026-98959, CVE-2026-99078, CVE-2026-99079, CVE-2026-99999 | Linux kernel RISC-V KVM, OVS conntrack, SCTP, TCP, Btrfs, mlx5e, IPv4 (local kernel) |
| CVE-2026-85746, CVE-2026-85748, CVE-2026-92262 | MariaDB ST_AsText OOB write, SYS_REFCURSOR use-after-free, #mysql50# code execution (database wire/library) |
| CVE-2026-106195, CVE-2026-106234, CVE-2026-106237, CVE-2026-106241, CVE-2026-106294, CVE-2026-106329, CVE-2026-106372, CVE-2026-106375, CVE-2026-106401, CVE-2026-106414, CVE-2026-106417, CVE-2026-110136, CVE-2026-110142, CVE-2026-110144, CVE-2026-110149, CVE-2026-110152, CVE-2026-110159, CVE-2026-110161, CVE-2026-110165, CVE-2026-110916, CVE-2026-112992, CVE-2026-113007, CVE-2026-113014, CVE-2026-113020, CVE-2026-113023, CVE-2026-113056, CVE-2026-113072, CVE-2026-113078, CVE-2026-113084, CVE-2026-113089, CVE-2026-113116, CVE-2026-113126, CVE-2026-113138, CVE-2026-113169, CVE-2026-113171, CVE-2026-113186, CVE-2026-113189, CVE-2026-113211, CVE-2026-113222, CVE-2026-113235, CVE-2026-113252, CVE-2026-113261, CVE-2026-113263, CVE-2026-113270, CVE-2026-113283, CVE-2026-113300, CVE-2026-113328, CVE-2026-113336, CVE-2026-113342, CVE-2026-118305, CVE-2026-118308, CVE-2026-118309, CVE-2026-118310, CVE-2026-118312, CVE-2026-118316, CVE-2026-118321, CVE-2026-118332, CVE-2026-118340, CVE-2026-118342, CVE-2026-118355 | Google Chrome rendering/engine family (V8, ANGLE, Skia, Dawn, Compositing, FileSystem, Mojo, Updater, DevTools, sandbox escapes) — client-side execution in the victim's browser |
| CVE-2025-24054 | Windows NTLM hash disclosure spoofing |
| CVE-2026-128116 | SvelteKit route-resolution XSS (victim's browser) |
| CVE-2026-68541 | PolicyPress CSS-only redaction and workflow defaults |

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
| CVE-2019-9513, CVE-2026-33871 | HTTP/2 framing (ping loop and Netty) |
| CVE-2026-116945 | Microsoft SQL Server TDS heap overflow |
| CVE-2026-118383 | MongoDB replica-set wire protocol |
| CVE-2020-11651 | SaltStack ZeroMQ message bus |
| CVE-2020-3952 | VMware vmdir (LDAP) |
| CVE-2021-33912, CVE-2021-33913 | libspf2 SPF parsing (DNS TXT) |
| CVE-2023-0286 | X.509 X.400 type confusion (TLS handshake) |
| CVE-2024-4741 | OpenSSL SSL_free_buffers (TLS stack) |
| CVE-2026-85102 | Check Point VPN certificate validation |
| CVE-2026-59310 | VMware vCenter Syslog service (UDP) |
| CVE-2026-128087, CVE-2026-128088 | russh SSH host-key / agent flaws (SSH wire) |
| CVE-2026-128089 | dnspython WKS parser DoS (DNS record parsing library) |
| CVE-2026-128128, CVE-2026-128129, CVE-2026-128132 | aiohttp WebSocket frame handling and connection data buffered while an upgrade is pending |
| CVE-2026-129217 | FRRouting bgpd OOB read (BGP wire) |

The [aiohttp CVE-2026-128131 case study](CVE_2026_128131.md) tested an HTTP request body size rule. WebSocket frames and client response decoding remain outside that test.

The [Apache mod_http2 case study for CVE-2026-119618](CVE_2026_119618.md) records a policy and module comparison on an HTTP/1.1 fixture. The HTTP/2 frame race in the affected Apache implementation remains outside that test.

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
| CVE-2026-128885, CVE-2026-128987 | RHEL 10 dbus local connection/transport DoS |
| CVE-2026-123039 | Check Point Gaia DynamicCLI root escalation through a local socket |
| CVE-2026-102490 | Zammad local privilege escalation in DEB/RPM service startup |

## Category D — application state, build output, and pending tests

| Advisory | Where the advisory applies |
| --- | --- |
| CVE-2026-128118 | SvelteKit build guard lets server-only imports enter the browser bundle |
| CVE-2026-128121 | SvelteKit prerender crawler emits unescaped paths into generated routing rules |
| CVE-2026-128122 | SvelteKit prerender crawler can inject rules into generated Cloudflare redirects |
| CVE-2026-128117, CVE-2026-128123 | SvelteKit adapter-node crashes during route decoding or request handling. No fixture comparison was recorded. |
| CVE-2026-128100 | Vault authorization depends on deployed ACLs and case-insensitive identity handling |
| CVE-2026-68551 | Budget Board account linking depends on validation of the identity provider's verified-email claim |
| CVE-2026-102489 | Zammad session hijacking leading to code execution. No fixture comparison was recorded. |

## Recording status

Stage reached for every entry above: **triaged against the 1.103.0 HTTP/1 interlay; no fixture, policy, or module was recorded, and none is claimed.** Entries move out of this document only when a live case study exists (see the [case study index](README.md)). Protocol entries additionally track the adapter backlog; when a wire adapter ships and is qualified, the corresponding advisories return to the actionable queue.
