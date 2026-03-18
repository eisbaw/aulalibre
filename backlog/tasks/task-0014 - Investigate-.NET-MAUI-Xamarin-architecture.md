---
id: TASK-0014
title: Investigate .NET MAUI/Xamarin architecture
status: To Do
assignee: []
created_date: '2026-03-18 13:59'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The Aula app uses Mono/.NET runtime (libmonodroid.so, libmonosgen-2.0.so, libassemblies.x86_64.blob.so). This means the app logic is in .NET assemblies, not Java/Kotlin DEX code. The assemblies blob needs to be extracted and decompiled with .NET tools (ILSpy/dnSpy) rather than jadx. This fundamentally changes the reverse engineering approach.
<!-- SECTION:DESCRIPTION:END -->
