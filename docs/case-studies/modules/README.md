# Native module sources

Each file uses the advisory ID in its path and the patch title in the source. The original snapshot supports the historical measurements in the report.

<<<<<<< HEAD
| Patch title | Module source | Case study | Current source SHA-256 |
| --- | --- | --- | --- |
| Realtek Miniigd NewInternalClient Guard | [`CVE-2014-8361.rs`](CVE-2014-8361.rs) | [CVE-2014-8361](../CVE_2014_8361.md) | `54a4c182ddcd5dcd888eae6f184965fd2a008568971f83933c3b05fbb0b9f569` |
| WP e-Commerce Shop Styling Download Path Guard | [`CVE-2015-5468.rs`](CVE-2015-5468.rs) | [CVE-2015-5468](../CVE_2015_5468.md) | `1369bc35e66cfe87bd2198de02ebfbaf5746313b0aacd70fbb9a37b82e3ea016` |
| WordPress Newspaper Theme Options Guard | [`CVE-2016-10972.rs`](CVE-2016-10972.rs) | [CVE-2016-10972](../CVE_2016_10972.md) | `0ddbb70e9d26ac8e3f87ac48e587328e6406b83b22caa09b49d6294c74afd5c8` |
| Intel AMT Digest Authentication Guard | [`CVE-2017-5689.rs`](CVE-2017-5689.rs) | [CVE-2017-5689](../CVE_2017_5689.md) | `78e3df4aae44ca42fdc4f46a197d76d5d29500270584e0c70eaa93e40badd9c0` |
| Linear eMerge E3 Command Injection Guard | [`CVE-2019-7256.rs`](CVE-2019-7256.rs) | [CVE-2019-7256](../CVE_2019_7256.md) | `b44654aa37311df65fe98b7a7f2f45facf2f73da2f3fc8015fb82480b3bdaf49` |
| Simple File List Executable Rename Guard | [`CVE-2020-36847.rs`](CVE-2020-36847.rs) | [CVE-2020-36847](../CVE_2020_36847.md) | `7919c1b64219f5fbef01dffb8ecae15b8b1687c2c772652b3f3e2a357a72b744` |
| ExifTool DjVu Input Guard | [`CVE-2021-22204.rs`](CVE-2021-22204.rs) | [CVE-2021-22204](../CVE_2021_22204.md) | `7442a55bc93d903c67aa7cb17939454a06cb4ff6036bd238f81b51d195328e65` |
| Grafana Plugin Path Traversal File Read Guard | [`CVE-2021-43798.rs`](CVE-2021-43798.rs) | [CVE-2021-43798](../CVE_2021_43798.md) | `31001f13bf73ba3778a2c82e243369ea91af69dc3f3e691b6715376b984e9150` |
| CuppaCMS URL Parameter Guard | [`CVE-2022-25486.rs`](CVE-2022-25486.rs) | [CVE-2022-25486](../CVE_2022_25486.md) | `2de4fbafc0e830ec605a0a3f37f3301a791bb4a5a1e0df453159d08385140f69` |
| Strapi Admin Query Filter Guard | [`CVE-2023-22894.rs`](CVE-2023-22894.rs) | [CVE-2023-22894](../CVE_2023_22894.md) | `5c654e3b9ee5e3bf37c6f23bbf3d9a40b95a2e4b97512d48956731bfb606bc2f` |
| Superset Session Forgery Guard | [`CVE-2023-27524.rs`](CVE-2023-27524.rs) | [CVE-2023-27524](../CVE_2023_27524.md) | `fb87253534417af7b713c182d50676458fb7dafe68eb769a0dfbc8c1aa84ecf2` |
| ASUS RT-AX55 Command Injection via Start Apply Guard | [`CVE-2023-39780.rs`](CVE-2023-39780.rs) | [CVE-2023-39780](../CVE_2023_39780.md) | `40f4da176fc0313bd2c4d702ab3e2aa94cdc7fc7c0a3b05e64a368f15384afd9` |
| Tinyproxy Connection Header Guard | [`CVE-2023-49606.rs`](CVE-2023-49606.rs) | [CVE-2023-49606](../CVE_2023_49606.md) | `56855dc6d64cab8efbae93be291d36884b355d22abfc0a6619630f6156ae42ab` |
| eXtplorer Login Bypass and PHP Upload Guard | [`CVE-2023-54335.rs`](CVE-2023-54335.rs) | [CVE-2023-54335](../CVE_2023_54335.md) | `e64accf05196b57e8eaba3b39453e10bab30a1a92e1aa1685a09351a2c893992` |
| MLflow Artifact Path Traversal Guard | [`CVE-2025-11201.rs`](CVE-2025-11201.rs) | [CVE-2025-11201](../CVE_2025_11201.md) | `ed7d81e88b047e67a9936ae77d26e4283d56b820a33c973d0809fc39861e0c75` |
| Wazuh Server API Deserialization Guard | [`CVE-2025-24016.rs`](CVE-2025-24016.rs) | [CVE-2025-24016](../CVE_2025_24016.md) | `a39ca78a9700b4906d32062ed6b09de37d8083226086936315cc95cff48ea1e5` |
| TOTOLINK A810R Command Injection Guard | [`CVE-2025-28137.rs`](CVE-2025-28137.rs) | [CVE-2025-28137](../CVE_2025_28137.md) | `0a481617a1502b0380623aedafeb2639f4c22578e80f13a117341f7a8c09aa53` |
| CrushFTP Admin Takeover via Session Injection Guard | [`CVE-2025-31161.rs`](CVE-2025-31161.rs) | [CVE-2025-31161](../CVE_2025_31161.md) | `6a9bb8e2e7175ab1124fd4473375d678b40bc01d58b3af2b12538185ac50bc41` |
| Craft CMS Remote Code Execution Guard | [`CVE-2025-32432.rs`](CVE-2025-32432.rs) | [CVE-2025-32432](../CVE_2025_32432.md) | `cf8055c665452544deb427300f754d1632b452fa92e5d0a2d2eaa15ebd979b61` |
| Langflow Code Execution in the Validation API Guard | [`CVE-2025-3248.rs`](CVE-2025-3248.rs) | [CVE-2025-3248](../CVE_2025_3248.md) | `cceb4d9235f33a1cd0aeec5773e14c999a72f61bf125813e84a17a20281bcccc` |
| Johnson Controls FX Server File Access Guard | [`CVE-2025-43867.rs`](CVE-2025-43867.rs) | [CVE-2025-43867](../CVE_2025_43867.md) | `dd4f00dcf6773f21331f2debd4cde76f99e71bf26f1e6814a49029962f67f6bc` |
| Johnson Controls iSTAR Command Input Guard | [`CVE-2025-43874.rs`](CVE-2025-43874.rs) | [CVE-2025-43874](../CVE_2025_43874.md) | `1fb6b23876eeaab6131c82fe69d6b32ce45ffd45db44062dc039ebb69289c91a` |
| Apache bRPC Heap Service Command Guard | [`CVE-2025-60021.rs`](CVE-2025-60021.rs) | [CVE-2025-60021](../CVE_2025_60021.md) | `839279dae3d3018b4b94a1fb9456f728bfa07f3a6b64f055f6e6a03347b4a91b` |
| FortiOS CLI File Path Guard | [`CVE-2025-61624.rs`](CVE-2025-61624.rs) | [CVE-2025-61624](../CVE_2025_61624.md) | `adfc19e0d80958f4d3c82407286eff81f63b364552a04a073fcec094f6fbb238` |
| FUXA Referer Authentication Guard | [`CVE-2025-69985.rs`](CVE-2025-69985.rs) | [CVE-2025-69985](../CVE_2025_69985.md) | `5441554e949734d40a9456de2750973d00f8f58afd8c5d9c4fd350e5104de55a` |
| PAN-OS GlobalProtect Captive Portal Guard | [`CVE-2026-0300.rs`](CVE-2026-0300.rs) | [CVE-2026-0300](../CVE_2026_0300.md) | `ac62a806b0855c8c51d1f1982e3335d6ba1b04d38872f8565fcb375995ed5168` |
| PAN-OS Management Interface Root Code Execution Guard | [`CVE-2026-0322.rs`](CVE-2026-0322.rs) | [CVE-2026-0322](../CVE_2026_0322.md) | `f4d4f6096e93857c0be340273bee318a16e2f124f42da108a5c05942ced4ab51` |
| Ideon IPv6 Transition Address Guard | [`CVE-2026-103980.rs`](CVE-2026-103980.rs) | [CVE-2026-103980](../CVE_2026_103980.md) | `05521fb9e8e5409dd471240fb7026507714b0dd2ba0ed355dbe3dcee614cc215` |
| AdonisJS ReDoS via Uncontrolled Regex Complexity Guard | [`CVE-2026-104150.rs`](CVE-2026-104150.rs) | [CVE-2026-104150](../CVE_2026_104150.md) | `4f88bf5a0628e54acfb9a4051f74e64761b3a24d0d58d387aae1d22dd791d7e1` |
| FortiMail File Write via Web Request Traversal Guard | [`CVE-2026-104286.rs`](CVE-2026-104286.rs) | [CVE-2026-104286](../CVE_2026_104286.md) | `cef35386609b0db6b5e0b7bdc3db03f267845e91ab1b68828cf6eb51aea29e53` |
| Ivanti Connect Secure Certificate Guard | [`CVE-2026-104935.rs`](CVE-2026-104935.rs) | [CVE-2026-104935](../CVE_2026_104935.md) | `482e0520e805172771efbdc83b2a6b121cf400980fadcc1dfe7099de7add2d97` |
| AhsayCBS API Password Check Guard | [`CVE-2026-105133.rs`](CVE-2026-105133.rs) | [CVE-2026-105133](../CVE_2026_105133.md) | `5e6022b68c0c268af4ca12383af6ea57cc9654ac1e4d3772c18e835c53fa92bb` |
| AhsayCBS Replication Receiver Command Guard | [`CVE-2026-105134.rs`](CVE-2026-105134.rs) | [CVE-2026-105134](../CVE_2026_105134.md) | `502f4c420f956cb7a40fb216a0d5b824853fd98f07316351788e96bdefafcb4a` |
| Xtraction Connector Redirect Guard | [`CVE-2026-106042.rs`](CVE-2026-106042.rs) | [CVE-2026-106042](../CVE_2026_106042.md) | `172b5b13989a22f2f9c1f21a10104b4b1a5a00646165e0fc7b2e55ce3282e3fe` |
| SiteGuard Server Command Input Guard | [`CVE-2026-106931.rs`](CVE-2026-106931.rs) | [CVE-2026-106931](../CVE_2026_106931.md) | `06880c2bc6368c69bd70c8777e107b3afd051d405d26870a10259455544691cf` |
| NetScaler Gateway Request Guard | [`CVE-2026-107412.rs`](CVE-2026-107412.rs) | [CVE-2026-107412](../CVE_2026_107412.md) | `264f9538225591200bd2294e06ce211ec1d65b6b1dc1b36666dbf05ddd2d807a` |
| NetScaler Email OTP Request Guard | [`CVE-2026-107415.rs`](CVE-2026-107415.rs) | [CVE-2026-107415](../CVE_2026_107415.md) | `f854e47a87f1ca550fc3e6667b8710a083aa4ff27aedccc27cd7dbf50861183f` |
| FortiSandbox Alternate Route Authentication Guard | [`CVE-2026-107656.rs`](CVE-2026-107656.rs) | [CVE-2026-107656](../CVE_2026_107656.md) | `f06f186db0fae8f90d3ae47c777771558e79214c72e101816223fa51a8f255ae` |
| AdonisJS Route Redirect Guard | [`CVE-2026-107718.rs`](CVE-2026-107718.rs) | [CVE-2026-107718](../CVE_2026_107718.md) | `11110a82de8a48a8e445953376649298ad501bd201b76a2049dd65a252dc8dc6` |
| ContextForge Public Tool Ownership Guard | [`CVE-2026-110178.rs`](CVE-2026-110178.rs) | [CVE-2026-110178](../CVE_2026_110178.md) | `244bcb43f395d3a78c52ef55f338382599f59098402cf43958bbed6fec540746` |
| ContextForge Tool URL Guard | [`CVE-2026-110194.rs`](CVE-2026-110194.rs) | [CVE-2026-110194](../CVE_2026_110194.md) | `51e44189fcd68d638c9c3dfbbe5eb9b1b3f74619546c7e7f0d5bbfc5861a2cda` |
| GitLab Regex Compiler Input Guard | [`CVE-2026-110264.rs`](CVE-2026-110264.rs) | [CVE-2026-110264](../CVE_2026_110264.md) | `562688b9a7eddd33abe1722262d6a4637b7edf781078cfdf7f74f01371904752` |
| ContextForge Upstream Authorization Header Guard | [`CVE-2026-110279.rs`](CVE-2026-110279.rs) | [CVE-2026-110279](../CVE_2026_110279.md) | `34696af8ffca123a389144d52dd6bbf01859662f3e3216349812ae9aea64bb11` |
| Collabora CODE Proxy Command Guard | [`CVE-2026-110447.rs`](CVE-2026-110447.rs) | [CVE-2026-110447](../CVE_2026_110447.md) | `4838e8de5edeb631758241c8cfad4616e2d9524c2b0e8da81bdaa8a7dd484a5d` |
| Red Hat Quay Robot List SQL Guard | [`CVE-2026-110697.rs`](CVE-2026-110697.rs) | [CVE-2026-110697](../CVE_2026_110697.md) | `4f4832108131dc7667b0c6b35622dd369ada3069c441cbc57f5711a4c7a44d7a` |
| pfSense QinQ Interface Name Guard | [`CVE-2026-111214.rs`](CVE-2026-111214.rs) | [CVE-2026-111214](../CVE_2026_111214.md) | `e70ec117cb467922bb987c53d97841eae78b2d9be9e1079bb1c56f365d2a5337` |
| NetScaler Endpoint Analysis Request Guard | [`CVE-2026-111334.rs`](CVE-2026-111334.rs) | [CVE-2026-111334](../CVE_2026_111334.md) | `ad2b9a2546d690a3a7d12e66f1007258448b9b36c9e62223d1acd5d5b7f1e025` |
| Bundled Brotli Stream Guard | [`CVE-2026-111497.rs`](CVE-2026-111497.rs) | [CVE-2026-111497](../CVE_2026_111497.md) | `30fb523ddac5a0184d9f56da83e131595dfc1f5265ae862528d4eac0dd22436a` |
| Django REST Charset Guard | [`CVE-2026-114136.rs`](CVE-2026-114136.rs) | [CVE-2026-114136](../CVE_2026_114136.md) | `0d8633c349338813521e8ca119bf92c254f31e1475a179332adaea5d67e68296` |
| Laravel TokenGuard Credential Type Guard | [`CVE-2026-114509.rs`](CVE-2026-114509.rs) | [CVE-2026-114509](../CVE_2026_114509.md) | `377245deec517fc3a90f01414f3298dfab41b307206d752e780f0e2b7b95fdcd` |
| pgAdmin Query Export Command Guard | [`CVE-2026-116241.rs`](CVE-2026-116241.rs) | [CVE-2026-116241](../CVE_2026_116241.md) | `1f72f396c5e785e1c18b13433f146ad86ce00c3fbdaca778e771f7a571fd7047` |
| Ivanti Connect Secure Predictable Upload Guard | [`CVE-2026-118527.rs`](CVE-2026-118527.rs) | [CVE-2026-118527](../CVE_2026_118527.md) | `2a674cdd326daeb72488a14800b0f5e40c60fbc09cf54e368f98a73fd8745d4e` |
| WordPress Upgrade Staging Path Guard | [`CVE-2026-119977.rs`](CVE-2026-119977.rs) | [CVE-2026-119977](../CVE_2026_119977.md) | `c48d246c230c6690a7b7ea07a93d0e7bfc3da1dd26993fa7c039643175286d61` |
| Fluent Forms Cloud Storage Path Guard | [`CVE-2026-120522.rs`](CVE-2026-120522.rs) | [CVE-2026-120522](../CVE_2026_120522.md) | `e0b5a7e1a3851ce0ebfcfc44417a585948af810afee4202819215e49c71b2cc3` |
| NetScaler Console Authentication Bypass Guard | [`CVE-2026-120869.rs`](CVE-2026-120869.rs) | [CVE-2026-120869](../CVE_2026_120869.md) | `81cd6d2140e7bf76340d82365ccadd157871054ee66d71c911d632de57b04096` |
| Ivanti EPM Core API SQL Guard | [`CVE-2026-121653.rs`](CVE-2026-121653.rs) | [CVE-2026-121653](../CVE_2026_121653.md) | `1f264f13dbd5a177ac931f715df516838db5b5164b77f257dfbd995a3c41c91e` |
| webMethods Integration Server Environment Guard | [`CVE-2026-123628.rs`](CVE-2026-123628.rs) | [CVE-2026-123628](../CVE_2026_123628.md) | `91f81d425d836b85b4b4383a6fedd26c67b93b31c94463df566a5f6d299dd902` |
| Pappl-retrofit Directory Traversal in Add-ppdfi Guard | [`CVE-2026-123747.rs`](CVE-2026-123747.rs) | [CVE-2026-123747](../CVE_2026_123747.md) | `2c120e1378a065140c4fc6b8a298221fa266983fa3d53f7450307628ed123065` |
| ComfyUI-Easy-Use Path Traversal Guard | [`CVE-2026-123790.rs`](CVE-2026-123790.rs) | [CVE-2026-123790](../CVE_2026_123790.md) | `ff0766bac1e025005e442869ce0acc21edbb1de5ff59404d4cd7d509f6dd8710` |
| CWP Password Reset Host Header Guard | [`CVE-2026-124020.rs`](CVE-2026-124020.rs) | [CVE-2026-124020](../CVE_2026_124020.md) | `1b843ad171948058a31a3699f6b8796736e04d339d605e74ccbf22e3282bdd08` |
| CWP Authorization Bypass in the Remote API Guard | [`CVE-2026-124021.rs`](CVE-2026-124021.rs) | [CVE-2026-124021](../CVE_2026_124021.md) | `0c4e63b2017a7fb757e1c8ba5d421148227ff3f8d536a99d48ff302de16ae0c4` |
| Kendo UI Gantt Parent Dropdown Guard | [`CVE-2026-124691.rs`](CVE-2026-124691.rs) | [CVE-2026-124691](../CVE_2026_124691.md) | `3dba52f02f822109a8e64c4f5005f6ea1f974d98c3a7c55595a1507a0916975f` |
| Kendo UI for jQuery XSS in Form Item Hints Guard | [`CVE-2026-124695.rs`](CVE-2026-124695.rs) | [CVE-2026-124695](../CVE_2026_124695.md) | `76f68a32006903a022ab3890bd77f7a9744fdf7f7bf3378202eaa550d1349d4f` |
| Kendo UI Spreadsheet Sheet Name Guard | [`CVE-2026-124696.rs`](CVE-2026-124696.rs) | [CVE-2026-124696](../CVE_2026_124696.md) | `57e9111096b10f528ede50b45a4660942289f07da87b385e20d60c54b8b4035d` |
| Kendo UI Chart Shared Tooltip Guard | [`CVE-2026-124697.rs`](CVE-2026-124697.rs) | [CVE-2026-124697](../CVE_2026_124697.md) | `731320a2d0b1ba88da9b798147bd3bb2ffb0412609f4c2a43fb78fe70edaf54e` |
| Kendo UI Chart Default Tooltip Guard | [`CVE-2026-124698.rs`](CVE-2026-124698.rs) | [CVE-2026-124698](../CVE_2026_124698.md) | `f12d9ed159e014cff883eb95cdf768179dc4e54e6782b2c8f4b70e1dfd3a5b27` |
| Windchill Deserialization Guard | [`CVE-2026-12569.rs`](CVE-2026-12569.rs) | [CVE-2026-12569](../CVE_2026_12569.md) | `316a6e76cc5fe2230370c3bad76e3e91b2069c0395581ab90812469cfa6f336f` |
| FortiMail Path Traversal to Code Execution Guard | [`CVE-2026-126147.rs`](CVE-2026-126147.rs) | [CVE-2026-126147](../CVE_2026_126147.md) | `41a3a01e9bbee874e2702b39e71292e092dd4f377a6c9828540dd7ae2d4fd4ec` |
| FortiMail IBE Password Recovery Guard | [`CVE-2026-126150.rs`](CVE-2026-126150.rs) | [CVE-2026-126150](../CVE_2026_126150.md) | `68d26c42e3d4734fdfe7a20d3be6704475c07162e4d28063a27bdd5af7db6fed` |
| FortiMail IBE Session Authorization Guard | [`CVE-2026-126155.rs`](CVE-2026-126155.rs) | [CVE-2026-126155](../CVE_2026_126155.md) | `836f90d2ecb772e5a24ddbabb36f2761e190e84a88f69d5359c45b219c5d3a34` |
| FortiMail IBE Account Key Guard | [`CVE-2026-126156.rs`](CVE-2026-126156.rs) | [CVE-2026-126156](../CVE_2026_126156.md) | `b4aa1d927a86b98d42e8270c1063d17ad89323a2c8b8879cc9acc3fa5f25a4f3` |
| FortiMail Secure Email Account Access Guard | [`CVE-2026-126157.rs`](CVE-2026-126157.rs) | [CVE-2026-126157](../CVE_2026_126157.md) | `fe98e5e40aea31bc210148fd10cffedded4241516ef0b5bd2e4b5e20d2e0b21a` |
| FortiMail Web Request SQL Guard | [`CVE-2026-126164.rs`](CVE-2026-126164.rs) | [CVE-2026-126164](../CVE_2026_126164.md) | `fedde3a6241c3c6cbf5fb5f6fd8ed0aec20078207cc49eeb0ff97541026a2345` |
| FortiManager and FortiAnalyzer Output Encoding Guard | [`CVE-2026-126170.rs`](CVE-2026-126170.rs) | [CVE-2026-126170](../CVE_2026_126170.md) | `03efe6013b7e4ab4596e336d97a0fcbd9811550222b2c0984f8d8178ee5bd23f` |
| FortiAnalyzer and FortiManager VDOM Access Guard | [`CVE-2026-126229.rs`](CVE-2026-126229.rs) | [CVE-2026-126229](../CVE_2026_126229.md) | `df1700de1d9355028baf4b0abaf6829d978e511fdae68d55dbb8609300b21a02` |
| FortiAuthenticator Authorization Route Guard | [`CVE-2026-126251.rs`](CVE-2026-126251.rs) | [CVE-2026-126251](../CVE_2026_126251.md) | `c57e0cfbf5c71c66fd2277885493c75c05aa6c3e53d0f193d11535878d35eeb2` |
| FortiAuthenticator Hardcoded Password Guard | [`CVE-2026-126257.rs`](CVE-2026-126257.rs) | [CVE-2026-126257](../CVE_2026_126257.md) | `dc596eca701bbf336f3e0dc385e93fcb6cd3e66421c1e1311e81feac5dd94421` |
| FortiMail Authenticated Request SQL Guard | [`CVE-2026-126263.rs`](CVE-2026-126263.rs) | [CVE-2026-126263](../CVE_2026_126263.md) | `2f58cadd689f0f66380da5d1229e4a052e04c66aa0e203e096e37e0c3fa97f9d` |
| OpsTree REMS Reports API Access Guard | [`CVE-2026-126768.rs`](CVE-2026-126768.rs) | [CVE-2026-126768](../CVE_2026_126768.md) | `c05ddfe7d42ed24ee94da5da40cd560f4e522c4b408ed596947acf05c2271042` |
| OpsTree REMS Config API Access Guard | [`CVE-2026-126769.rs`](CVE-2026-126769.rs) | [CVE-2026-126769](../CVE_2026_126769.md) | `5a1b0ca3a0cd6d3425df964321a796f26acb077acc570c5b234d9d259c610b9c` |
| Traccar Email Parameter Guard | [`CVE-2026-126777.rs`](CVE-2026-126777.rs) | [CVE-2026-126777](../CVE_2026_126777.md) | `384d0af232001aa96c2b82cbd14b564d7f5a7e43ddafb1a0cd1bd8c576a109d7` |
| Hydro Default UI Comment Guard | [`CVE-2026-126800.rs`](CVE-2026-126800.rs) | [CVE-2026-126800](../CVE_2026_126800.md) | `34ca571d6b52c8c2191d9f1b092e68ba29e59abeca8c0e4169d498c666ae6df2` |
| WWBN AVideo Path Traversal File Read Guard | [`CVE-2026-126804.rs`](CVE-2026-126804.rs) | [CVE-2026-126804](../CVE_2026_126804.md) | `28257dc6c59adfe74732a89302cc3b5bb6f9b47fb9dff2ee9ca0df62fc2add86` |
| WWBN AVideo Stored XSS via Video Name Guard | [`CVE-2026-126806.rs`](CVE-2026-126806.rs) | [CVE-2026-126806](../CVE_2026_126806.md) | `67cb1e488a55343c552e1c7518d0275f3f13ccb9d40d66689b5f0af4fb65c5fc` |
| JeecgBoot SSRF via Flow-run Endpoint Guard | [`CVE-2026-126997.rs`](CVE-2026-126997.rs) | [CVE-2026-126997](../CVE_2026_126997.md) | `ccb4a4dc5ceb3c0f6839d37e1eb0d5dac9facfbc2e5d9aa4dd8cd1ac8ce4af18` |
| JeecgBoot OpenAPI URL Guard | [`CVE-2026-126998.rs`](CVE-2026-126998.rs) | [CVE-2026-126998](../CVE_2026_126998.md) | `d6aa5c3a183e0db2ec0ea3b4045421f3e91c6dec58528a779c2448742d87f67c` |
| Alinto SOGo Password Recovery Brute Force Guard | [`CVE-2026-127111.rs`](CVE-2026-127111.rs) | [CVE-2026-127111](../CVE_2026_127111.md) | `227d271a4ce49793c8c983d25d9466e4804efaa4f7648b892a7aac2890000f76` |
| Alinto SOGo S/MIME Certificate Guard | [`CVE-2026-127121.rs`](CVE-2026-127121.rs) | [CVE-2026-127121](../CVE_2026_127121.md) | `d43177add3868871fda5a3cc5012f861596fd3942258610b987d0db80cc3576d` |
| Alinto SOGo Sieve Notification Guard | [`CVE-2026-127135.rs`](CVE-2026-127135.rs) | [CVE-2026-127135](../CVE_2026_127135.md) | `e943a62377c69b5b4020952e6560152c9206967e1c88b0f03a0760fe4790394a` |
| Alinto SOGo SSRF via Web Calendar Subscriptions Guard | [`CVE-2026-127145.rs`](CVE-2026-127145.rs) | [CVE-2026-127145](../CVE_2026_127145.md) | `fbf421c03883a3ecaa77aab49a1be906b86aeb2a944545894ca06665b9d797d0` |
| MLflow Log Outputs Model ID Guard | [`CVE-2026-127202.rs`](CVE-2026-127202.rs) | [CVE-2026-127202](../CVE_2026_127202.md) | `469020b95de4a4d3e154168b806e529791b097b724d123f6c87c7f0f6f759dde` |
| AppFlowy Cloud Blob Content Type Guard | [`CVE-2026-127203.rs`](CVE-2026-127203.rs) | [CVE-2026-127203](../CVE_2026_127203.md) | `1bf5bdb25e17f7afc147c458adebbf330fb3b1b8b173c83c06726671a1ab8f89` |
| LearnPress Wishlist Change via IDOR Guard | [`CVE-2026-127213.rs`](CVE-2026-127213.rs) | [CVE-2026-127213](../CVE_2026_127213.md) | `46e5f1b857072ea672919e5b1c35711458dd393fb052d40c832e894cd8e3b543` |
| Flowise Custom Function Module Subpath Guard | [`CVE-2026-127214.rs`](CVE-2026-127214.rs) | [CVE-2026-127214](../CVE_2026_127214.md) | `257c7b991a00129c9459e1e40bde1ba18647ff23fc18caebc9bb02c6bbe09e80` |
| Kanbanz Admin AJAX Access Guard | [`CVE-2026-127215.rs`](CVE-2026-127215.rs) | [CVE-2026-127215](../CVE_2026_127215.md) | `2f6d928b3320efb53b06f5e942cb1f6bf043c230ba616f085034bfdcf5d9c017` |
| Injection Guard Settings Request Guard | [`CVE-2026-127216.rs`](CVE-2026-127216.rs) | [CVE-2026-127216](../CVE_2026_127216.md) | `752e6ec71f8192c90e563ed98175f86a23e70fce7bc1ad29eeb6d418d198b882` |
| Post to Google My Business Fetch URL Guard | [`CVE-2026-127217.rs`](CVE-2026-127217.rs) | [CVE-2026-127217](../CVE_2026_127217.md) | `80cc3cd23f272e504f7f57ec07f9bbe28ccdc89186fb0b08c17b1efec597b3e7` |
| FrankenPHP Brotli Decoder Out-of-bounds Write Guard | [`CVE-2026-128014.rs`](CVE-2026-128014.rs) | [CVE-2026-128014](../CVE_2026_128014.md) | `b8b00ffac9e8aade896609ddaf23d013af19f80b10a5c3e8605d3aa3bd551295` |
| FrankenPHP Dot-header Trust Spoof in PHP Guard | [`CVE-2026-128023.rs`](CVE-2026-128023.rs) | [CVE-2026-128023](../CVE_2026_128023.md) | `3ca6ba233d5f3632dd57840106e8c0c5a09fd64652dbbbd95525fd96eca5a60e` |
| FrankenPHP Bundled Library Input Guard | [`CVE-2026-128027.rs`](CVE-2026-128027.rs) | [CVE-2026-128027](../CVE_2026_128027.md) | `670c187434fe26cb3d5b2b2b198f9b8613234212a1815a1ad9a2021093af0602` |
| SvelteKit Query Batch Argument Guard | [`CVE-2026-128119.rs`](CVE-2026-128119.rs) | [CVE-2026-128119](../CVE_2026_128119.md) | `f252782922e23cc93f6ccd54176817eaba00e4a5ae408a7f88d8b56aa431a976` |
| SvelteKit Form Data Guard | [`CVE-2026-128120.rs`](CVE-2026-128120.rs) | [CVE-2026-128120](../CVE_2026_128120.md) | `59d1ad150762983d87dab9b9cb0c637bf43a7c6b20fba82d2ba3e664e382d6eb` |
| aiohttp Host Header Mask Guard | [`CVE-2026-128124.rs`](CVE-2026-128124.rs) | [CVE-2026-128124](../CVE_2026_128124.md) | `e95f99cfe2358983d03671d392d3079c4249b478db98ee55a7e6ef970501a1da` |
| Aiohttp CookieJar Resource Exhaustion Guard | [`CVE-2026-128125.rs`](CVE-2026-128125.rs) | [CVE-2026-128125](../CVE_2026_128125.md) | `3180850542b4e6a1ac8736a86a21bcfcf397c39fcbd70abe6b5f852290adca99` |
| aiohttp Form Post Guard | [`CVE-2026-128126.rs`](CVE-2026-128126.rs) | [CVE-2026-128126](../CVE_2026_128126.md) | `9479f37f469945347f8a9856248b0c695b3c35ec05b69770df2698bd2389c992` |
| aiohttp Multipart Post Guard | [`CVE-2026-128127.rs`](CVE-2026-128127.rs) | [CVE-2026-128127](../CVE_2026_128127.md) | `561aa451e98e6928c8374603858806f511317cc9102eccb37357799da34acd8e` |
| aiohttp Python Parser Buffer Guard | [`CVE-2026-128130.rs`](CVE-2026-128130.rs) | [CVE-2026-128130](../CVE_2026_128130.md) | `be51f32c2233033cc5fdb554a10ce2bba9db4a6db0d4a95161ce2cb44028ee6f` |
| aiohttp Concatenated Stream Guard | [`CVE-2026-128131.rs`](CVE-2026-128131.rs) | [CVE-2026-128131](../CVE_2026_128131.md) | `cd75a26089d8aba9379b6929776d855220ef89461c034df614df85ecf391886e` |
| free-fs Password Reset Code Guard | [`CVE-2026-129161.rs`](CVE-2026-129161.rs) | [CVE-2026-129161](../CVE_2026_129161.md) | `8b4b97691ed0fd1d8aa3afb6ca5484955060d1c222ae9a5d4869dc6a1ec1b674` |
| Tactical RMM Report Preview File URL Guard | [`CVE-2026-129165.rs`](CVE-2026-129165.rs) | [CVE-2026-129165](../CVE_2026_129165.md) | `ff0daca9ab9d2eb7919360c0cadc3bfd2e1ba9d6f242daa111bbbb6c4c084671` |
| SeaDoc Absolute Path Traversal Guard | [`CVE-2026-129167.rs`](CVE-2026-129167.rs) | [CVE-2026-129167](../CVE_2026_129167.md) | `7d36a5f2217f6a14cb8917f6d2a113b823d49b59ee72ce47b4701c70cc429735` |
| NginxPulse SQL Injection on the Stats Route Guard | [`CVE-2026-129171.rs`](CVE-2026-129171.rs) | [CVE-2026-129171](../CVE_2026_129171.md) | `1e2fbbc6837b5ed4734c80ea3eb99420ba2bc474cf4e853767f1100895f9fb81` |
| Visitor Management System Action Page SQL Guard | [`CVE-2026-129180.rs`](CVE-2026-129180.rs) | [CVE-2026-129180](../CVE_2026_129180.md) | `b65568a2c9e7726983a66a93e28f7e7de1bb4cdec1b76e797e632ed90e17ca84` |
| Edimax BR6478ACV2 PPTP Username Guard | [`CVE-2026-129250.rs`](CVE-2026-129250.rs) | [CVE-2026-129250](../CVE_2026_129250.md) | `fc1a6b17bb3076bef5ab7a8563764259a78f306e7c65042e57f22b61733ce165` |
| Tenda AC18 Firmware Upgrade Request Guard | [`CVE-2026-129258.rs`](CVE-2026-129258.rs) | [CVE-2026-129258](../CVE_2026_129258.md) | `a44fafdde83180063d245f926e40da477c2d14fade7499ff700b10bae2a461e3` |
| VanBlog Stored XSS via the Image Upload Endpoint Guard | [`CVE-2026-129259.rs`](CVE-2026-129259.rs) | [CVE-2026-129259](../CVE_2026_129259.md) | `9efe1e4ccc1b2d998929347c91533815a44140f58c84c03133046e85cb0ee583` |
| VanBlog Custom Page Path Guard | [`CVE-2026-129260.rs`](CVE-2026-129260.rs) | [CVE-2026-129260](../CVE_2026_129260.md) | `e150efdb753d8e493dda40d4feaa23d1a10202d1169dd4f575a31aac4c20d7e4` |
| Pulse AI Approval Request Guard | [`CVE-2026-129428.rs`](CVE-2026-129428.rs) | [CVE-2026-129428](../CVE_2026_129428.md) | `97198af6291f737f4ac38f5d425ff7f4961f101bab6070d2ac916192bdce05cb` |
| Dokploy Backup Service Name Guard | [`CVE-2026-129433.rs`](CVE-2026-129433.rs) | [CVE-2026-129433](../CVE_2026_129433.md) | `b1dfee6641fa9d78642c989637366898216aec9cc1e89225360c901fc111ea55` |
| WGDashboard OS Command Injection Guard | [`CVE-2026-15733.rs`](CVE-2026-15733.rs) | [CVE-2026-15733](../CVE_2026_15733.md) | `d60a6a54502d4bb34d9f3918e03972272963b556190ac5747c845fb552cb2824` |
| MemberDash Subscriber Account Takeover via IDOR Guard | [`CVE-2026-18053.rs`](CVE-2026-18053.rs) | [CVE-2026-18053](../CVE_2026_18053.md) | `1892db8ceba4433910c9020a801fe96eb3f1365857a0ad9334028f092b648faa` |
| N-central Admin Authentication Guard | [`CVE-2026-18556.rs`](CVE-2026-18556.rs) | [CVE-2026-18556](../CVE_2026_18556.md) | `e07dcc86b5c2d69bd906955ca2be75c31a161f24c6f20bb5508b0e29c908d2c0` |
| Keycloak Reset Credentials Guard | [`CVE-2026-18963.rs`](CVE-2026-18963.rs) | [CVE-2026-18963](../CVE_2026_18963.md) | `d34ad01f172e91f122a7a5e0e3db15babdfc427ec72f3c6dd1ce709e78a1ae8b` |
| Shipo Login Cookie Guard | [`CVE-2026-19907.rs`](CVE-2026-19907.rs) | [CVE-2026-19907](../CVE_2026_19907.md) | `f4045e1603e0e6634e20379ecd49726b67c7dd24f18cebe45ae82ba2ea239522` |
| Cisco SD-WAN Administrative Takeover Guard | [`CVE-2026-20182.rs`](CVE-2026-20182.rs) | [CVE-2026-20182](../CVE_2026_20182.md) | `58c739ba27503db44a698a42238b57ea3bf36c0952aed812e107a8523dd4382f` |
| Splunk PostgreSQL Sidecar File Write Guard | [`CVE-2026-20253.rs`](CVE-2026-20253.rs) | [CVE-2026-20253](../CVE_2026_20253.md) | `d8957a01d0fb606643570cf39a76884c33d0016352ddbb635c039d825a707f24` |
| FortiClientEMS Client Request SQL Guard | [`CVE-2026-21643.rs`](CVE-2026-21643.rs) | [CVE-2026-21643](../CVE_2026_21643.md) | `3808c0a72e7bcf99bfbc20447290e829afceeb74339d909a997c181918ad4900` |
| XWiki SSX JSX Resource Path Guard | [`CVE-2026-23734.rs`](CVE-2026-23734.rs) | [CVE-2026-23734](../CVE_2026_23734.md) | `a8dca6fc64ada1324efd24d5a79bebe522c912b934f86c755524364364b8a335` |
| FUXA Scheduler Write Guard | [`CVE-2026-25939.rs`](CVE-2026-25939.rs) | [CVE-2026-25939](../CVE_2026_25939.md) | `166f37b10bb7992af22f8eff83c086cc0ddde54d162858390fbe5ab8ce510ead` |
| MajorDoMo RC Endpoint Command Guard | [`CVE-2026-27175.rs`](CVE-2026-27175.rs) | [CVE-2026-27175](../CVE_2026_27175.md) | `225db51c2a340cca7ec795e0c1ae3962c57fb4304cc99ebb9b3b89829ec14622` |
| ColdFusion File Read Path Guard | [`CVE-2026-27305.rs`](CVE-2026-27305.rs) | [CVE-2026-27305](../CVE_2026_27305.md) | `e9c7841f7bad61559694e41ab9291ab419928c85fef9e011f307e3a9d4534a4d` |
| OpenCATS Installer AJAX Payload Guard | [`CVE-2026-27760.rs`](CVE-2026-27760.rs) | [CVE-2026-27760](../CVE_2026_27760.md) | `17a9738343e2e7ccdb9e9e5d60c2bc14f7ceeeda2f45cbc984ba3c7a40528f05` |
| SolarWinds Serv-U Deflate Request Guard | [`CVE-2026-28318.rs`](CVE-2026-28318.rs) | [CVE-2026-28318](../CVE_2026_28318.md) | `3b523e24e8192a142378c436de70e78eed7da617d12118156ffbf3747d01cf4c` |
| Sonatype Nexus Task Component Guard | [`CVE-2026-3199.rs`](CVE-2026-3199.rs) | [CVE-2026-3199](../CVE_2026_3199.md) | `419e661a8c5266cfae606a6862ee958001a4e4bc3a148a7af16a35529b5a5c57` |
| UniFi OS Configuration Request Guard | [`CVE-2026-34908.rs`](CVE-2026-34908.rs) | [CVE-2026-34908](../CVE_2026_34908.md) | `c3faafff23e914086183486477405409bf02555693690e68b4fbde17a4558b90` |
| UniFi OS File Path Guard | [`CVE-2026-34909.rs`](CVE-2026-34909.rs) | [CVE-2026-34909](../CVE_2026_34909.md) | `4c7fc832c6a8530d08871466a83e758b191e777ab61eae3aadce6474698fe5dc` |
| UniFi OS Command Input Guard | [`CVE-2026-34910.rs`](CVE-2026-34910.rs) | [CVE-2026-34910](../CVE_2026_34910.md) | `0adcc771382a523cfbfc9eb9b3e27791e9a8a24a8827eb2405609beb5bf4e2f1` |
| Joomla Configuration Web Service Guard | [`CVE-2026-35223.rs`](CVE-2026-35223.rs) | [CVE-2026-35223](../CVE_2026_35223.md) | `df52231b4f1aa98a5b8c71c908f188d5d67326945db813e04a72df5ad94e2114` |
| MeiG FORGE SLT711 Remote Access Command Guard | [`CVE-2026-36356.rs`](CVE-2026-36356.rs) | [CVE-2026-36356](../CVE_2026_36356.md) | `57aee8bd78abd7479df09d7a7088b5e29505b4091f2b2405b20567d59b4a5d48` |
| Breeze Cache Upload Request Guard | [`CVE-2026-3844.rs`](CVE-2026-3844.rs) | [CVE-2026-3844](../CVE_2026_3844.md) | `235e77dfcf010b99d68609e53859dcf6a97b4e5775175d5fb3c81a7dfe1236fd` |
| FortiSandbox JRPC Path Guard | [`CVE-2026-39813.rs`](CVE-2026-39813.rs) | [CVE-2026-39813](../CVE_2026_39813.md) | `e2de2a35acea05ff3ef1e0efc44e1e9daffb2e780b373e4a5c4dedf6bc6a2274` |
| Marimo Terminal Command Execution Guard | [`CVE-2026-39987.rs`](CVE-2026-39987.rs) | [CVE-2026-39987](../CVE_2026_39987.md) | `359d5c56c8e194649e39970c655ef5d21e0e28997052ac84b977b6a25af404b5` |
| Joomla HTMLView Layout Guard | [`CVE-2026-40383.rs`](CVE-2026-40383.rs) | [CVE-2026-40383](../CVE_2026_40383.md) | `d107622de1bba9bd5cca6f3a678dc30da0664d3088bf2e088a4ce46ab377e8a7` |
| Flowise RCE via MCP Adapters Guard | [`CVE-2026-40933.rs`](CVE-2026-40933.rs) | [CVE-2026-40933](../CVE_2026_40933.md) | `4f245a134be032f46611f0d107faed56cd9c576ac5866598cee8c796ae003d38` |
| Rclone Options Set Access Guard | [`CVE-2026-41176.rs`](CVE-2026-41176.rs) | [CVE-2026-41176](../CVE_2026_41176.md) | `d6e480255c6d725d11f0479a73458378833b5d90a88fee3e5560917a9e5e8f32` |
| Flowise Parameter-override Bypass Guard | [`CVE-2026-41268.rs`](CVE-2026-41268.rs) | [CVE-2026-41268](../CVE_2026_41268.md) | `c0207b1f5657454cae03a446ef50a9bd850ee80162fb6f0554df7c54413e89db` |
| Paperclip RCE via Import Payload Guard | [`CVE-2026-41679.rs`](CVE-2026-41679.rs) | [CVE-2026-41679](../CVE_2026_41679.md) | `a1fde210405a13e83c679f260c3f1dcc30d8106ec240f599305f87eb74e7d3de` |
| cPanel and WHM Login-flow Authentication Bypass Guard | [`CVE-2026-41940.rs`](CVE-2026-41940.rs) | [CVE-2026-41940](../CVE_2026_41940.md) | `424b93bea0103e7d2bed77058293260cf0e3bea6fe7eab62d71b68beeaa9b970` |
| LiteLLM Proxy API Key SQL Guard | [`CVE-2026-42208.rs`](CVE-2026-42208.rs) | [CVE-2026-42208](../CVE_2026_42208.md) | `7410980155aabeff99080750bd73f401cef7a4f527b38489093879844e685452` |
| LiteLLM MCP Stdio Test Request Guard | [`CVE-2026-42271.rs`](CVE-2026-42271.rs) | [CVE-2026-42271](../CVE_2026_42271.md) | `04ec50c000f8f162e774aee43534d1db8e4b3a8cb4fe644fd0fce6c426e95437` |
| Apache CGI Internal Redirect Guard | [`CVE-2026-42356.rs`](CVE-2026-42356.rs) | [CVE-2026-42356](../CVE_2026_42356.md) | `2616334b37482b326e90e1531196c6abd1e4ef8ff1973bd50559f7c2269d5fb9` |
| Apache Mod Dav Shared Lock Overflow Guard | [`CVE-2026-42528.rs`](CVE-2026-42528.rs) | [CVE-2026-42528](../CVE_2026_42528.md) | `ec643b4c279bbc86ac6a97afcd678979c40faa2955a45fe95db4245d39967c67` |
| Apache Mod Xml2enc Heap Overflow Guard | [`CVE-2026-42536.rs`](CVE-2026-42536.rs) | [CVE-2026-42536](../CVE_2026_42536.md) | `aeb271e68b8cf33590fa9fd798ad95abd7a1cc5a46b930fb7263b4c3d4538d1d` |
| Apache mod_ssl OCSP Request Guard | [`CVE-2026-44185.rs`](CVE-2026-44185.rs) | [CVE-2026-44185](../CVE_2026_44185.md) | `1524a0d19e413de7940886285b87dd4fb6b095825ccb833af0d3a80d57fd875f` |
| Next.js WebSocket URL Guard | [`CVE-2026-44578.rs`](CVE-2026-44578.rs) | [CVE-2026-44578](../CVE_2026_44578.md) | `17b04d074937e3fbbb60fafcad051a45bb36e09b1e12654f035d4eca03f0c87d` |
| OFBiz Password Change Access Guard | [`CVE-2026-45434.rs`](CVE-2026-45434.rs) | [CVE-2026-45434](../CVE_2026_45434.md) | `c3dbe984b00f02d301998a0dbcc013fbdb43e3a1150cacb9bd2945d0a8da1a84` |
| Flowise Node Custom Function Request Guard | [`CVE-2026-46442.rs`](CVE-2026-46442.rs) | [CVE-2026-46442](../CVE_2026_46442.md) | `97e02ee75c84c0df1fb8d43fb8873a1c68635a51edb0d65ef412232371d0b1bd` |
| Apache Mod Heartmonitor Denial of Service Guard | [`CVE-2026-46729.rs`](CVE-2026-46729.rs) | [CVE-2026-46729](../CVE_2026_46729.md) | `dd0b6eac3276232a877af48790fc9d36983592b4eca3485338ee60ebcf949830` |
| Oracle EBS Payments File Transmission Guard | [`CVE-2026-46817.rs`](CVE-2026-46817.rs) | [CVE-2026-46817](../CVE_2026_46817.md) | `85a66b58cd0b838da0dca45e99f7d69e38793283acfb1bea8a96a4ee4992be6f` |
| ColdFusion Code Execution Path Guard | [`CVE-2026-48282.rs`](CVE-2026-48282.rs) | [CVE-2026-48282](../CVE_2026_48282.md) | `8323905d7d72693952897ef1e784156f844f83b8d87f86323322a02b2f0bbb64` |
| JCE Executable Upload Guard | [`CVE-2026-48907.rs`](CVE-2026-48907.rs) | [CVE-2026-48907](../CVE_2026_48907.md) | `04b17a14ee5f74ab79ffc32ac7201041afc0a637018587ba4ed84a2f71cc3914` |
| SP Page Builder RCE Guard | [`CVE-2026-48908.rs`](CVE-2026-48908.rs) | [CVE-2026-48908](../CVE_2026_48908.md) | `ea79728ac5ebca857c457725324a222c436b2dc07cfc92ecf17e660d7e5ec3c6` |
| iCagenda PHP Upload Guard | [`CVE-2026-48939.rs`](CVE-2026-48939.rs) | [CVE-2026-48939](../CVE_2026_48939.md) | `b8ce97c308249e3b9138bb84e92e6c5a08af7a97207bc6f3fd8483cba9a9fa1f` |
| WSO2 JWT Algorithm Guard | [`CVE-2026-5430.rs`](CVE-2026-5430.rs) | [CVE-2026-5430](../CVE_2026_5430.md) | `ec8421419ed35aa6c78ca30e790dfaab078ee704901d5b47059d08cf2dccbe96` |
| Apache mod_charset_lite Partial Character Guard | [`CVE-2026-56153.rs`](CVE-2026-56153.rs) | [CVE-2026-56153](../CVE_2026_56153.md) | `c9b61b9339ba92ed65466f3c9e332e4241813c8ca356aedf418d8376858a4b36` |
| Apache mod_rewrite URL Lookahead Guard | [`CVE-2026-56154.rs`](CVE-2026-56154.rs) | [CVE-2026-56154](../CVE_2026_56154.md) | `d075e6fd2f017908c9e0d4a8ff0c5407db51d5b11fa90db883aa88aa28638821` |
| Page Builder CK Executable Upload Guard | [`CVE-2026-56290.rs`](CVE-2026-56290.rs) | [CVE-2026-56290](../CVE_2026_56290.md) | `7862e8d2714db7bcdd59f51d331992bf79a42e122a11223936b330ad6515acfd` |
| Balbooa Forms Executable Upload Guard | [`CVE-2026-56291.rs`](CVE-2026-56291.rs) | [CVE-2026-56291](../CVE_2026_56291.md) | `174b43844249eff40e019ca3bef8d5817af5b9720c2de1be2d71ab7b682d1fa0` |
| Apache Mod Proxy Html Crash in Dump Content Guard | [`CVE-2026-56449.rs`](CVE-2026-56449.rs) | [CVE-2026-56449](../CVE_2026_56449.md) | `3521ab70118147a31c6231591f5f1d6f0b6d14d5c825988e598f3a45a4eb0f99` |
| Apache mod_http2 Shared Session Guard | [`CVE-2026-57941.rs`](CVE-2026-57941.rs) | [CVE-2026-57941](../CVE_2026_57941.md) | `d2c052c0598bf16b2d328ed62e385f79a69409aafc4e11433c0466f28e5920dd` |
| Apache Mod Dav Fs Property Database Read Access Guard | [`CVE-2026-58415.rs`](CVE-2026-58415.rs) | [CVE-2026-58415](../CVE_2026_58415.md) | `9de501468029badf348776aee0a31cc765dbb1e31e617a83f5fdddc003a3d3f0` |
| Apache HTTP Server Directory Walk Guard | [`CVE-2026-59685.rs`](CVE-2026-59685.rs) | [CVE-2026-59685](../CVE_2026_59685.md) | `2ea57ed8b1b2dc5ea15232ce9b5d83968d614cd953f1394d1118b1b52778ea55` |
| Gitea Org-mode Renderer Guard | [`CVE-2026-59774.rs`](CVE-2026-59774.rs) | [CVE-2026-59774](../CVE_2026_59774.md) | `ad023a5ff8df811b8e879b8b4add7381a2dbd9017eb323b86a5139b09c4b7ba4` |
| Apache mod_ssl SSLRequire Expression Guard | [`CVE-2026-59797.rs`](CVE-2026-59797.rs) | [CVE-2026-59797](../CVE_2026_59797.md) | `a57de7bb923e15ae98fb7166f067ff6eaa1c69252173b01684d756c2ddc35347` |
| PraisonAI RCE via CodeAgent. Execute Python Guard | [`CVE-2026-61447.rs`](CVE-2026-61447.rs) | [CVE-2026-61447](../CVE_2026_61447.md) | `24526532e473f76c60779f283f8d4d4995fb60434bdfdb81778df4e88feb2ff4` |
| vBulletin Eval Injection in the Template Runtime Guard | [`CVE-2026-61511.rs`](CVE-2026-61511.rs) | [CVE-2026-61511](../CVE_2026_61511.md) | `ad33d5aa822130670be2159ca0788c428269db1b3e86d2a31c59370fcc254c89` |
| AdonisJS Exception Rendering Guard | [`CVE-2026-61526.rs`](CVE-2026-61526.rs) | [CVE-2026-61526](../CVE_2026_61526.md) | `ca6650212676092260dced8e91530f6a4799421e9cf4e2ac4e235c64e8097aa4` |
| WordPress File Upload Plugin SQL Injection Guard | [`CVE-2026-62071.rs`](CVE-2026-62071.rs) | [CVE-2026-62071](../CVE_2026_62071.md) | `bc970bf33fd517c016bcd5340a0f996a53e425dd10395087f37bab8312e70d4f` |
| Apache Mod Proxy Ftp PASV Address Handling Guard | [`CVE-2026-63045.rs`](CVE-2026-63045.rs) | [CVE-2026-63045](../CVE_2026_63045.md) | `8e2274232ff4c627920790f22bdb09b55ae2093c8b40ac9b7968fddf21635977` |
| Apache Mod Vhost Alias Stack Overflow Guard | [`CVE-2026-63292.rs`](CVE-2026-63292.rs) | [CVE-2026-63292](../CVE_2026_63292.md) | `fbc5bd035f8b8b4891ace27c4465cca97d4f3e7fb4243c8e4ef297fc1af8dc41` |
| Apache mod_xml2enc Charset Conversion Guard | [`CVE-2026-63686.rs`](CVE-2026-63686.rs) | [CVE-2026-63686](../CVE_2026_63686.md) | `04f1319b18018eda4019dea1716c1dcb0436b942915fb664c5bd008f71c564f1` |
| Apache mod_proxy_uwsgi Transfer Encoding Guard | [`CVE-2026-63718.rs`](CVE-2026-63718.rs) | [CVE-2026-63718](../CVE_2026_63718.md) | `9075d031c2d0e8fd61017680e638680eda18b2e2402cd7f293df4aa0f07e117f` |
| Wavlink WL-WN530H4 Internet CGI Command Guard | [`CVE-2026-6483.rs`](CVE-2026-6483.rs) | [CVE-2026-6483](../CVE_2026_6483.md) | `7e099b4f434d703e25c325a6b774f19f9f23d2123d3c43972ae0512b89525701` |
| MLflow SSRF in Webhook Delivery Guard | [`CVE-2026-64849.rs`](CVE-2026-64849.rs) | [CVE-2026-64849](../CVE_2026_64849.md) | `59e6ec3e600e55cd9fd08d0cc4eff8f4400cfd4562c94e270e90c02458eb0d35` |
| Rails Active Storage libvips Input Guard | [`CVE-2026-66066.rs`](CVE-2026-66066.rs) | [CVE-2026-66066](../CVE_2026_66066.md) | `955f3f65bf7b0701174a3be2eb1d0d97de23e2bdb29b898cf19431892a9f0316` |
| Hubleys Upload Content Type Guard | [`CVE-2026-67444.rs`](CVE-2026-67444.rs) | [CVE-2026-67444](../CVE_2026_67444.md) | `f8017522d05f3c3bba854cc58dd3fc64609a1cdaf5cd5f8c5b98443d2a9605b3` |
| Portabase Path Traversal in TUS Web Endpoint Guard | [`CVE-2026-67449.rs`](CVE-2026-67449.rs) | [CVE-2026-67449](../CVE_2026_67449.md) | `db2fbacf17acd04ee447ef5da356ddb65bf260368857c3067bf3291c6c3bd81b` |
| Hugo Blog AI Proxy Access Guard | [`CVE-2026-68905.rs`](CVE-2026-68905.rs) | [CVE-2026-68905](../CVE_2026_68905.md) | `de39d762fc78d89c5c639156277a13adfdc89f0ed4e8705e03a573062d2f3459` |
| Ivanti EPMM Administrative Input Guard | [`CVE-2026-6973.rs`](CVE-2026-6973.rs) | [CVE-2026-6973](../CVE_2026_6973.md) | `17938d84bf06584475fef17775a86fb5e933a6ad29a970f2f4b8615a4e815085` |
| ToolJet App Environment Setup Access Guard | [`CVE-2026-70642.rs`](CVE-2026-70642.rs) | [CVE-2026-70642](../CVE_2026_70642.md) | `2b55e64aedbd8afce69d8067a6004ae04441b486e7f353c5072831a88173f0d0` |
| PrestaShop Webservice Customer Records Guard | [`CVE-2026-71519.rs`](CVE-2026-71519.rs) | [CVE-2026-71519](../CVE_2026_71519.md) | `9ff201d34eb523bd2e2ce8364ca00a7a6a7dacd2c0df3831836c137421dc7688` |
| PrestaShop Webservice Scope Override Guard | [`CVE-2026-71531.rs`](CVE-2026-71531.rs) | [CVE-2026-71531](../CVE_2026_71531.md) | `576fcc8fdea64b378b665af740bbae4e657b0605d0262800daa990d26a544542` |
| Zyxel GS1900 CGI Request Guard | [`CVE-2026-7273.rs`](CVE-2026-7273.rs) | [CVE-2026-7273](../CVE_2026_7273.md) | `9100a007720cc1ed55030beb8c8701a4af584389ad787df331b5e2f2977cccc6` |
| Metabase Password Reset SQL Guard | [`CVE-2026-72898.rs`](CVE-2026-72898.rs) | [CVE-2026-72898](../CVE_2026_72898.md) | `7f80d85bba6440ba9f14d829a4cce38e970079b0f6c9fb66c87f47daaf9b0370` |
| Apache Mod Auth Digest DoS Attack Guard | [`CVE-2026-73637.rs`](CVE-2026-73637.rs) | [CVE-2026-73637](../CVE_2026_73637.md) | `9570b85e7051c8008f376fcdc46d6cb23850f376bc1b04ca140e35c13ef5f468` |
| ManageEngine DDI Central DNS Suffix Guard | [`CVE-2026-75869.rs`](CVE-2026-75869.rs) | [CVE-2026-75869](../CVE_2026_75869.md) | `43281432852a58ddb070ff0258574e48cfb09c9c117c70dac5a45fc7ba550132` |
| Cisco ASA and FTD SSL VPN Request Guard | [`CVE-2026-76418.rs`](CVE-2026-76418.rs) | [CVE-2026-76418](../CVE_2026_76418.md) | `769165ea99f48167586cf4a7f708a3e40099725bf5dd647bbe77d1111183ffda` |
| Cisco ISE Privileged API Guard | [`CVE-2026-76460.rs`](CVE-2026-76460.rs) | [CVE-2026-76460](../CVE_2026_76460.md) | `a5054d497937104213d6bb7b2e0ff951b2f7a883e9bd4e7a070c1a7e2833a51f` |
| Cisco Secure Email Gateway Email Parsing SQL Guard | [`CVE-2026-76461.rs`](CVE-2026-76461.rs) | [CVE-2026-76461](../CVE_2026_76461.md) | `09d02d8fc87efbf0a003e48ba10b76ec18be34f26e4011995928a486a09874c0` |
| Cisco SD-WAN Manager Admin API Guard | [`CVE-2026-76504.rs`](CVE-2026-76504.rs) | [CVE-2026-76504](../CVE_2026_76504.md) | `121b8952a9b6b6ec6189d4bef458f9f37d9362bb2a998ae43dc8f6f12815e9dc` |
| Cisco Secure Email Gateway File Write Guard | [`CVE-2026-76519.rs`](CVE-2026-76519.rs) | [CVE-2026-76519](../CVE_2026_76519.md) | `8ccba39f96f32f323daf20cac7a601cbab74989b47f01852b47df44ea9e23e0a` |
| Apache Mod Userdir Information Disclosure Guard | [`CVE-2026-79768.rs`](CVE-2026-79768.rs) | [CVE-2026-79768](../CVE_2026_79768.md) | `8260b19cd0363dd8311ab32571ef228ab8d4d0c49df90444103ed7e107989f7f` |
| Progress LoadMaster API Command Guard | [`CVE-2026-8037.rs`](CVE-2026-8037.rs) | [CVE-2026-8037](../CVE_2026_8037.md) | `d89591180d335fd29a71a2b7e0256669229eba5d03140566b08d1aa27bea0161` |
| Burst Statistics Privileged Request Guard | [`CVE-2026-8181.rs`](CVE-2026-8181.rs) | [CVE-2026-8181](../CVE_2026_8181.md) | `871e12e384454028a80e7cc6be94f6b64a19e2185f86968b12e2094577118699` |
| PaperCut Database Driver Guard | [`CVE-2026-82078.rs`](CVE-2026-82078.rs) | [CVE-2026-82078](../CVE_2026_82078.md) | `339e1da1c084331b452536a458100f4470b949a5df234e9ee088a5904e7c89aa` |
| DataStage Connector URL Guard | [`CVE-2026-82102.rs`](CVE-2026-82102.rs) | [CVE-2026-82102](../CVE_2026_82102.md) | `7f77a1f9d34dcb0722e4fd2c8fa450db2ef07ce5a8b16a2961480d7d2405ad86` |
| Artifactory Administrative Access Guard | [`CVE-2026-82329.rs`](CVE-2026-82329.rs) | [CVE-2026-82329](../CVE_2026_82329.md) | `0d3b216f02a0b2c2884c75debd89783cac7077c529eed38ed0270abcd4d87107` |
| VITEC EZ TV ASPX Upload Guard | [`CVE-2026-82504.rs`](CVE-2026-82504.rs) | [CVE-2026-82504](../CVE_2026_82504.md) | `62e728fbdd9c9ccc96b22fe4eb7ec8cea49f6daa50d5a33184aceeab686add24` |
| Plesk Obsidian GoAccess Statistics Guard | [`CVE-2026-82977.rs`](CVE-2026-82977.rs) | [CVE-2026-82977](../CVE_2026_82977.md) | `7c191db1675553222e63e2c4d96a715807adbf0de6154fa99ec3f7e1354981f7` |
| GitLab Commits API File Read Guard | [`CVE-2026-85706.rs`](CVE-2026-85706.rs) | [CVE-2026-85706](../CVE_2026_85706.md) | `20bb0bb9bbdb6d3596d559486b43d438516e57d603bc3da5b320da259b45adb4` |
| pgAdmin Webserver Identity Header Guard | [`CVE-2026-86863.rs`](CVE-2026-86863.rs) | [CVE-2026-86863](../CVE_2026_86863.md) | `80237f28a12079859eddf464dab442305162fcbfde28f98b7ec522008e92dd2a` |
| WP Maps Pro Temporary Access Guard | [`CVE-2026-8732.rs`](CVE-2026-8732.rs) | [CVE-2026-8732](../CVE_2026_8732.md) | `69445fcb10e19b5666701a510897f15976e0d7411209b8b8a03e3b02074b6912` |
| WordPress Page Template Path Guard | [`CVE-2026-87902.rs`](CVE-2026-87902.rs) | [CVE-2026-87902](../CVE_2026_87902.md) | `bb65b475bf2057489175954c9aac715f9bb3fc4774383056e32f7cd267c713a6` |
| NetScaler Gateway Input Guard | [`CVE-2026-88771.rs`](CVE-2026-88771.rs) | [CVE-2026-88771](../CVE_2026_88771.md) | `ec1c3bb66bef5bea692b035d7ff402aece1701c9245ff523dcbc02b5ff6fce47` |
| NetScaler Gateway Buffer Overflow Guard | [`CVE-2026-88772.rs`](CVE-2026-88772.rs) | [CVE-2026-88772](../CVE_2026_88772.md) | `6966479fed72e17fec4d7361c15cecb01fa530481b73923233bd771feba03597` |
| NetScaler HTTP Framing Guard | [`CVE-2026-88773.rs`](CVE-2026-88773.rs) | [CVE-2026-88773](../CVE_2026_88773.md) | `9557407080a66a7ff09586751a9dde179b0dc09f60ada686c18b9f077ff35f3c` |
| NetScaler Gateway Denial of Service Guard | [`CVE-2026-88779.rs`](CVE-2026-88779.rs) | [CVE-2026-88779](../CVE_2026_88779.md) | `9911aa80aaa8ac6e7eda9b1531e166a20ab4d8a017ca4d2ee97c9240f4d67b9e` |
| Dolibarr Ticket Attachment Access Guard | [`CVE-2026-89037.rs`](CVE-2026-89037.rs) | [CVE-2026-89037](../CVE_2026_89037.md) | `869defdce08352957275da36beb61261b295aad824f6d4cf2a438e4d730bee21` |
| Drupal Core SQL Injection on PostgreSQL Sites Guard | [`CVE-2026-9082.rs`](CVE-2026-9082.rs) | [CVE-2026-9082](../CVE_2026_9082.md) | `a6ef137407cbee080693ff90de4c2c8007f0e8e639b646fc24c39346e86a2bd6` |
| GitLab AI Gateway Prompt-template Sandbox Escape Guard | [`CVE-2026-90970.rs`](CVE-2026-90970.rs) | [CVE-2026-90970](../CVE_2026_90970.md) | `06bbdc222e84bb393a7142cbcc6f8a69552e21975606df7328f385fc99924e72` |
| IBM Langflow Auto-login and Code Validation Guard | [`CVE-2026-9198.rs`](CVE-2026-9198.rs) | [CVE-2026-9198](../CVE_2026_9198.md) | `cc6fe268e874bdd73155f2798211b2a109d412e0bd8fd66f8218f75d656d0902` |
| Vault MFA Pending Login Guard | [`CVE-2026-92738.rs`](CVE-2026-92738.rs) | [CVE-2026-92738](../CVE_2026_92738.md) | `77982b380e6ad5b9250b27d1f2053acf0e0039bda854b3c315b8b29827e7b3ad` |
| Apache Mod Dav Fs Namespace Overflow Guard | [`CVE-2026-93546.rs`](CVE-2026-93546.rs) | [CVE-2026-93546](../CVE_2026_93546.md) | `3c2902db9f5fd5ccaf02bffa9f6c42c351ef6dcef92702b4b9a2340ca2c7d6c4` |
| Check Point Management Server Upload Path Guard | [`CVE-2026-93616.rs`](CVE-2026-93616.rs) | [CVE-2026-93616](../CVE_2026_93616.md) | `6301449df44a2fcf2a8dc7e79fb6e48e508d84a29af2678477959100b45c2ad7` |
| Langflow OS Command Input Guard | [`CVE-2026-93674.rs`](CVE-2026-93674.rs) | [CVE-2026-93674](../CVE_2026_93674.md) | `9f10e323c257ec9cf0eb1067eddc5f906fe0b1750c7a72f2c808d13062f35c1c` |
| VeloCloud Orchestrator Internal Route Guard | [`CVE-2026-93952.rs`](CVE-2026-93952.rs) | [CVE-2026-93952](../CVE_2026_93952.md) | `f1104cdb1149edadbe9833f6bc79eddead43df24c28f7c6d9bd5ec7c86e49d25` |
| F5 BIG-IP APM OAuth Access Guard | [`CVE-2026-94127.rs`](CVE-2026-94127.rs) | [CVE-2026-94127](../CVE_2026_94127.md) | `10b934294a9aff1e3b4c9e870dc531b048c0b4a598ffe8eb5ab35b22eac84bfc` |
| Edimax BR-6478AC formAccept Command Guard | [`CVE-2026-9440.rs`](CVE-2026-9440.rs) | [CVE-2026-9440](../CVE_2026_9440.md) | `4cbd42825f2b3faf5919f2fd84eeb44cb97a7ef82d0e348e6f9e13e3d9f0b306` |
| Edimax BR-6478AC formiNICbasic Command Guard | [`CVE-2026-9441.rs`](CVE-2026-9441.rs) | [CVE-2026-9441](../CVE_2026_9441.md) | `f9b66fa9fe028f949df3de8a23eca6751a415677279499a6dde405a746c6c412` |
| TOTOLINK CA750-PoE Network Diagnostics Guard | [`CVE-2026-9514.rs`](CVE-2026-9514.rs) | [CVE-2026-9514](../CVE_2026_9514.md) | `6b658c6da1d6d9b2a665d82eb9ec845f8af78bea3af4b5475d0e6fc8bc8d467c` |
| TOTOLINK CA750-PoE User Data Reset Guard | [`CVE-2026-9515.rs`](CVE-2026-9515.rs) | [CVE-2026-9515](../CVE_2026_9515.md) | `484282a1934489fac25a0efe3e5ab34e70a725d47a94d7c8f187b6c448424834` |
| TOTOLINK CA750-PoE Bootloader Upgrade Guard | [`CVE-2026-9531.rs`](CVE-2026-9531.rs) | [CVE-2026-9531](../CVE_2026_9531.md) | `fb0cfc025dc62545c6d1aed852489ca12d5b44c42e9baa83d31072407eeb8428` |
| TOTOLINK CA750-PoE User Data Upload Guard | [`CVE-2026-9532.rs`](CVE-2026-9532.rs) | [CVE-2026-9532](../CVE_2026_9532.md) | `ac353f084480c91fcd0f3d4aa8dd35bcd86014c06c57e141b652dad8b89f8e07` |
| TOTOLINK CA750-PoE Firmware Receive Guard | [`CVE-2026-9533.rs`](CVE-2026-9533.rs) | [CVE-2026-9533](../CVE_2026_9533.md) | `b354dfc1e07c9243a6b853598a19ea64ac7be8a9ee59dbe04afedbe8cc6c2154` |
| TOTOLINK CA750-PoE WPS Configuration Guard | [`CVE-2026-9534.rs`](CVE-2026-9534.rs) | [CVE-2026-9534](../CVE_2026_9534.md) | `85687090adb44716dbc991d93f333b7eb8f9b21407d5358e9cf4a20ea2684a95` |
| Switchvox SQL Injection Guard | [`CVE-2026-9586.rs`](CVE-2026-9586.rs) | [CVE-2026-9586](../CVE_2026_9586.md) | `325fbb6b068a2846827c5570ecba215a8b21e12ac9432da15d6c53accdba5a70` |
| Plex Plugin Search URL Guard | [`CVE-2026-96654.rs`](CVE-2026-96654.rs) | [CVE-2026-96654](../CVE_2026_96654.md) | `f0069d3fe180311eeaad6e2f6f85df9d00ff18c59a5d3aef19e455bcbe157ab7` |
| Plex Timeline Request Guard | [`CVE-2026-96655.rs`](CVE-2026-96655.rs) | [CVE-2026-96655](../CVE_2026_96655.md) | `aa5830a896d9b2cb926167d32479a4bafbc5e844f13308c916da58a2a6981d62` |
| Plex Transcoder Options Guard | [`CVE-2026-96656.rs`](CVE-2026-96656.rs) | [CVE-2026-96656](../CVE_2026_96656.md) | `052db6bbaff3d7f439401a35bd38358fdfa9b11bf5d87db1502e5f582d9a8938` |
| GitLab CI/CD Regex Rule Guard | [`CVE-2026-97229.rs`](CVE-2026-97229.rs) | [CVE-2026-97229](../CVE_2026_97229.md) | `76fe30f7cfe1709376754abbc71d9fd728ed9d2f1fff8d665fb0ea26ee61f68e` |
=======
| Module source | Case study | SHA-256 |
| --- | --- | --- |
| [CVE-2014-8361.rs](CVE-2014-8361.rs) | [CVE-2014-8361](../CVE_2014_8361.md) | `b2de5cda81ddcf6b504817f1104143000ff27327eaff85e9d08b2f076cc7dd87` |
| [CVE-2015-5468.rs](CVE-2015-5468.rs) | [CVE-2015-5468](../CVE_2015_5468.md) | `09a0fdc8ed75d7c3985fe39317a919673cdc849fbaf9e164e705012b7a6bff18` |
| [CVE-2016-10972.rs](CVE-2016-10972.rs) | [CVE-2016-10972](../CVE_2016_10972.md) | `834c4e8e1445c699e5024f6a0215a5bb0b9d001cc29d1d19b6d7bb602fc0aba5` |
| [CVE-2017-5689.rs](CVE-2017-5689.rs) | [CVE-2017-5689](../CVE_2017_5689.md) | `e2a5762520fb428e4fac6e85254fcd7a52ea6b3131a893e682c5421ae2380955` |
| [CVE-2019-7256.rs](CVE-2019-7256.rs) | [CVE-2019-7256](../CVE_2019_7256.md) | `7aec78d03af203d0ad9a5ba0fd832636f5c8e1b5e328e289551176086df8f8ac` |
| [CVE-2020-11978.rs](CVE-2020-11978.rs) | [CVE-2020-11978](../CVE_2020_11978.md) | `baaeb951315c5808bad32f42764e0cd6a9f65cf9a243a572bb565f2607b9a002` |
| [CVE-2020-11981.rs](CVE-2020-11981.rs) | [CVE-2020-11981](../CVE_2020_11981.md) | `4c108011fc1d2f47242a89b2b3b3feaa9364fb224a5da159bb252ca896c864c3` |
| [CVE-2020-13927.rs](CVE-2020-13927.rs) | [CVE-2020-13927](../CVE_2020_13927.md) | `1ea44c53e198a02d09a2c97b026542b8fc7fed4022baac4701fe7ffab7ea5eb7` |
| [CVE-2020-36847.rs](CVE-2020-36847.rs) | [CVE-2020-36847](../CVE_2020_36847.md) | `7702fc65c8cbd06db169885c641086dc2c0a55075432ddf6be2596ee65acc46d` |
| [CVE-2021-22204.rs](CVE-2021-22204.rs) | [CVE-2021-22204](../CVE_2021_22204.md) | `1a9f8e7240f86e12775be7dc28a9b4b0f2a8aaf16f10704b3187034b73bea46f` |
| [CVE-2021-38540.rs](CVE-2021-38540.rs) | [CVE-2021-38540](../CVE_2021_38540.md) | `1ea44c53e198a02d09a2c97b026542b8fc7fed4022baac4701fe7ffab7ea5eb7` |
| [CVE-2021-43798.rs](CVE-2021-43798.rs) | [CVE-2021-43798](../CVE_2021_43798.md) | `f23dc865c1ad656eebd864394fc4871c57f109d86d8d41ecb9ea6db3c021803d` |
| [CVE-2022-24288.rs](CVE-2022-24288.rs) | [CVE-2022-24288](../CVE_2022_24288.md) | `baaeb951315c5808bad32f42764e0cd6a9f65cf9a243a572bb565f2607b9a002` |
| [CVE-2022-25486.rs](CVE-2022-25486.rs) | [CVE-2022-25486](../CVE_2022_25486.md) | `3d2fc2463cab9d366d30b51ac9ab3c4b4ae0d8ecc78a212e7d2e2b22c57ca520` |
| [CVE-2022-40127.rs](CVE-2022-40127.rs) | [CVE-2022-40127](../CVE_2022_40127.md) | `baaeb951315c5808bad32f42764e0cd6a9f65cf9a243a572bb565f2607b9a002` |
| [CVE-2023-22884.rs](CVE-2023-22884.rs) | [CVE-2023-22884](../CVE_2023_22884.md) | `baaeb951315c5808bad32f42764e0cd6a9f65cf9a243a572bb565f2607b9a002` |
| [CVE-2023-22894.rs](CVE-2023-22894.rs) | [CVE-2023-22894](../CVE_2023_22894.md) | `800ae3f374bb8ba7bf3678481dfa5661a81f91201e256c79e64c6cbadd36e088` |
| [CVE-2023-27524.rs](CVE-2023-27524.rs) | [CVE-2023-27524](../CVE_2023_27524.md) | `9dd2943e6ea18505ae9d1ca1530cd2be0b8b47eb3c69539576a06610e6023400` |
| [CVE-2023-39780.rs](CVE-2023-39780.rs) | [CVE-2023-39780](../CVE_2023_39780.md) | `d2e76be629df988449cd4a3e676cba14147fe23b886ab760fa7fc79fee06c581` |
| [CVE-2023-49606.rs](CVE-2023-49606.rs) | [CVE-2023-49606](../CVE_2023_49606.md) | `c3695972865517e7277e9cb58988c02cb8978e5d51c29b0dc1d24962321dc11d` |
| [CVE-2023-54335.rs](CVE-2023-54335.rs) | [CVE-2023-54335](../CVE_2023_54335.md) | `9c069b522e581d36da9e98f98d7279185b520c5965da094e41a04d14314281e0` |
| [CVE-2025-3248.rs](CVE-2025-3248.rs) | [CVE-2025-3248](../CVE_2025_3248.md) | `9e517ac5c21023247dc58456b6f198dfd5ab182ea8c912029ef0d9dacac77536` |
| [CVE-2025-11201.rs](CVE-2025-11201.rs) | [CVE-2025-11201](../CVE_2025_11201.md) | `1c949dc3ee026fc4bbfb5ff60c93f07751ab0ece5f9e8aecde3fb4b0040d5a45` |
| [CVE-2025-24016.rs](CVE-2025-24016.rs) | [CVE-2025-24016](../CVE_2025_24016.md) | `d39f99eeb9352c916a3b467ac1d4a9d9e114bd8207491dceeac967a843cebe02` |
| [CVE-2025-28137.rs](CVE-2025-28137.rs) | [CVE-2025-28137](../CVE_2025_28137.md) | `94faa1133f57f999652dbf576354dfc46630528294a424b882c0874c06b7603d` |
| [CVE-2025-31161.rs](CVE-2025-31161.rs) | [CVE-2025-31161](../CVE_2025_31161.md) | `baeec160b4f54ecff50d08a586177d484a8e8065a20949d12c098a8be46f8d1a` |
| [CVE-2025-32432.rs](CVE-2025-32432.rs) | [CVE-2025-32432](../CVE_2025_32432.md) | `4c7c4cab75f97396719aaae395a2515183831f1d3df52067d6b5b3f156f8ffc9` |
| [CVE-2025-43867.rs](CVE-2025-43867.rs) | [CVE-2025-43867](../CVE_2025_43867.md) | `b93c2b141595ba0d5adcbf04d9713aa7163348b272b057f854daa2329e74b626` |
| [CVE-2025-43874.rs](CVE-2025-43874.rs) | [CVE-2025-43874](../CVE_2025_43874.md) | `1cf044edf4bda47341c25df7bb23f9aeda93a8f6284a8e0f1a4fc09af12b2cde` |
| [CVE-2025-60021.rs](CVE-2025-60021.rs) | [CVE-2025-60021](../CVE_2025_60021.md) | `5c2b1f302cc1ba3c23cd8aab98aba75301e8bff4b1c8e8a04414ea16f75e12c8` |
| [CVE-2025-61624.rs](CVE-2025-61624.rs) | [CVE-2025-61624](../CVE_2025_61624.md) | `deb67c8dcebbe2b6bc9930595555b0ebe9a7ecff8de7a2a87adf7a1c077f425c` |
| [CVE-2025-69985.rs](CVE-2025-69985.rs) | [CVE-2025-69985](../CVE_2025_69985.md) | `87babd58fab6a57763690c895d04d3ae218349fe078b86dc47b2a59a492a120d` |
| [CVE-2026-0300.rs](CVE-2026-0300.rs) | [CVE-2026-0300](../CVE_2026_0300.md) | `de255d17275b04cea91c2b9e720aec2b299529d95f5ea0d2dec6cb49268e60be` |
| [CVE-2026-0322.rs](CVE-2026-0322.rs) | [CVE-2026-0322](../CVE_2026_0322.md) | `b3c6c199342be774e579edf2261a0f138653d9bfd1979fc66eea1ce6599cb632` |
| [CVE-2026-3199.rs](CVE-2026-3199.rs) | [CVE-2026-3199](../CVE_2026_3199.md) | `7cb6228c17aa4186f38af4ca445a258058cba67a9338f284be0ba84e8d86dfe3` |
| [CVE-2026-3844.rs](CVE-2026-3844.rs) | [CVE-2026-3844](../CVE_2026_3844.md) | `5398f4e5ef7931a7b8d2b49bcc422cd150ad06473f284ec8c3597c1b6a66537b` |
| [CVE-2026-5430.rs](CVE-2026-5430.rs) | [CVE-2026-5430](../CVE_2026_5430.md) | `60014aa4fd6401a7604234b482cbf3c02f64117b49154e0a3c3c830f56f2c5d9` |
| [CVE-2026-6483.rs](CVE-2026-6483.rs) | [CVE-2026-6483](../CVE_2026_6483.md) | `425ef7ef41ba6a2514de9422bb66cf047b6b5e38e5366e6fe2819274c4a7ac7a` |
| [CVE-2026-6973.rs](CVE-2026-6973.rs) | [CVE-2026-6973](../CVE_2026_6973.md) | `75375c3f977d3888af1222a92cb062ff8a779f3e51c27191ec6293595baf8e27` |
| [CVE-2026-7273.rs](CVE-2026-7273.rs) | [CVE-2026-7273](../CVE_2026_7273.md) | `c0b337a0345c2431cb9ec2633145918b5a505b6cb6cd9d76045cdb660c9c74ac` |
| [CVE-2026-8181.rs](CVE-2026-8181.rs) | [CVE-2026-8181](../CVE_2026_8181.md) | `225e70a50749606ac2e8399980aed88aad8e433da6592484e008a6208cf82ff5` |
| [CVE-2026-8732.rs](CVE-2026-8732.rs) | [CVE-2026-8732](../CVE_2026_8732.md) | `88bbcf04cfe8072e027f87e57d0367925b91420cef1ee3b931473f37c5f97da3` |
| [CVE-2026-9440.rs](CVE-2026-9440.rs) | [CVE-2026-9440](../CVE_2026_9440.md) | `c24fcf3dd1b9eb2ebad8c7088ef07ce913887666596b1c928546d1fa6b1f68e6` |
| [CVE-2026-9441.rs](CVE-2026-9441.rs) | [CVE-2026-9441](../CVE_2026_9441.md) | `c24fcf3dd1b9eb2ebad8c7088ef07ce913887666596b1c928546d1fa6b1f68e6` |
| [CVE-2026-9514.rs](CVE-2026-9514.rs) | [CVE-2026-9514](../CVE_2026_9514.md) | `4defca74e787aefa9a5a067b680c0f3a52908ae22498c3d2680c8a619836b829` |
| [CVE-2026-9515.rs](CVE-2026-9515.rs) | [CVE-2026-9515](../CVE_2026_9515.md) | `4defca74e787aefa9a5a067b680c0f3a52908ae22498c3d2680c8a619836b829` |
| [CVE-2026-9531.rs](CVE-2026-9531.rs) | [CVE-2026-9531](../CVE_2026_9531.md) | `4defca74e787aefa9a5a067b680c0f3a52908ae22498c3d2680c8a619836b829` |
| [CVE-2026-9532.rs](CVE-2026-9532.rs) | [CVE-2026-9532](../CVE_2026_9532.md) | `4defca74e787aefa9a5a067b680c0f3a52908ae22498c3d2680c8a619836b829` |
| [CVE-2026-9533.rs](CVE-2026-9533.rs) | [CVE-2026-9533](../CVE_2026_9533.md) | `4defca74e787aefa9a5a067b680c0f3a52908ae22498c3d2680c8a619836b829` |
| [CVE-2026-9534.rs](CVE-2026-9534.rs) | [CVE-2026-9534](../CVE_2026_9534.md) | `4defca74e787aefa9a5a067b680c0f3a52908ae22498c3d2680c8a619836b829` |
| [CVE-2026-12569.rs](CVE-2026-12569.rs) | [CVE-2026-12569](../CVE_2026_12569.md) | `a6b1a3c4bf4d2c43c4e45756575316288affb83bf6d7957f23276718c9b214c8` |
| [CVE-2026-8037.rs](CVE-2026-8037.rs) | [CVE-2026-8037](../CVE_2026_8037.md) | `c462f57422d80bab054769f1ad03006afeb2286c82a4d53620b97fbaa7c78ff5` |
| [CVE-2026-9586.rs](CVE-2026-9586.rs) | [CVE-2026-9586](../CVE_2026_9586.md) | `fd65995d4b31b5f53234784453d9eb18a9c03aa714f8ccb654852ba4c85b5c33` |
| [CVE-2026-15733.rs](CVE-2026-15733.rs) | [CVE-2026-15733](../CVE_2026_15733.md) | `76a90a665792f654b065d54600f266c1786ee62e8adbedd40d14874521ca3886` |
| [CVE-2026-18053.rs](CVE-2026-18053.rs) | [CVE-2026-18053](../CVE_2026_18053.md) | `e9243f6bb5e157e690bb6fbdec852a0d2dcc795d916c9c063406677c3a0c5a87` |
| [CVE-2026-18556.rs](CVE-2026-18556.rs) | [CVE-2026-18556](../CVE_2026_18556.md) | `2f98d38fa9a306086c126a9f17a49770ad3bc0d02e7462c404b1e1abc60356b6` |
| [CVE-2026-18963.rs](CVE-2026-18963.rs) | [CVE-2026-18963](../CVE_2026_18963.md) | `549a315008cfc84d0dcf8079220a3798ba97aa697f3e3e54a4dcddd34ebd89ed` |
| [CVE-2026-19907.rs](CVE-2026-19907.rs) | [CVE-2026-19907](../CVE_2026_19907.md) | `1b67d633bdf41242c3d42852738d3c784d872e03f73f9fb5323a7362dcea43fd` |
| [CVE-2026-20182.rs](CVE-2026-20182.rs) | [CVE-2026-20182](../CVE_2026_20182.md) | `2492a566c687262c003f05ebc87a3df6744bd8bdd833563ed492baea7f4e3911` |
| [CVE-2026-20253.rs](CVE-2026-20253.rs) | [CVE-2026-20253](../CVE_2026_20253.md) | `4a57fbf048f3360857ef13f3f1cd53057b78e9614bdf41423a691e9f3fa8595c` |
| [CVE-2026-23734.rs](CVE-2026-23734.rs) | [CVE-2026-23734](../CVE_2026_23734.md) | `a9e4d21f44d192a57378146a5473c0016559d129f7ce3a28376a96cdf8321f4d` |
| [CVE-2026-25939.rs](CVE-2026-25939.rs) | [CVE-2026-25939](../CVE_2026_25939.md) | `af22f92e9319e0aa9ade5d6ccf9af1b13b42a80bfd6eb66dc88f3401628ebcab` |
| [CVE-2026-27175.rs](CVE-2026-27175.rs) | [CVE-2026-27175](../CVE_2026_27175.md) | `7a28025aee471b067cba82bf84b516ba94d307ff1a39ddf88f59f83ac02cd5cf` |
| [CVE-2026-27305.rs](CVE-2026-27305.rs) | [CVE-2026-27305](../CVE_2026_27305.md) | `5566862d05d5386c70670ff4321bde454888731687fdc50d3b7566a2089eb89e` |
| [CVE-2026-27760.rs](CVE-2026-27760.rs) | [CVE-2026-27760](../CVE_2026_27760.md) | `37c703077f222458f610b6c5e09e300dba79fd49b61e0fb97c8809102e5914e3` |
| [CVE-2026-28318.rs](CVE-2026-28318.rs) | [CVE-2026-28318](../CVE_2026_28318.md) | `06c5d6bb78944f18ac69ddc18c31c44636e16519d90390d09997d1d06c4077f8` |
| [CVE-2026-34908.rs](CVE-2026-34908.rs) | [CVE-2026-34908](../CVE_2026_34908.md) | `582abc2d0b149d9419450f0fba5384d033a6a512c8a4d51421c9aeb5eea9d41c` |
| [CVE-2026-34909.rs](CVE-2026-34909.rs) | [CVE-2026-34909](../CVE_2026_34909.md) | `582abc2d0b149d9419450f0fba5384d033a6a512c8a4d51421c9aeb5eea9d41c` |
| [CVE-2026-34910.rs](CVE-2026-34910.rs) | [CVE-2026-34910](../CVE_2026_34910.md) | `582abc2d0b149d9419450f0fba5384d033a6a512c8a4d51421c9aeb5eea9d41c` |
| [CVE-2026-35223.rs](CVE-2026-35223.rs) | [CVE-2026-35223](../CVE_2026_35223.md) | `2ae5e42667cd028895a0e105b7b6e9a4508e815a33707603ac8d260e947a165c` |
| [CVE-2026-36356.rs](CVE-2026-36356.rs) | [CVE-2026-36356](../CVE_2026_36356.md) | `4e9fcc623dcd6681780d444ad750868dac5abbb339c6156d6980ea2f674c3175` |
| [CVE-2026-39813.rs](CVE-2026-39813.rs) | [CVE-2026-39813](../CVE_2026_39813.md) | `f5a48f76a93e3aaf6414eb16114c9b1aa7412048e9fee08dfcc9295594812662` |
| [CVE-2026-39987.rs](CVE-2026-39987.rs) | [CVE-2026-39987](../CVE_2026_39987.md) | `a650a2056097fc003d01c7f275a434b87d50e2a4c4b4106861225464d3fe9bd1` |
| [CVE-2026-40383.rs](CVE-2026-40383.rs) | [CVE-2026-40383](../CVE_2026_40383.md) | `5d0a2dc7602e548ee2c737f6ad6bd1048956c819968b5c1be7ffcee6fe582f1d` |
| [CVE-2026-40933.rs](CVE-2026-40933.rs) | [CVE-2026-40933](../CVE_2026_40933.md) | `03d6131a51786792d39069f74bb024bfd01e753569b53c27e47083d2179c4bcb` |
| [CVE-2026-41084.rs](CVE-2026-41084.rs) | [CVE-2026-41084](../CVE_2026_41084.md) | `4c108011fc1d2f47242a89b2b3b3feaa9364fb224a5da159bb252ca896c864c3` |
| [CVE-2026-41176.rs](CVE-2026-41176.rs) | [CVE-2026-41176](../CVE_2026_41176.md) | `fea32b12f9bb6c48e1923744664dff5c9299fba561d69ab90a1e613994bdebc0` |
| [CVE-2026-41268.rs](CVE-2026-41268.rs) | [CVE-2026-41268](../CVE_2026_41268.md) | `fdb42fe20cc1d8a21a7eeb66b9fb572d4147e91fdb5ef341ae72a78129e31f62` |
| [CVE-2026-41679.rs](CVE-2026-41679.rs) | [CVE-2026-41679](../CVE_2026_41679.md) | `29d3eb09e76810da9bc0e61c2093c70734791937ddc613d0708cfb5284e7162e` |
| [CVE-2026-41940.rs](CVE-2026-41940.rs) | [CVE-2026-41940](../CVE_2026_41940.md) | `047a0695c210acc1f6a292ab720878b9f8545c70d57317278b23e68412793e25` |
| [CVE-2026-42208.rs](CVE-2026-42208.rs) | [CVE-2026-42208](../CVE_2026_42208.md) | `865ab104debef7e8d6989bcaf89882bbaef642a9039c7e23cd5a267d0c01ba92` |
| [CVE-2026-42252.rs](CVE-2026-42252.rs) | [CVE-2026-42252](../CVE_2026_42252.md) | `baaeb951315c5808bad32f42764e0cd6a9f65cf9a243a572bb565f2607b9a002` |
| [CVE-2026-42271.rs](CVE-2026-42271.rs) | [CVE-2026-42271](../CVE_2026_42271.md) | `a859def97614569210ff7d2c85ec2156c8e45185685d4572c4f32844a0dcd56e` |
| [CVE-2026-44578.rs](CVE-2026-44578.rs) | [CVE-2026-44578](../CVE_2026_44578.md) | `afdd6d5278784d34396f6c09d68544895104e52440f09a075437991ae364df5b` |
| [CVE-2026-45434.rs](CVE-2026-45434.rs) | [CVE-2026-45434](../CVE_2026_45434.md) | `1b5659c902bad9965478d26a024b47e1c03fb4cd4a2fda8793fd957aa21b33c1` |
| [CVE-2026-46442.rs](CVE-2026-46442.rs) | [CVE-2026-46442](../CVE_2026_46442.md) | `6841512281ca8320440d8e9540d5f9f266ce513206d79e8e668a35c53d342bdd` |
| [CVE-2026-46817.rs](CVE-2026-46817.rs) | [CVE-2026-46817](../CVE_2026_46817.md) | `4663701fe4a5d74a940d67694f58ccf304b6b184db3fb9476ec85eec85e35825` |
| [CVE-2026-48282.rs](CVE-2026-48282.rs) | [CVE-2026-48282](../CVE_2026_48282.md) | `5566862d05d5386c70670ff4321bde454888731687fdc50d3b7566a2089eb89e` |
| [CVE-2026-48907.rs](CVE-2026-48907.rs) | [CVE-2026-48907](../CVE_2026_48907.md) | `b25e9817524d8b5f9eea548c8b98fc2437ae78e04cb764bf6f02e56e33e015e2` |
| [CVE-2026-48908.rs](CVE-2026-48908.rs) | [CVE-2026-48908](../CVE_2026_48908.md) | `52740425825187035055bcc48d39dcaed1d4040f5f6596e1da5588510abbd44a` |
| [CVE-2026-48939.rs](CVE-2026-48939.rs) | [CVE-2026-48939](../CVE_2026_48939.md) | `72f78d1c5b246efa4a44d21e62c7913241e491248c7d095e7851b2e3aa037881` |
| [CVE-2026-56290.rs](CVE-2026-56290.rs) | [CVE-2026-56290](../CVE_2026_56290.md) | `b25e9817524d8b5f9eea548c8b98fc2437ae78e04cb764bf6f02e56e33e015e2` |
| [CVE-2026-56291.rs](CVE-2026-56291.rs) | [CVE-2026-56291](../CVE_2026_56291.md) | `b25e9817524d8b5f9eea548c8b98fc2437ae78e04cb764bf6f02e56e33e015e2` |
| [CVE-2026-59774.rs](CVE-2026-59774.rs) | [CVE-2026-59774](../CVE_2026_59774.md) | `89021bf6a58ee9b6364e8a5ff7d9e8390f12b5a82fc916a374279347b6ddaee6` |
| [CVE-2026-61447.rs](CVE-2026-61447.rs) | [CVE-2026-61447](../CVE_2026_61447.md) | `0c91c1d5a84165fbfceb0236fc1a4623299b215158cdf59e56f4399207582636` |
| [CVE-2026-61511.rs](CVE-2026-61511.rs) | [CVE-2026-61511](../CVE_2026_61511.md) | `b1a07e51f3503f129fb3221413369e6da1dc8b7a42bacfef8016bea74189bfee` |
| [CVE-2026-62071.rs](CVE-2026-62071.rs) | [CVE-2026-62071](../CVE_2026_62071.md) | `aaec294ae7159a80e62341d2be92ed7a378961bc55ce5b6f4d5121dc5e837754` |
| [CVE-2026-64849.rs](CVE-2026-64849.rs) | [CVE-2026-64849](../CVE_2026_64849.md) | `2d71e67e0854ebe981244b39b09e9e4f2a3fecfd5f4951ac1739eba84b480a30` |
| [CVE-2026-66066.rs](CVE-2026-66066.rs) | [CVE-2026-66066](../CVE_2026_66066.md) | `4e893b942d98c00313177af60032cb74047918f5612ea9f5f0df98f5b2d57072` |
| [CVE-2026-67260.rs](CVE-2026-67260.rs) | [CVE-2026-67260](../CVE_2026_67260.md) | `4c108011fc1d2f47242a89b2b3b3feaa9364fb224a5da159bb252ca896c864c3` |
| [CVE-2026-67444.rs](CVE-2026-67444.rs) | [CVE-2026-67444](../CVE_2026_67444.md) | `f5c0c96076157873d3950acf079ecec3f8271ddf1aaa4c22388a0a452f89fdb8` |
| [CVE-2026-67587.rs](CVE-2026-67587.rs) | [CVE-2026-67587](../CVE_2026_67587.md) | `4c108011fc1d2f47242a89b2b3b3feaa9364fb224a5da159bb252ca896c864c3` |
| [CVE-2026-68905.rs](CVE-2026-68905.rs) | [CVE-2026-68905](../CVE_2026_68905.md) | `5b6e256dff588016de663ff304c65d9c4fad949e85ceec79cf202d41e9508aa0` |
| [CVE-2026-68982.rs](CVE-2026-68982.rs) | [CVE-2026-68982](../CVE_2026_68982.md) | `4c108011fc1d2f47242a89b2b3b3feaa9364fb224a5da159bb252ca896c864c3` |
| [CVE-2026-71531.rs](CVE-2026-71531.rs) | [CVE-2026-71531](../CVE_2026_71531.md) | `8b654e5760cb0074e0a38c0d65347ecfd286b1252e9163220929de216bdfcbb7` |
| [CVE-2026-75157.rs](CVE-2026-75157.rs) | [CVE-2026-75157](../CVE_2026_75157.md) | `4c11b00fe816cc3f5fbb0f28045d4ee0322f267583494615269c130595c5e80d` |
| [CVE-2026-75869.rs](CVE-2026-75869.rs) | [CVE-2026-75869](../CVE_2026_75869.md) | `0406fff342e484402b338c17c74bd95a0c56ab0278e5e2653ef72fd891850030` |
| [CVE-2026-76418.rs](CVE-2026-76418.rs) | [CVE-2026-76418](../CVE_2026_76418.md) | `741d84a7d813d8104f0906ca685e7eb45f27fc55f774e6498bf012d199580929` |
| [CVE-2026-76460.rs](CVE-2026-76460.rs) | [CVE-2026-76460](../CVE_2026_76460.md) | `fbc762d401909904d5a6cc8c3f3d4cf55a56cc0ec9b28077d5d43e3b39462511` |
| [CVE-2026-76504.rs](CVE-2026-76504.rs) | [CVE-2026-76504](../CVE_2026_76504.md) | `af55aa5b1b6c3bd398352e1c9aba36c8185067dbdceadf7774bba9c25aa82262` |
| [CVE-2026-76519.rs](CVE-2026-76519.rs) | [CVE-2026-76519](../CVE_2026_76519.md) | `4e1fd38e2805a5a5a964ceb85de8589d10ed1efa11b1ce5b1fd691a58e63803b` |
| [CVE-2026-82078.rs](CVE-2026-82078.rs) | [CVE-2026-82078](../CVE_2026_82078.md) | `18efccc38195c5174c0dbc35d3b6d34456a9ffc5e5dd26460b4e1897012b0ba2` |
| [CVE-2026-82102.rs](CVE-2026-82102.rs) | [CVE-2026-82102](../CVE_2026_82102.md) | `373a13dd5ada020423492b0d2d06f50e91451d41f22eb3af2acc46cfdc8cbff0` |
| [CVE-2026-82329.rs](CVE-2026-82329.rs) | [CVE-2026-82329](../CVE_2026_82329.md) | `1f12aec235264db38b77181cedee3dc75e91a08b93aa98bacdeb41e16c5d7e7e` |
| [CVE-2026-82504.rs](CVE-2026-82504.rs) | [CVE-2026-82504](../CVE_2026_82504.md) | `ac606767d4b24e64958cd5c9d308ce5ab4d0df9c5b262d865287e7fe67009eef` |
| [CVE-2026-85706.rs](CVE-2026-85706.rs) | [CVE-2026-85706](../CVE_2026_85706.md) | `e2379cef7b9a876f5ab174e329ac985f2d7005415dfc94619551435b6ab5fe2f` |
| [CVE-2026-86829.rs](CVE-2026-86829.rs) | [CVE-2026-86829](../CVE_2026_86829.md) | `4c108011fc1d2f47242a89b2b3b3feaa9364fb224a5da159bb252ca896c864c3` |
| [CVE-2026-88771.rs](CVE-2026-88771.rs) | [CVE-2026-88771](../CVE_2026_88771.md) | `107a20c7b45c4bd924d972bda701a4ce521b3fea6409cec8127bddb37b401b45` |
| [CVE-2026-88772.rs](CVE-2026-88772.rs) | [CVE-2026-88772](../CVE_2026_88772.md) | `6cd82a750a2761b193933fcd650aa3bb808cb2e4fc9a8df3f1add9fff7766b74` |
| [CVE-2026-88773.rs](CVE-2026-88773.rs) | [CVE-2026-88773](../CVE_2026_88773.md) | `3891cb32d86f885874f681883578b307f75032449307a4307ade83b852a56564` |
| [CVE-2026-88779.rs](CVE-2026-88779.rs) | [CVE-2026-88779](../CVE_2026_88779.md) | `b4cdb2311373c00e4aaa9e6846a46ca1b612534c174893dfcd0d14081a7fee16` |
| [CVE-2026-89037.rs](CVE-2026-89037.rs) | [CVE-2026-89037](../CVE_2026_89037.md) | `eb795bbed971067fb5b80da467fca48ac6d0abdb27f9dab562d464baf6b2a5e3` |
| [CVE-2026-90970.rs](CVE-2026-90970.rs) | [CVE-2026-90970](../CVE_2026_90970.md) | `10f00e6cce8f8f380c5c3b114afe5936612f12e2f978ce236d7e98ab713c163e` |
| [CVE-2026-92738.rs](CVE-2026-92738.rs) | [CVE-2026-92738](../CVE_2026_92738.md) | `7f8270d6c2897f3dd1d9aec5b45a1fa5f76839d390d3c57cc0f8a1e4d75ff1ce` |
| [CVE-2026-93616.rs](CVE-2026-93616.rs) | [CVE-2026-93616](../CVE_2026_93616.md) | `fb2c467a224dde047e43d3322eb58e7e6c3f1f42daa1c6e048bbb636aa187035` |
| [CVE-2026-93674.rs](CVE-2026-93674.rs) | [CVE-2026-93674](../CVE_2026_93674.md) | `8fe072b4ea30245e062c83f3fe74e7cb09af6a3840070d560e6dae2aefbf6764` |
| [CVE-2026-93952.rs](CVE-2026-93952.rs) | [CVE-2026-93952](../CVE_2026_93952.md) | `85e98d2b1ca2bcbb3a4da2ad497eaa21608c27ee52ee44d15215ea93660359b2` |
| [CVE-2026-94127.rs](CVE-2026-94127.rs) | [CVE-2026-94127](../CVE_2026_94127.md) | `96395721b0bd970dd917e8233caaf89df99daede72b2137ef58ab226322848bf` |
| [CVE-2026-96654.rs](CVE-2026-96654.rs) | [CVE-2026-96654](../CVE_2026_96654.md) | `56ee297c40c4a0225874af1cdf36f96d79f4c78e93bd6f76e6df42355342bc26` |
| [CVE-2026-96655.rs](CVE-2026-96655.rs) | [CVE-2026-96655](../CVE_2026_96655.md) | `ff33ca15f7424d54ee0842bccf25d9a8e73c00d41516d77d09856dceb58af9f8` |
| [CVE-2026-96656.rs](CVE-2026-96656.rs) | [CVE-2026-96656](../CVE_2026_96656.md) | `b6cd460e9fbd524de2acd003edce87955477369569bc6f4a165eb82897a98979` |
| [CVE-2026-97229.rs](CVE-2026-97229.rs) | [CVE-2026-97229](../CVE_2026_97229.md) | `2082f1e7c95633caa49fa43389df01fd9f4a2440f1ddd24dd98b33ae62d92b6d` |
| [CVE-2026-103980.rs](CVE-2026-103980.rs) | [CVE-2026-103980](../CVE_2026_103980.md) | `e905e9ee089a55dbf9f031fdf867d37164b469696153067a0acdadccadd83b0e` |
| [CVE-2026-104150.rs](CVE-2026-104150.rs) | [CVE-2026-104150](../CVE_2026_104150.md) | `0cbf3e64021be2ee9d0d50878b8a5378d0b32fcf71ea7a0acbfb0f169f9cfdba` |
| [CVE-2026-104286.rs](CVE-2026-104286.rs) | [CVE-2026-104286](../CVE_2026_104286.md) | `7411c95b689a950a9f4fa637863d0a23c60481076c6a2e828f1b854e06dbe4a4` |
| [CVE-2026-104935.rs](CVE-2026-104935.rs) | [CVE-2026-104935](../CVE_2026_104935.md) | `3c10109061df6dbd71335209f5fca8a90f26c7be100fd0fed9745ccf18469ad1` |
| [CVE-2026-105133.rs](CVE-2026-105133.rs) | [CVE-2026-105133](../CVE_2026_105133.md) | `5deff1fa30f9decefae1f42ae729382103c03d0ed117e1d7a3e2e30d3c4f232d` |
| [CVE-2026-105134.rs](CVE-2026-105134.rs) | [CVE-2026-105134](../CVE_2026_105134.md) | `83bc31924ba206b50e58ec773d90b7e14e863b3bb00516787a095306e38c2d08` |
| [CVE-2026-106042.rs](CVE-2026-106042.rs) | [CVE-2026-106042](../CVE_2026_106042.md) | `a93cce2734a7302150328b5b884be3af554c72d402c8c51a0e110a81de3abd75` |
| [CVE-2026-106931.rs](CVE-2026-106931.rs) | [CVE-2026-106931](../CVE_2026_106931.md) | `fb21cbf1a38b3bc000139be735c9f965dbd5e64681a87ae60403b8ff175695c4` |
| [CVE-2026-107412.rs](CVE-2026-107412.rs) | [CVE-2026-107412](../CVE_2026_107412.md) | `311769777c14941f98b376b9e3cfeee0534a79a6697d99b53900223f21cb31d5` |
| [CVE-2026-107415.rs](CVE-2026-107415.rs) | [CVE-2026-107415](../CVE_2026_107415.md) | `e3381175eba280b1a1ee45f1064d0ba854d9586ae0b3a282e5e4a3d1533bcad3` |
| [CVE-2026-107656.rs](CVE-2026-107656.rs) | [CVE-2026-107656](../CVE_2026_107656.md) | `6eb65c12d86f9ac4724ca8b25a89292c5534b586284d663fcb2436eb3c1cadc5` |
| [CVE-2026-107718.rs](CVE-2026-107718.rs) | [CVE-2026-107718](../CVE_2026_107718.md) | `6e686c6b7b0fd4d7e837348d1aeb3160ab00f71423a1962d0ef2d749fb5da31c` |
| [CVE-2026-110178.rs](CVE-2026-110178.rs) | [CVE-2026-110178](../CVE_2026_110178.md) | `794710eb53b0f0e4d4ea9a3b1565411c8fb52d767d8bf9c3250bedf6bb738f37` |
| [CVE-2026-110194.rs](CVE-2026-110194.rs) | [CVE-2026-110194](../CVE_2026_110194.md) | `5733c32c6ab0b5dbc2fa555851f73b97f420891fecf3e68cd7b74edaf4a2ae74` |
| [CVE-2026-110264.rs](CVE-2026-110264.rs) | [CVE-2026-110264](../CVE_2026_110264.md) | `18dfc51acc245c1bee2f46c4ce3f7196e4984932c12e3e1c2d33e33bc8ad7ea6` |
| [CVE-2026-110279.rs](CVE-2026-110279.rs) | [CVE-2026-110279](../CVE_2026_110279.md) | `df294687805fe0bdaffe424bd73c88184a7d450a295cfd00679e3364d78ad950` |
| [CVE-2026-110447.rs](CVE-2026-110447.rs) | [CVE-2026-110447](../CVE_2026_110447.md) | `9eb549f48c59cfae8e80c29cbd20fc73f22d01616e5bd7d672ff056acd50ff9a` |
| [CVE-2026-110697.rs](CVE-2026-110697.rs) | [CVE-2026-110697](../CVE_2026_110697.md) | `4a959126e64bea4a72505b2c85bbfdcb7017f0b2ce14f666a387f59ea2324841` |
| [CVE-2026-111214.rs](CVE-2026-111214.rs) | [CVE-2026-111214](../CVE_2026_111214.md) | `78ccfe2e63b13458dbbcaddec679a24e1590b37b692e23d88767b0b33b28df2a` |
| [CVE-2026-111334.rs](CVE-2026-111334.rs) | [CVE-2026-111334](../CVE_2026_111334.md) | `1c1c03c0660134cb2030aec2d8ee8d4003d2ebd23da9532ff77b2b6f48d8cc0a` |
| [CVE-2026-111497.rs](CVE-2026-111497.rs) | [CVE-2026-111497](../CVE_2026_111497.md) | `9e1db1f36974de15ca255d6252a22f321e7fcc1dba30be9c222768d3b9549fc2` |
| [CVE-2026-114136.rs](CVE-2026-114136.rs) | [CVE-2026-114136](../CVE_2026_114136.md) | `8792145cc25b31217dc6d78956ed24820d2251cc251a64badbc7d754bfccc2b3` |
| [CVE-2026-114509.rs](CVE-2026-114509.rs) | [CVE-2026-114509](../CVE_2026_114509.md) | `2faf56e8723148f7c6ab8be83a79f41496ae1620a989f151bedf56aafc111fd3` |
| [CVE-2026-116241.rs](CVE-2026-116241.rs) | [CVE-2026-116241](../CVE_2026_116241.md) | `a8ead4886194e722f9a17507662bc30bca748ec9415cf4275e92d2aab3d64930` |
| [CVE-2026-118527.rs](CVE-2026-118527.rs) | [CVE-2026-118527](../CVE_2026_118527.md) | `27d08b99456bc20f35b7b345c10cd3e80a4450101a00911971729978fc0e5422` |
| [CVE-2026-119977.rs](CVE-2026-119977.rs) | [CVE-2026-119977](../CVE_2026_119977.md) | `1c6006a98ad96e1c2c66fc6e0a20b55212addb72c2316e458b9163e807f6ae4d` |
| [CVE-2026-120522.rs](CVE-2026-120522.rs) | [CVE-2026-120522](../CVE_2026_120522.md) | `d1d7675460525eaf4515cabd7fad2bb1753d9bda9fff6daa827ff5f5c726f0e7` |
| [CVE-2026-120869.rs](CVE-2026-120869.rs) | [CVE-2026-120869](../CVE_2026_120869.md) | `1d61affa64801487141e05a79d5645a118cf8e4f3f7ab8ce2a16ca11811605aa` |
| [CVE-2026-121653.rs](CVE-2026-121653.rs) | [CVE-2026-121653](../CVE_2026_121653.md) | `3b55f8b74f30696e7cad4c73189a9536375b005c7b03e6019f21935d44082696` |
| [CVE-2026-123628.rs](CVE-2026-123628.rs) | [CVE-2026-123628](../CVE_2026_123628.md) | `fe29cf30541b24656b98b93a262c3ba4b71ac8f9c01263e82cec1b1e00928db0` |
| [CVE-2026-123747.rs](CVE-2026-123747.rs) | [CVE-2026-123747](../CVE_2026_123747.md) | `2b6f29b8dd7e1e331493a558e5a46b6c9b99886fd4e9d5f26fcc3866890dda51` |
| [CVE-2026-123790.rs](CVE-2026-123790.rs) | [CVE-2026-123790](../CVE_2026_123790.md) | `71d162a9ba0641cdc15397f779ece829d9a75a3e1c1f1b342d4f22b227107042` |
| [CVE-2026-124020.rs](CVE-2026-124020.rs) | [CVE-2026-124020](../CVE_2026_124020.md) | `899e054174a6f52bcc7dd04b9f28537046f55104398a37da1cb0c0ab672418ca` |
| [CVE-2026-124021.rs](CVE-2026-124021.rs) | [CVE-2026-124021](../CVE_2026_124021.md) | `7991365429eb0e4ff3ed25d3b2f9e88c0cd8d76814f47fc5f74515b78c4ac9d2` |
| [CVE-2026-124691.rs](CVE-2026-124691.rs) | [CVE-2026-124691](../CVE_2026_124691.md) | `bdadf1d9aec2ac7e5dc1fa0d2cc515d6a0614b6d2866d1a1f938cd273d43f2a3` |
| [CVE-2026-124695.rs](CVE-2026-124695.rs) | [CVE-2026-124695](../CVE_2026_124695.md) | `40473801a0f77fe4d5482f629342a95491f5b5a9486a1c0b920f9db1902a32ae` |
| [CVE-2026-124696.rs](CVE-2026-124696.rs) | [CVE-2026-124696](../CVE_2026_124696.md) | `0bf7c3f1a5ca5a8a3e3ac6cbd8ca3b0408722de1953942af1b9f7b126eceb594` |
| [CVE-2026-124697.rs](CVE-2026-124697.rs) | [CVE-2026-124697](../CVE_2026_124697.md) | `260f50e18d7ef13890c13f3c0ad45488dcb050308f0a4c06efa8246bb8d02922` |
| [CVE-2026-124698.rs](CVE-2026-124698.rs) | [CVE-2026-124698](../CVE_2026_124698.md) | `49846cc0bb5fdce88a8792b09d1be14ff93c1541c683b783d6aef8388b6ccdd1` |
| [CVE-2026-126147.rs](CVE-2026-126147.rs) | [CVE-2026-126147](../CVE_2026_126147.md) | `805e7e32b4eeff8da32925068472620cab47d303ac7a4e84b4deab0ef0295e8a` |
| [CVE-2026-126150.rs](CVE-2026-126150.rs) | [CVE-2026-126150](../CVE_2026_126150.md) | `e5bb52b49d473648e8d9e1a1031714e58b958a724d6aeb5929bd9fe1f9b295b4` |
| [CVE-2026-126155.rs](CVE-2026-126155.rs) | [CVE-2026-126155](../CVE_2026_126155.md) | `cd040cfc370dde2791eac2a8d2d1ac0aae958e686297a13e47f777707340407d` |
| [CVE-2026-126156.rs](CVE-2026-126156.rs) | [CVE-2026-126156](../CVE_2026_126156.md) | `35c0b6498f47746651c061184e73b72742715f58c5c959598a69d6681a776971` |
| [CVE-2026-126157.rs](CVE-2026-126157.rs) | [CVE-2026-126157](../CVE_2026_126157.md) | `4862569da1b71d5ecae40fd5a5bc6a0a5f041bd3334bd2e346acf0606ad24ef0` |
| [CVE-2026-126164.rs](CVE-2026-126164.rs) | [CVE-2026-126164](../CVE_2026_126164.md) | `784fd5809839b90d54c267072f37585369b08dd80250daa99080f51d29664eec` |
| [CVE-2026-126170.rs](CVE-2026-126170.rs) | [CVE-2026-126170](../CVE_2026_126170.md) | `66f67073c1e01ad1cc6948489780b5e9d28b77d67f76395ac1cf80ffdcf67017` |
| [CVE-2026-126229.rs](CVE-2026-126229.rs) | [CVE-2026-126229](../CVE_2026_126229.md) | `351ac8d1b8dbbb30535b67c08f890b34d833ad36271a7982f5e8f66826cbab91` |
| [CVE-2026-126251.rs](CVE-2026-126251.rs) | [CVE-2026-126251](../CVE_2026_126251.md) | `11774ff1b6051978e7c2136df4c597f0b07dcd13c97a28622275360907f8cea9` |
| [CVE-2026-126257.rs](CVE-2026-126257.rs) | [CVE-2026-126257](../CVE_2026_126257.md) | `ffdce3dd13e654b70f862360fc2d47d73c447c1b59fb035cad1703fdd22c7e33` |
| [CVE-2026-126263.rs](CVE-2026-126263.rs) | [CVE-2026-126263](../CVE_2026_126263.md) | `59d52bc506d482814372163cce54e500f0d838957953331bc8e20625b92e44dc` |
| [CVE-2026-126768.rs](CVE-2026-126768.rs) | [CVE-2026-126768](../CVE_2026_126768.md) | `042621d5d0fb9721fb60b70143cc945ea6ede36092905ab3e80f8d5b5fd61f13` |
| [CVE-2026-126769.rs](CVE-2026-126769.rs) | [CVE-2026-126769](../CVE_2026_126769.md) | `68c5e9b16ea6eee934724994ec44926c0c4619d102cbd0a7dc468c66e4fd3aa1` |
| [CVE-2026-126777.rs](CVE-2026-126777.rs) | [CVE-2026-126777](../CVE_2026_126777.md) | `013299b0749e78efc867effbda0252801992879bc774fb5cd2833acd4edabc61` |
| [CVE-2026-126800.rs](CVE-2026-126800.rs) | [CVE-2026-126800](../CVE_2026_126800.md) | `cfe2a6e49b2ec22ef75073da34e758e751c29af87c139c01b1b029a8dddd8716` |
| [CVE-2026-126804.rs](CVE-2026-126804.rs) | [CVE-2026-126804](../CVE_2026_126804.md) | `8e4a97d45e31baf38e5e947328afaeaa1442e9e03a16073e52e65afb23741da0` |
| [CVE-2026-126806.rs](CVE-2026-126806.rs) | [CVE-2026-126806](../CVE_2026_126806.md) | `80569e2ce4d381afa3d1f5b438814ea48ef76e9f082990e7392a0556d05aa526` |
| [CVE-2026-126997.rs](CVE-2026-126997.rs) | [CVE-2026-126997](../CVE_2026_126997.md) | `ca443a1c963b60dee4aa0986084c02c1eb82f744d6b5304c9ea5acc3f3591a44` |
| [CVE-2026-126998.rs](CVE-2026-126998.rs) | [CVE-2026-126998](../CVE_2026_126998.md) | `cf5746b932895abeff11d95883dbeb042b7b8cb0e22884917b792bb58436219f` |
| [CVE-2026-127111.rs](CVE-2026-127111.rs) | [CVE-2026-127111](../CVE_2026_127111.md) | `25a9fb1f2e0d9d0a283d9362c828ceec2278cbe24c23eab9f65a88efa67f4482` |
| [CVE-2026-127121.rs](CVE-2026-127121.rs) | [CVE-2026-127121](../CVE_2026_127121.md) | `1c52b148b7bbb6e3dc6d1d864a8bd11c6c4011b93d909f5420d18ef5cd26f797` |
| [CVE-2026-127135.rs](CVE-2026-127135.rs) | [CVE-2026-127135](../CVE_2026_127135.md) | `9ef5b15bc983c908102531c99f3039e9bb5ccaba7db4d652264370c2ecd6a906` |
| [CVE-2026-127145.rs](CVE-2026-127145.rs) | [CVE-2026-127145](../CVE_2026_127145.md) | `f46abc24e1f334760c51afb337433453f2c96b52ea1df3cd8be461116afb0dfa` |
| [CVE-2026-127202.rs](CVE-2026-127202.rs) | [CVE-2026-127202](../CVE_2026_127202.md) | `726866334c147cbd6a822e673211012110b94ac19e6dd5ab96fffa3028e40ed4` |
| [CVE-2026-127203.rs](CVE-2026-127203.rs) | [CVE-2026-127203](../CVE_2026_127203.md) | `87c02f6326bc8ee161b51b5ccce3f858e3d7c5719fb031e3391124efd5426590` |
| [CVE-2026-21643.rs](CVE-2026-21643.rs) | [CVE-2026-21643](../CVE_2026_21643.md) | `2a133e17b385743e3dee54870c24fcc243aa6295ef7886686e038f2715710d5a` |
| [CVE-2026-42356.rs](CVE-2026-42356.rs) | [CVE-2026-42356](../CVE_2026_42356.md) | `803fb460e82cbd3e897c359bd4679a94566a0c8285ac6e94cd8de66042e3d169` |
| [CVE-2026-42528.rs](CVE-2026-42528.rs) | [CVE-2026-42528](../CVE_2026_42528.md) | `1069f3cee44936ee1c434fcc60c840e1f81abbd7239fc97d4d51ddddcab1b649` |
| [CVE-2026-42536.rs](CVE-2026-42536.rs) | [CVE-2026-42536](../CVE_2026_42536.md) | `9c4c66a33dd641181b0aaf4c4e18b8af9a736a3887937e3dc50020b8a7e3c00d` |
| [CVE-2026-44185.rs](CVE-2026-44185.rs) | [CVE-2026-44185](../CVE_2026_44185.md) | `30356dad432f4f70df260ecedfd8b84060fe694ee21d4b92a29f0dd2de35bbd1` |
| [CVE-2026-46729.rs](CVE-2026-46729.rs) | [CVE-2026-46729](../CVE_2026_46729.md) | `fd750b6ebf201b5227059e2930fb25174d9d66927fdff4ab8badc1eaf8b9b858` |
| [CVE-2026-56153.rs](CVE-2026-56153.rs) | [CVE-2026-56153](../CVE_2026_56153.md) | `6159b05aaefef6caf1fce10ca6267210469b637d8b41bdc4270089f56cfd67b3` |
| [CVE-2026-56154.rs](CVE-2026-56154.rs) | [CVE-2026-56154](../CVE_2026_56154.md) | `8f5b347eeae4f1b0b71e03236cb9bb55334f606b12bc754ca3402892246c8b78` |
| [CVE-2026-56449.rs](CVE-2026-56449.rs) | [CVE-2026-56449](../CVE_2026_56449.md) | `fbc0d6e484ab027fa399853b1185ec485dcbbd8604f0200ebd8eb230c223bc0a` |
| [CVE-2026-57941.rs](CVE-2026-57941.rs) | [CVE-2026-57941](../CVE_2026_57941.md) | `03fbeccb39751557c164c9c5a8e6f764c0084dd2b7bcf76896cfc19ffba2ac9e` |
| [CVE-2026-58415.rs](CVE-2026-58415.rs) | [CVE-2026-58415](../CVE_2026_58415.md) | `ca591ffdf37859c08e502ae821d8b57781c08e9a85a46bc3dbb34c4e1b8dd9f1` |
| [CVE-2026-59685.rs](CVE-2026-59685.rs) | [CVE-2026-59685](../CVE_2026_59685.md) | `9ae4bde5ec3c18a388b52b9b5c9c3b9bc3ce3c546efc7a202fb14628570f23be` |
| [CVE-2026-59797.rs](CVE-2026-59797.rs) | [CVE-2026-59797](../CVE_2026_59797.md) | `b200df5278d5461b5bcd34006a9aec356172536ca95b1ac203c95b49ddee8c18` |
| [CVE-2026-61526.rs](CVE-2026-61526.rs) | [CVE-2026-61526](../CVE_2026_61526.md) | `de7c1e0da876ed4b6f5e7e1601f176c2fc3a4ddc324c8a3572766ece3f3880c3` |
| [CVE-2026-63045.rs](CVE-2026-63045.rs) | [CVE-2026-63045](../CVE_2026_63045.md) | `c717cf75823fcf71466a03330f0b7f58f7e5cdddd38fd21930751571a74eeee2` |
| [CVE-2026-63292.rs](CVE-2026-63292.rs) | [CVE-2026-63292](../CVE_2026_63292.md) | `3a845d41a414ad4ae5c9851a7229f4fe273c04836bbd654d07f21879ebb60b0b` |
| [CVE-2026-63686.rs](CVE-2026-63686.rs) | [CVE-2026-63686](../CVE_2026_63686.md) | `70333b5fb2b3f0e3da5156d32d0314ede8490f81330ef878371250ce3c655734` |
| [CVE-2026-63718.rs](CVE-2026-63718.rs) | [CVE-2026-63718](../CVE_2026_63718.md) | `4c9bf28d8206cffb193fd7a25cd816f51df7924af8a60bb4ad28be6924e43feb` |
| [CVE-2026-67449.rs](CVE-2026-67449.rs) | [CVE-2026-67449](../CVE_2026_67449.md) | `5375c3dca12a28cd6cf594a7c5278eb5e47316ecb18ade822047d11e3a98b039` |
| [CVE-2026-70642.rs](CVE-2026-70642.rs) | [CVE-2026-70642](../CVE_2026_70642.md) | `008455dffb1a102fb32c66d1ac2588aabbb2e7cf10ef81f5b70439011e4d85b4` |
| [CVE-2026-72898.rs](CVE-2026-72898.rs) | [CVE-2026-72898](../CVE_2026_72898.md) | `75f7f680eaff3750ae9f59a2cb49d4aad2dc2f34469cd6ab732fb480a2b78cd5` |
| [CVE-2026-73637.rs](CVE-2026-73637.rs) | [CVE-2026-73637](../CVE_2026_73637.md) | `14402be7a63ec64afe26b8544bcaf9f0e15608837e809c1b890c29f9964ba5cb` |
| [CVE-2026-76461.rs](CVE-2026-76461.rs) | [CVE-2026-76461](../CVE_2026_76461.md) | `a54f59724a12c1751d1bb41913d5475e8119d23d659b2e1a88948a195d68f140` |
| [CVE-2026-79768.rs](CVE-2026-79768.rs) | [CVE-2026-79768](../CVE_2026_79768.md) | `cd0ff87b4fcb7acc44efbd8bac8d937d2d9d5d68def3f921362dfb1c5fb2a62d` |
| [CVE-2026-82977.rs](CVE-2026-82977.rs) | [CVE-2026-82977](../CVE_2026_82977.md) | `f2a0d634f945ecee9408c8cc7a38780519fb768ea7c225a1314651a1bf0c7ceb` |
| [CVE-2026-87902.rs](CVE-2026-87902.rs) | [CVE-2026-87902](../CVE_2026_87902.md) | `79028f1cb227dfaf64d04b641b5f02dcc25eb5f5b9f9d9557b2adcf491724858` |
| [CVE-2026-9082.rs](CVE-2026-9082.rs) | [CVE-2026-9082](../CVE_2026_9082.md) | `0a5fb5f4e30ef8c1475c32626694dfe9b488405f40ea8f79c0e55d9bf9e39c6c` |
| [CVE-2026-9198.rs](CVE-2026-9198.rs) | [CVE-2026-9198](../CVE_2026_9198.md) | `306e2c1d7e5c9fee797c01bef54f03b3ed653af539963549add73c89486022de` |
| [CVE-2026-93546.rs](CVE-2026-93546.rs) | [CVE-2026-93546](../CVE_2026_93546.md) | `c5781df4ac6c2fda2c52cdb0aea635cc05ed59defd3db831cb9cde087ac23a98` |
| [CVE-2026-127213.rs](CVE-2026-127213.rs) | [CVE-2026-127213](../CVE_2026_127213.md) | `7994d15d37b12692df9ce902817d61360a1da553616c601e6ba6fdff17d40df6` |
| [CVE-2026-127214.rs](CVE-2026-127214.rs) | [CVE-2026-127214](../CVE_2026_127214.md) | `fb517ab95f270842094aa6de4c9d76f59c2cb35bc41274d27f425826cb00ca51` |
| [CVE-2026-127215.rs](CVE-2026-127215.rs) | [CVE-2026-127215](../CVE_2026_127215.md) | `1dadbbd7b6853b74cc35db95ebf7add89ae0a2198a436a208616128b018c2d15` |
| [CVE-2026-127216.rs](CVE-2026-127216.rs) | [CVE-2026-127216](../CVE_2026_127216.md) | `d7df4bfd6f2d09a588780eb20aff0a7e42758862fe130fa9f1c1fbef20350e2c` |
| [CVE-2026-127217.rs](CVE-2026-127217.rs) | [CVE-2026-127217](../CVE_2026_127217.md) | `2893592c1ffac596b05a5ee2590807500f729b46aaa9251f82516a8725ac36a5` |
| [CVE-2026-128014.rs](CVE-2026-128014.rs) | [CVE-2026-128014](../CVE_2026_128014.md) | `bf1888ed52cd457bbb607156f3bd2d34056c7d2a2adbc58760cb0b3aa829e6d0` |
| [CVE-2026-128023.rs](CVE-2026-128023.rs) | [CVE-2026-128023](../CVE_2026_128023.md) | `e245c6a836da844c0e6d74f566dfbd9c475b1962110cc9672b00d4aac79c09ea` |
| [CVE-2026-128027.rs](CVE-2026-128027.rs) | [CVE-2026-128027](../CVE_2026_128027.md) | `bcf92b06eef3108c7fffabaa71ec51bb92153d1ba9e15c130fcda3a96afeabd6` |
| [CVE-2026-128119.rs](CVE-2026-128119.rs) | [CVE-2026-128119](../CVE_2026_128119.md) | `e4b217a54141e5e8dfbfd0081c29ae18aca7cc1685c861e6b98659aab1332cf7` |
| [CVE-2026-128120.rs](CVE-2026-128120.rs) | [CVE-2026-128120](../CVE_2026_128120.md) | `bda4c08775bc3a922b4059ffcedfadd015b8fa7d36c067d16292ff13aaea02d7` |
| [CVE-2026-128124.rs](CVE-2026-128124.rs) | [CVE-2026-128124](../CVE_2026_128124.md) | `e44076bd90c090e6171489f68856ed3ca6d8ddcb641f2a1efcfe3be0963667f0` |
| [CVE-2026-128125.rs](CVE-2026-128125.rs) | [CVE-2026-128125](../CVE_2026_128125.md) | `02145ffb8f22431d9a30c579703e7b37fb02e0d63bbceb8cfd8ec78135bb3896` |
| [CVE-2026-128126.rs](CVE-2026-128126.rs) | [CVE-2026-128126](../CVE_2026_128126.md) | `69671253a6533406e3e8a8221807e9a6c6dc73ec469ea5298531e77439e335b7` |
| [CVE-2026-128127.rs](CVE-2026-128127.rs) | [CVE-2026-128127](../CVE_2026_128127.md) | `0bedb0a4f025abf1a45288462a8ce0c47e6992c41d004657f217dc04df6d8845` |
| [CVE-2026-128130.rs](CVE-2026-128130.rs) | [CVE-2026-128130](../CVE_2026_128130.md) | `847dfeff7e0112c70cc8ddd274a52eaf0a554f16dd479e7164eb315cc42861d2` |
| [CVE-2026-128131.rs](CVE-2026-128131.rs) | [CVE-2026-128131](../CVE_2026_128131.md) | `b63372ffd435be85e0eb1079f7995abb0ae506116f148d11df4074d67c732bdb` |
| [CVE-2026-129161.rs](CVE-2026-129161.rs) | [CVE-2026-129161](../CVE_2026_129161.md) | `0f3a8813e74cf0c8ea6dc4d1faa913e5ba12f8fc240acd6928207d13426478c4` |
| [CVE-2026-129165.rs](CVE-2026-129165.rs) | [CVE-2026-129165](../CVE_2026_129165.md) | `a2f3de3000a13ac1a6930db831ff089e111f9d82bd91983846a58ef814dbab01` |
| [CVE-2026-129167.rs](CVE-2026-129167.rs) | [CVE-2026-129167](../CVE_2026_129167.md) | `2ce770bd106b00a5ba7e8e35fd1a899e53ec6a63c48e6c3a94a6407919055175` |
| [CVE-2026-129171.rs](CVE-2026-129171.rs) | [CVE-2026-129171](../CVE_2026_129171.md) | `5dc4d2002da5a710de7227a7685bf9d97dbd64fcf775649297bfe185bf6ed78a` |
| [CVE-2026-86863.rs](CVE-2026-86863.rs) | [CVE-2026-86863](../CVE_2026_86863.md) | `0d06ccd5b08b3a9cb042bd23d6135ef2ce6b263f965228b100b84195920ea384` |
| [CVE-2026-129180.rs](CVE-2026-129180.rs) | [CVE-2026-129180](../CVE_2026_129180.md) | `3be83f9af83290aa6a950d3f2982def30eca96e5304490ed3802461357fff248` |
| [CVE-2026-71519.rs](CVE-2026-71519.rs) | [CVE-2026-71519](../CVE_2026_71519.md) | `bb6ea5bec03835a4a24d68a1785a380fe8bd17446c95cda53998bc79d9320ad6` |
| [CVE-2026-129250.rs](CVE-2026-129250.rs) | [CVE-2026-129250](../CVE_2026_129250.md) | `e1665d478bfe35b27b59585029fdd150b8a78173025c2066ddd3c77119a4b246` |
| [CVE-2026-129258.rs](CVE-2026-129258.rs) | [CVE-2026-129258](../CVE_2026_129258.md) | `d49ab97d3f9a5e9469509ff19a892437fee6aeb37cf857fb55ed77ad97501b48` |
| [CVE-2026-129259.rs](CVE-2026-129259.rs) | [CVE-2026-129259](../CVE_2026_129259.md) | `5d678f2fe1e45e2daf79240b2e44e04dd68e0cc3b26a1b7f21c87139c652858a` |
| [CVE-2026-129260.rs](CVE-2026-129260.rs) | [CVE-2026-129260](../CVE_2026_129260.md) | `f9e7eff5bf9f1211216ae2671f6b14c785c28ad0daf559542faaf2ec7c0a3e20` |
| [CVE-2026-129428.rs](CVE-2026-129428.rs) | [CVE-2026-129428](../CVE_2026_129428.md) | `b732f4818fa06f2cd0289053b59e340f81946dcf351d9ce6722a30a66e827f03` |
| [CVE-2026-129433.rs](CVE-2026-129433.rs) | [CVE-2026-129433](../CVE_2026_129433.md) | `e8f5d1ab5f76945ba1369413acb2b06c29606b11d3fb066c7476d1cd342da84c` |
>>>>>>> f700dc5 (docs(cve): publish Gabriel wave 38 — fourteen Apache Airflow advisories)
