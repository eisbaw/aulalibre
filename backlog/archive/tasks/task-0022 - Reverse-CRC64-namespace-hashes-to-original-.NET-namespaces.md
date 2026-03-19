---
id: TASK-0022
title: Reverse CRC64 namespace hashes to original .NET namespaces
status: To Do
assignee: []
created_date: '2026-03-18 14:14'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The 179 unique CRC64 namespace hashes in the DEX files (e.g. crc649881f3fa1611df58) map to original .NET namespaces. By extracting the .NET assemblies from libassemblies.x86_64.blob.so and computing CRC64 of their namespace strings, we can build a complete hash-to-namespace lookup table. This would let us understand which .NET namespace each Android Callable Wrapper belongs to.
<!-- SECTION:DESCRIPTION:END -->
