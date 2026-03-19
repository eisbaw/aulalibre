---
id: TASK-0024
title: Decompile CRC64 app classes with jadx for detailed analysis
status: Done
assignee:
  - '@claude'
created_date: '2026-03-18 14:14'
updated_date: '2026-03-19 05:39'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The 678 CRC64 classes in classes2.dex are Android Callable Wrappers for the Aula .NET code. Decompiling them with jadx would reveal: activity lifecycle implementations, view binding patterns, intent extras/data passing between screens, WebView JavaScript bridge interfaces, and Firebase messaging service implementation. Focus on classes2.dex only.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 classes2.dex decompiled with jadx to readable Java source
- [x] #2 Key activity lifecycle patterns documented (what data is passed between screens via intents)
- [x] #3 WebView JavaScript bridge interfaces identified if any
- [x] #4 Firebase messaging service implementation analyzed
- [x] #5 Findings documented in a committed analysis file
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Run jadx on classes2.dex to decompile to Java source\n2. Survey the decompiled output structure\n3. Analyze Activity classes for lifecycle patterns and intent data passing\n4. Search for WebView setup and JavaScript bridge interfaces\n5. Analyze AulaFirebaseMessagingService implementation\n6. Cross-reference with CRC64 namespace mapping and decompiled C# source\n7. Document findings\n8. Commit and finalize task
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Decompiled classes2.dex with jadx 1.5.0 (--show-bad-code flag, 1 error). Output is 3,322 Java files in jadx_classes2_output/ (gitignored). All 678 CRC64 classes are pure ACW pass-throughs with zero Java-side logic. No @JavascriptInterface bridges found anywhere. Firebase service implementation cross-referenced with decompiled C# -- uses elementId/type data payload keys, empty OnNewToken. Analysis documented in task24_jadx_classes2_analysis.md.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Decompiled classes2.dex (4.8 MB) with jadx 1.5.0, producing 3,322 Java source files across 179 CRC64 packages (678 Aula ACW classes).\n\nKey findings:\n- All ACW classes are structurally identical: Java method -> native JNI call -> Mono runtime. Zero application logic in Java.\n- Class distribution: ~121 Activities, ~82 Fragments, ~186 ViewHolders, ~123 Adapters, 4 WebViews, 3 WebViewClients, 3 Services, 4 BroadcastReceivers\n- No @JavascriptInterface annotations or addJavascriptInterface() calls exist. WebView communication uses URL interception (shouldOverrideUrlLoading) and HTTP auth handling.\n- Firebase messaging: extracts elementId and type from data payload, distributes via MessagingCenter event bus. OnNewToken is empty (no token refresh handling).\n- AulaPortalWebView.AulaWebViewClient handles onReceivedHttpAuthRequest for portal widget SSO.\n- MainApplication implements full ActivityLifecycleCallbacks with pre/post hooks.\n- 644 of 678 registered types belong to AulaNative.Droid assembly; remainder from Microsoft.Maui.Essentials, Xamarin.AndroidX.*, Plugin.SecureStorage, Plugin.Fingerprint.\n\nFiles: task24_jadx_classes2_analysis.md (committed), jadx_classes2_output/ (gitignored)
<!-- SECTION:FINAL_SUMMARY:END -->
