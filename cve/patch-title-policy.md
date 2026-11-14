# Patch title reservations

`patch-titles.json` maps each published registry package name to its CVE.
Keep each name associated with the same CVE across releases. Use a
descriptive title and its lowercase hyphenated form for the package name.
Record the CVE separately and use the registry's SemVer field for releases.

Add a new name to this file in the same commit as the public case study and
bundle. Keep existing entries for previously published names. The automated
publisher updates the index when it publishes a new case.
