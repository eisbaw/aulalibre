---
id: TASK-0079
title: Map Xamarin crc64 Android Callable Wrapper namespaces to .NET assemblies
status: To Do
assignee: []
created_date: '2026-03-18 23:42'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
classes2.dex contains 100+ crc64* namespaces which are Xamarin Android Callable Wrappers (ACWs). Each wraps a .NET class and delegates via mono.android.Runtime. Mapping these to their .NET assembly origins would reveal the complete Java-to-.NET bridge surface and help understand which .NET types are exposed to the Android platform layer.
<!-- SECTION:DESCRIPTION:END -->
