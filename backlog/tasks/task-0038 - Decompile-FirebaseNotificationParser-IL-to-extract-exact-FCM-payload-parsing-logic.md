---
id: TASK-0038
title: >-
  Decompile FirebaseNotificationParser IL to extract exact FCM payload parsing
  logic
status: To Do
assignee: []
created_date: '2026-03-18 16:04'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
monodis crashes on full IL disassembly of the main assemblies. Try alternative decompilation tools (ilspycmd, dnSpy, dotnet-ildasm) to extract the method body of FirebaseNotificationParser to see exact FCM data key extraction and mapping logic. This would confirm the payload format documented from field-level analysis.
<!-- SECTION:DESCRIPTION:END -->
