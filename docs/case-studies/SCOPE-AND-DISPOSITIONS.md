# Enforcement scope and recorded dispositions

This document records advisories triaged against the current Eden HTTP remediation profile and the reason each remains outside the recorded case studies. It follows the portfolio's assessment rule: state the exact stage reached rather than relabeling a gap as coverage. As of 2026-11-13 the qualified profile is the 1.103.0 single-node HTTP/1 native MVP — a request/response interlay whose declarative policies and native modules see the HTTP method, target, headers, and body of traffic they front.

The 30-day review closed on November 13, 2026. The review recorded dispositions for 292 distinct advisories. Waves 31 and 32 added 19 more. Wave 36 recorded 22 additional dispositions, bringing this annex to 333. Wave 37 recorded 62 additional dispositions, bringing this annex to 395. The 2026-11-18 cve.now critical-feed review added 15 more, bringing this annex to 410. The 2026-11-18 cve.now newest-list review (high/critical page 2) added 15 more, bringing this annex to 425. The 2026-11-19 Ruby release review (ruby2shell and siblings) added 6 more, bringing this annex to 431. The 2026-11-19 php2shell incident rollup review added 6 more (CVE-2026-126714 is patched separately), bringing this annex to 437. The 2026-11-19 cve.now critical-feed recheck added 12 more, bringing this annex to 449.

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
| CVE-2026-86950 | Apple CoreGraphics processing of crafted files |
| CVE-2026-106195, CVE-2026-106234, CVE-2026-106237, CVE-2026-106241, CVE-2026-106294, CVE-2026-106329, CVE-2026-106372, CVE-2026-106375, CVE-2026-106401, CVE-2026-106414, CVE-2026-106417, CVE-2026-110136, CVE-2026-110142, CVE-2026-110144, CVE-2026-110149, CVE-2026-110152, CVE-2026-110159, CVE-2026-110161, CVE-2026-110165, CVE-2026-110916, CVE-2026-112992, CVE-2026-113007, CVE-2026-113014, CVE-2026-113020, CVE-2026-113023, CVE-2026-113056, CVE-2026-113072, CVE-2026-113078, CVE-2026-113084, CVE-2026-113089, CVE-2026-113116, CVE-2026-113126, CVE-2026-113138, CVE-2026-113169, CVE-2026-113171, CVE-2026-113186, CVE-2026-113189, CVE-2026-113211, CVE-2026-113222, CVE-2026-113235, CVE-2026-113252, CVE-2026-113261, CVE-2026-113263, CVE-2026-113270, CVE-2026-113283, CVE-2026-113300, CVE-2026-113328, CVE-2026-113336, CVE-2026-113342, CVE-2026-118305, CVE-2026-118308, CVE-2026-118309, CVE-2026-118310, CVE-2026-118312, CVE-2026-118316, CVE-2026-118321, CVE-2026-118332, CVE-2026-118340, CVE-2026-118342, CVE-2026-118355 | Google Chrome rendering/engine family (V8, ANGLE, Skia, Dawn, Compositing, FileSystem, Mojo, Updater, DevTools, sandbox escapes) — client-side execution in the victim's browser |
| CVE-2026-102304, CVE-2026-102306, CVE-2026-102308, CVE-2026-102309, CVE-2026-102316, CVE-2026-102322, CVE-2026-102331, CVE-2026-103626, CVE-2026-103628, CVE-2026-103630, CVE-2026-106197, CVE-2026-106211, CVE-2026-106227, CVE-2026-106239, CVE-2026-106281, CVE-2026-106298, CVE-2026-106323, CVE-2026-106358, CVE-2026-106382, CVE-2026-106419, CVE-2026-95277, CVE-2026-95281, CVE-2026-95283, CVE-2026-95284, CVE-2026-95299, CVE-2026-95310, CVE-2026-95311, CVE-2026-95313, CVE-2026-95318, CVE-2026-95325, CVE-2026-95329, CVE-2026-95331, CVE-2026-95347, CVE-2026-95349, CVE-2026-95356, CVE-2026-95339, CVE-2026-95350, CVE-2026-95357, CVE-2026-93372, CVE-2026-93373 | Google Chrome engine family, additional release trains (Views, ANGLE, Tint, WebGL, GPU, Fonts, FullScreen, Updater, sandbox escapes) — client-side execution |
| CVE-2026-130910, CVE-2026-130971, CVE-2026-130994, CVE-2026-130996, CVE-2026-131025, CVE-2026-131031, CVE-2026-131054, CVE-2026-131057, CVE-2026-131061, CVE-2026-131077, CVE-2026-131088 | Google Chrome 158.x family, fixed in 158.0.8127.36 (ANGLE, Tint, WebGL, GPU, Color, WebAppInstalls; several rated critical by cve.now) — client-side execution in the victim's browser |
| CVE-2026-130926, CVE-2026-130929, CVE-2026-130968, CVE-2026-131020, CVE-2026-131037, CVE-2026-131043 | Google Chrome 158.0.8127.36 batch 2 (Chromoting UAF ×2, Views UAF, Android WebView UAF, HID UAF, Sync confused deputy) — client-side execution in the victim's browser; several carry a social-engineering or renderer-compromise precondition |
| CVE-2026-118900 | Imperial Palaces Guide Android app command execution (victim's endpoint application) |
| CVE-2026-130121 | AI Puffer stored XSS in AI-generated comment replies (victim-browser execution) |
| CVE-2026-131800 | GitHub Enterprise Server stored XSS in custom properties (victim-browser execution) |
| CVE-2026-131875, CVE-2026-131876 | Iru fqterm terminal-decoder and SSH-client out-of-bounds flaws (victim's terminal client parsing server output) |
| CVE-2026-84388 | FortiPAM Chrome extension (client-side browser extension) |
| CVE-2025-24054 | Windows NTLM hash disclosure spoofing |
| CVE-2026-128116 | SvelteKit route-resolution XSS (victim's browser) |
| CVE-2026-68541 | PolicyPress CSS-only redaction and workflow defaults |
| CVE-2026-110199 | IBM ContextForge privileged Web UI cross-site scripting can expose credentials in a browser session |
| CVE-2025-71464, CVE-2025-71465 | Angular Editor XSS (toggleEditorMode / refreshView) — victim-browser execution |
| CVE-2026-110531 | FV Player stored XSS via popup shortcode — victim-browser execution |
| CVE-2026-116753 | Multi Step for Contact Form 7 stored XSS — victim-browser execution |
| CVE-2026-120687 | FooBox stored DOM XSS via image alt — victim-browser execution |
| CVE-2026-127115 | BuddyForms stored XSS in login shortcode — victim-browser execution |
| CVE-2026-129254 | Opixido OCMS admin-TUI XSS — victim interaction (crafted link) |
| CVE-2026-129251 | CloudSchool event-description XSS — victim interaction |
| CVE-2026-129430 | filesharing XSS in bundle description links — victim-browser execution |

## Category B — wire-protocol gaps

| Advisory | Protocol / surface |
| --- | --- |
| CVE-2026-129289 | PHP-FPM request smuggling through an unread `php://input` body on reused persistent FastCGI connections (front-end dependent — nginx `fastcgi_keep_conn`, Apache `enablereuse=on`; requires the app to open `php://input` without reading it) — the smuggled request crosses the FastCGI wire protocol, not an HTTP request boundary |
| CVE-2020-1350 | Windows DNS (SIGRED) |
| CVE-2026-107407, CVE-2026-107408, CVE-2026-107409, CVE-2026-107410, CVE-2026-120868 | Citrix NetScaler SMPP / DNS / DoH / Push / packet-processing |
| CVE-2026-114819, CVE-2026-114822, CVE-2026-114848 | Oracle Database Server network takeover, Oracle Text and RDBMS components (SQL*Net wire protocol; CVSS 9.9) |
| CVE-2026-114952 | Oracle MySQL Server / Cluster client-programs flaw (MySQL wire protocol; CVSS 9.1) |
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
| CVE-2026-126148 | FortiMail HA join-request packets |
| CVE-2026-106173 | FortiSIEM path traversal through crafted packets |
| CVE-2026-114713 | UnrealIRCd IRCv3 label injection across linked servers |
| CVE-2026-85746, CVE-2026-85748, CVE-2026-92262 | MariaDB spatial conversion, cursor handling, and table-name encoding through database operations |
| CVE-2026-128886 | D-Bus nonce-tcp authentication can stall new connections while waiting for a token |
| CVE-2026-129431, CVE-2026-129432 | Ettercap SOCKS5/NFS dissector memory flaws — passive capture tool, not fronted HTTP |
| CVE-2011-10042 | HTTP::Session for Perl passes an unfiltered session identifier to the Memcached text protocol. No fixture comparison was recorded. |

The [aiohttp CVE-2026-128131 case study](CVE_2026_128131.md) tested an HTTP request body size rule. WebSocket frames and client response decoding remain outside that test.

The [Apache mod_http2 case study for CVE-2026-119618](CVE_2026_119618.md) records a policy and module comparison on an HTTP/1.1 fixture. The HTTP/2 frame race in the affected Apache implementation remains outside that test.

## Category C — local execution and host libraries

| Advisory | Component |
| --- | --- |
| CVE-2021-4034 | polkit pkexec (local) |
| CVE-2026-130644 | Red Hat ansible-galaxy collection install path traversal (victim runs the installer on a malicious artifact) |
| CVE-2026-131871 | WLA DX wlalink stack overflow parsing local object files |
| CVE-2026-119469 | ASUS DisplayWidget Center incorrect default local permissions |
| CVE-2026-112115 | ASUS Armoury Crate local privilege escalation via untrusted search path |
| CVE-2026-112117 | ASUS System Control Interface local path traversal |
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
| CVE-2026-110936 | Siemens Software Center local DLL loading |
| CVE-2026-59346 | VMware Workstation and Fusion VMXNET3 guest-to-host code execution |
| CVE-2026-98464, CVE-2026-98490, CVE-2026-98548, CVE-2026-98549, CVE-2026-98959, CVE-2026-99078, CVE-2026-99079, CVE-2026-99999 | Linux kernel RISC-V KVM, OVS conntrack, SCTP, TCP, Btrfs, mlx5e, IPv4 (local kernel) |
| CVE-2026-98323, CVE-2026-98365, CVE-2026-100075, CVE-2026-97413, CVE-2026-93228, CVE-2026-93207 | Linux kernel RDMA siw/RXE, SRPT, RTRS, svcrdma, SUNRPC GSS (local kernel) |
| CVE-2026-87886 | Acronis cPanel/Plesk plugin insecure file permissions (local; exploited per CISA KEV) |
| CVE-2025-53698, CVE-2025-53699 | Johnson Controls iSTAR alternate physical hardware interfaces |
| CVE-2026-9661 | Postecert FIRMAOK! Windows OpenSSL search-path hijack |
| CVE-2026-5514 | IBM Bob session token exposure to in-process extensions |
| CVE-2026-115501 | IBM i QFileSvr.400 stack overflow through a local authenticated file-system command |
| CVE-2026-128134 | NLTK dot2img executes a Graphviz binary planted in a writable PATH directory |
| CVE-2025-53700 | Johnson Controls iSTAR insecure storage allows file manipulation after local access |
| CVE-2026-124536, CVE-2026-124539, CVE-2026-124540, CVE-2026-124541, CVE-2026-129113, CVE-2026-129114, CVE-2026-129115 | JSON-XS parser family (incr_parse, decode_hv/tag/str, underflow, FREEZE) — Perl library, decoding after arrival |
| CVE-2026-129255 | CoolerMaster MASTERPLUS+ WinRing0x64.sys driver — local Windows driver flaw |
| CVE-2025-57735 | Apache Airflow logout JWT not invalidated — session lifecycle semantics, no request signature |
| CVE-2026-86473 | Apache Airflow logout bearer not revoked — same logout-semantics family |
| CVE-2025-66236 | Apache Airflow security-model documentation advisory — deployment guidance, not a specific flaw |
| CVE-2025-68438 | Apache Airflow rendered-template display of oversized templated fields — display-side, no request signature |
| CVE-2025-68675 | Apache Airflow Connection proxy-field secret handling — stored-secret display, no request signature |
| CVE-2026-28779 | Apache Airflow session cookie path=/ scope — Set-Cookie semantics, not request-refusable |
| CVE-2026-30912 | Apache Airflow stack-trace exposure in API error responses — server-side error handling |
| CVE-2020-17526 | Apache Airflow session validation cross-site — session semantics |
| CVE-2026-33264 | Apache Airflow serialized DAG import_string RCE — arrives via scheduler DagBag processing (DAG-author code path), not a frontable HTTP route; control surface is the DAG repo |
| CVE-2026-45360 | Apache Airflow scheduler deadline-reference decoder — same DAG-author/scheduler path, not frontable HTTP |
| CVE-2024-39877 | Apache Airflow doc_md code execution in scheduler context — DAG-author code path |
| CVE-2026-68982 broker note | The direct forged-callback vector for CVE-2026-68982 rides the Redis/RabbitMQ Celery broker wire — a protocol gap; the HTTP callback-queueing surface is remediated in its case study |
| CVE-2020-11981 broker note | Same broker-wire gap for CVE-2020-11981; the API queueing surface is remediated in its case study |
| CVE-2026-129261 | Kingston FURY CTRL through 2.0.66.0 exposes a local driver IOCTL with insufficient access control. |

## Category D — application state, build output, and pending tests

| Advisory | Where the advisory applies |
| --- | --- |
| CVE-2026-129481 | PHP use-after-free after interrupted session shutdown (php 8.3–8.5 before 8.3.37/8.4.28/8.5.13; same php2shell release) — needs a session-cleanup callback to fail fatally (e.g. user stream-wrapper `stream_close`) with the script filename in request memory (OPcache off), then a same-worker later request; worker memory lifecycle, no HTTP request-boundary discriminator |
| CVE-2026-129474 | PHP magic-method backtrace use-after-free (php 8.5.x before 8.5.13) — needs off-default fatal-error backtraces with exception args kept, and the application to reach `__call`/`__callStatic` while a fatal error (e.g. memory_limit) lands; the UAF is in the worker's fatal handling, and a size-triggered refusal would blanket every large request — no surgical request-boundary discriminator |
| CVE-2026-129182 | PHP `last_fatal_error_backtrace` use-after-free after shutdown (php 8.5.x before 8.5.13) — needs per-path `max_memory_limit` or conflicting `zlib.output_compression`/output_handler settings and a same-worker diagnostic before executor init; same-worker lifecycle, not an HTTP request boundary |
| CVE-2026-123890 | VMware vCenter authentication bypass via the VMware Directory Service (vCenter 5.x/8.0/9.0.x/9.1.x) — the vendor publishes no request route or bypass shape; the flaw is in the appliance's internal directory-service authentication logic, not a rule an HTTP request boundary can evaluate |
| CVE-2026-95643, CVE-2026-95645 | ManageEngine DDI Central authenticated command injection (≤ 6.3.1 build 6301) — telnet connectivity feature `/ipam/telnet/` and the Windows DDI agent; attacker needs a high-privilege login, and the injected feature is a management capability that legitimately runs telnet commands — no unauthenticated request-boundary discriminator |
| CVE-2026-129496 | PHP timezone cache use-after-free via a startup diagnostic written through file-based `error_log` (startup/diagnostic path, then gadget chain — no HTTP request-boundary discriminator) |
| CVE-2026-129291 | PHP-FPM epoll use-after-free local privilege escalation (`catch_workers_output` + epoll backend; local access on the FPM host, not an HTTP request boundary) |
| CVE-2026-127960 | PHP folded HTTP response-header denial of service through stream fetches (`file_get_contents`/`fopen` on an attacker URL; response-side parsing of the upstream reply, not the inbound request boundary) |
| CVE-2026-128064 | PHP mixed-case HTTPS/FTPS URL plaintext exposure (outbound URL handling in the stream layer; exposure depends on the application's outbound fetches, not the inbound request boundary) |
| CVE-2026-128090 | PHP PHAR OpenSSL signature bypass via algorithm downgrade (requires delivering a crafted signed PHAR to a verification path; no general HTTP request-boundary discriminator) |
| CVE-2026-128118 | SvelteKit build guard lets server-only imports enter the browser bundle |
| GHSA-87fm-7w84-j2px | Ruby case-conversion **integer overflow** (same release as ruby2shell). Giant-value trigger; no boundary discriminator at HTTP sizes, and the sharp-s module does not cap sizes. Revisit with the vendor PoC. |
| GHSA-7x2q-68qv-6p8x | Ruby Regexp compilation **double free** — triggered by compiling a crafted pattern; pattern compilation is application-controlled, no general request-boundary discriminator. Revisit when a default-config vector is published. |
| GHSA-3p8q-wmhx-vm2w | Ruby Regexp compilation **heap buffer overflow** — same application-controlled-pattern precondition as GHSA-7x2q. Revisit when a default-config vector is published. |
| GHSA-wpqw-5qxg-m68r | Ruby **out-of-bounds read** in case-insensitive regexp matching — attacker-crafted pattern or crafted match subject; no general request-boundary discriminator. Revisit when a default-config vector is published. |
| GHSA-7whj-cfh7-xgm4 | Ruby `Regexp#to_s` **heap buffer overflow** — converting a crafted pattern to a string; application-controlled pattern again. Revisit when a default-config vector is published. |
| GHSA-pvgc-g7gj-pv6h | Ruby **out-of-bounds read** in case-insensitive backreference matching — matching a crafted string; no general request-boundary discriminator. Revisit when a default-config vector is published. |
| CVE-2026-128121 | SvelteKit prerender crawler emits unescaped paths into generated routing rules |
| CVE-2026-128122 | SvelteKit prerender crawler can inject rules into generated Cloudflare redirects |
| CVE-2026-128117, CVE-2026-128123 | SvelteKit adapter-node crashes during route decoding or request handling. No fixture comparison was recorded. |
| CVE-2026-128100 | Vault authorization depends on deployed ACLs and case-insensitive identity handling |
| CVE-2026-68551 | Budget Board account linking depends on validation of the identity provider's verified-email claim |
| CVE-2026-102489 | Zammad session hijacking leading to code execution. No fixture comparison was recorded. |
| CVE-2026-105445 | Docker Hub account conversion and personal access token reassignment |
| CVE-2026-96012 | IBM Db2 Mirror for i privilege escalation. No fixture comparison was recorded. |
| CVE-2026-89078, CVE-2026-93577 | GitLab CI/CD regex parsing and compilation. The [related case study](CVE_2026_97229.md) documents a different CVE and a separately tested rule. No fixture comparison was recorded for these two advisories. |
| CVE-2026-129173 | Stirling-PDF workflow participant details session timeout |
| CVE-2026-110189 | ContextForge jq filter can terminate workers or exhaust resources |
| CVE-2026-110195 | ContextForge token exchange cache can reuse upstream credentials across credential contexts |
| CVE-2026-110278 | ContextForge identity claim signature replay or disclosure during tool invocation |
| CVE-2026-94407 | ContextForge regular expression complexity in a tool schema |
| CVE-2026-109158, CVE-2026-114468 | IBM App Connect disclosure through logged errors and administrative HTTP calls from inside the running container |
| CVE-2026-115788 | IBM App Connect log forging through resource creation |
| CVE-2026-91868, CVE-2026-91869 | IBM QRadar EDR information disclosure and inadequate TOTP verification rate limiting |
| CVE-2026-129429 | Frappe india-payroll tax-regime methods lack employee-level authorization. No fixture comparison was recorded. |
| CVE-2026-17176 | TP-Link Deco BE11000 / M9 Plus TDDP module — binary device protocol, not frontable HTTP |
| CVE-2026-86218 | N-able N-central RMM unauthenticated RCE — vendor route not published; revisit when disclosed |
| CVE-2026-101113 | ELECOM WRC router OS command injection in the admin interface — JVN and vendor do not publish the injection parameter; blanket admin denial is not a CVE-specific control; revisit when disclosed |
| CVE-2026-126574 | ASUS Control Center Datacenter/Enterprise password-update escalation from Viewer role — vendor route and request shape not published; the boundary cannot evaluate server-side roles; revisit when disclosed |
| CVE-2026-114202, CVE-2026-114204 | ASUS Control Center Express hard-coded key forged authentication/session tokens — vendor does not publish the token format or key material, so no request-boundary discriminator exists; revisit when disclosed |
| CVE-2026-114205 | ASUS Control Center Express low-privilege administrator-account creation — vendor route not published; the boundary cannot evaluate the creator's server-side role; revisit when disclosed |
| CVE-2026-81578 | PaperCut MF/NG web auth bypass — vendor bypass conditions not published; revisit when disclosed |
| CVE-2026-19490 | NetScaler ADC/Gateway delivery-controller flaw — vendor bulletin without HTTP mechanics |
| CVE-2026-73570 | Zimbra SNMP monitoring RCE — SNMP wire protocol, not frontable HTTP |
| CVE-2026-71362 | Adobe Commerce ACL privilege escalation — vendor route not published |
| CVE-2026-18577 | N-able N-central admin takeover — vendor route not published; revisit when disclosed |
| CVE-2026-20316 | Cisco Secure FMC hardcoded account — credential issue, no generic request signature |
| CVE-2026-63077 | JetBrains TeamCity agent-polling RCE — custom binary protocol on agent port |
| CVE-2026-16232 | Check Point SmartConsole auth bypass via app token — vendor mechanics not published |
| CVE-2026-15410 | SonicWall SMA1000 AMC post-auth code execution — vendor route not published |
| CVE-2026-55040, CVE-2026-58644, CVE-2026-45484, CVE-2026-45659, CVE-2026-33112, CVE-2026-32201, CVE-2026-20945 | Microsoft SharePoint family — server-side .NET/deserialization flaws without published HTTP routes; client-server wire internals |
| CVE-2026-45502 | Microsoft Exchange SSRF — authenticated server-side path, route not published |
| CVE-2026-43825 | Apache OpenNLP LibSVM deserialization — JVM library, decoding after arrival |
| CVE-2026-50656 | Microsoft Defender MMPE — local engine flaw |
| CVE-2026-20262 | Cisco SD-WAN Manager arbitrary file write — vendor route not published |
| CVE-2026-48558 | SimpleHelp OIDC JWT signature verification — token semantics inside the flow, not request-visible |
| CVE-2026-44492 | Axios shouldBypassProxy IPv4-mapped IPv6 — client library logic |
| CVE-2026-35273 | Oracle PeopleSoft Updates Environment Management — vendor route not published |
| CVE-2026-20251 | Splunk Enterprise deserialization RCE — vendor component route not published |
| CVE-2026-53435 | Jenkins core deserialization — build data path, not fronted HTTP |
| CVE-2026-49160, CVE-2026-47291 | Microsoft HTTP.sys DoS / RCE — kernel driver, wire internals |
| CVE-2026-50508 | Windows NTLM spoofing — authentication protocol internals |
| CVE-2026-45657 | Windows 11 kernel RCE — local kernel |
| CVE-2026-10727, CVE-2026-25089, CVE-2026-10523, CVE-2026-10520 | Ivanti EPMM / Sentry / FortiSandbox — vendor routes not published; revisit when disclosed |
| CVE-2026-50751 | Check Point VPN IKEv1 authentication bypass — VPN key-exchange protocol |
| CVE-2026-20245 | Cisco SD-WAN Controller CLI privilege escalation — authenticated local CLI |
| CVE-2026-20230 | Cisco UC Manager SSRF — vendor route not published |
| CVE-2026-0826 | Poly Voice device remote control — binary device protocol |
| CVE-2026-49373 | JetBrains TeamCity Perforce settings RCE — vendor route not published |
| CVE-2026-34650, CVE-2026-34648, CVE-2026-34649 | Adobe Commerce resource consumption — no CVE-specific request signature |
| CVE-2026-41089 | Windows Netlogon RCE — SMB/netlogon wire protocol |
| CVE-2026-43500 | Linux kernel rxrpc — local kernel |
| CVE-2026-44338 | PraisonAI legacy API server — generated per-deployment, no stable route |
| CVE-2026-44127 | LFI and arbitrary file deletion — vendor not published |
| CVE-2026-7864 | SEPPmail info exposure — vendor route not published |
| CVE-2026-25874 | Hugging Face LeRobot gRPC inference deserialization — gRPC wire protocol |
| CVE-2026-31431 | Linux kernel algif_aead — local kernel |
| CVE-2026-40372 | ASP.NET Core signing — framework internals, route not published |
| CVE-2026-33824 | Windows IKE service RCE — IKE wire protocol |
| CVE-2026-9256, CVE-2026-42945 | NGINX ngx_http_rewrite_module heap overflow — parser internals below the request view |
| CVE-2026-23918 | Apache HTTP Server http2 double free — HTTP/2 framing, wire internals |
| CVE-2026-48172 | LiteSpeed cPanel plugin local Redis file — local privilege path |
| CVE-2026-47323 | Apache Camel CXF/Knative header injection — framework integration internals |
| CVE-2026-41948 | Dify plugin daemon traversal — tenant-path semantics inside the daemon API |
| CVE-2026-2652 | mlflow FastAPI auth bypass — server startup configuration, no request signature |
| CVE-2026-42897 | Microsoft Exchange XSS — victim interaction |
| CVE-2026-0257 | PAN-OS GlobalProtect auth bypass — VPN protocol flow |
| CVE-2026-129262 | Wubang Five-in-One Safety Production Management Information System through 3.5 allows remote unrestricted uploads in DiyFileUpload on /ws.asmx. HTTP enforcement has not been evaluated. |

## Recording status

Stage reached for every entry above: **triaged against the 1.103.0 HTTP/1 interlay; no fixture, policy, or module was recorded, and none is claimed.** Entries move out of this document only when a live case study exists (see the [case study index](README.md)). Protocol entries additionally track the adapter backlog; when a wire adapter ships and is qualified, the corresponding advisories return to the actionable queue.
