# Native Library Inventory (.so files)

APK: `com.netcompany.aulanativeprivate` v2.15.4
Architecture: x86_64 (from `config.x86_64.apk` split)
Location: `config.x86_64.apk.extracted/lib/x86_64/`

## Summary

All 12 native libraries are **infrastructure/runtime** -- no business logic lives in native code.
Business logic resides in packed .NET assemblies inside `libassemblies.x86_64.blob.so`.

## Library Inventory

| # | Library | Size | Category | Purpose |
|---|---------|------|----------|---------|
| 1 | `libmonosgen-2.0.so` | 3.2 MB | .NET Runtime | Mono runtime with SGen GC. Core CLR execution engine. Exports `coreclr_*` and `mono_*` APIs. Links to libz for compression. |
| 2 | `libmonodroid.so` | 491 KB | .NET Runtime | Android/.NET bridge (`libmono-android.release.so`). JNI entry point (`JNI_OnLoad`), assembly loading, timezone, exception propagation. Links to libmonosgen + libxamarin-app. Not stripped. |
| 3 | `libmono-component-marshal-ilgen.so` | 37 KB | .NET Runtime | IL code generation for P/Invoke marshaling at runtime. Links to libmonosgen. |
| 4 | `libxamarin-app.so` | 1.5 MB | App Metadata | Xamarin/MAUI app registration data. Contains 680 Java-to-.NET type mappings (CRC64 hashes), assembly image cache, compressed assembly store, JNI remapping tables. No executable code beyond `xamarin_app_init`. |
| 5 | `libassemblies.x86_64.blob.so` | 38 MB | Assembly Store | Packed .NET assemblies in a `payload` section (38.8 MB). Stub ELF wrapping compressed DLLs. Contains all app and framework assemblies. See "Packed Assemblies" section below. |
| 6 | `libarc.bin.so` | 18 KB | App Data | Archive DSO stub (`libarchive-dso-stub.so`). Data payload container for Xamarin/MAUI runtime configuration. Contains `dotnet_for_android_data_payload` + config flags. |
| 7 | `libe_sqlite3.so` | 1.7 MB | Database | SQLite 3 engine (e_sqlite3 build). Used by SQLitePCLRaw .NET wrapper. Full SQLite API exported. |
| 8 | `libSystem.Security.Cryptography.Native.Android.so` | 143 KB | Cryptography | .NET BCL crypto native layer for Android. AES (128/192/256, CBC/ECB/GCM/CCM/CFB), DSA, RSA, ECDSA, ECDH, X509 operations via JNI to Android's Java crypto provider. |
| 9 | `libSystem.Native.so` | 98 KB | System | .NET BCL POSIX native layer. File I/O, sockets, process management, terminal, random bytes (crypto + non-crypto). |
| 10 | `libSystem.IO.Compression.Native.so` | 817 KB | Compression | Brotli encoder/decoder + zlib wrapper. Links to system libz. |
| 11 | `libSystem.Globalization.Native.so` | 68 KB | Globalization | ICU wrapper for .NET globalization. Locale, calendar, collation, case conversion. Dynamically loads ICU at runtime. |
| 12 | `libdatastore_shared_counter.so` | 6.1 KB | AndroidX | AndroidX DataStore shared counter (atomic file-backed counter). JNI exports for `NativeSharedCounter`. |

## Categories

### .NET Runtime (3 libs, ~3.7 MB)
- `libmonosgen-2.0.so` -- CLR execution engine
- `libmonodroid.so` -- Android/JNI bridge
- `libmono-component-marshal-ilgen.so` -- Marshal IL generation

### App Metadata & Assembly Store (3 libs, ~39.5 MB)
- `libassemblies.x86_64.blob.so` -- Packed .NET DLLs (the actual app)
- `libxamarin-app.so` -- Type mappings, assembly cache
- `libarc.bin.so` -- Runtime config data

### .NET BCL Native Support (4 libs, ~1.1 MB)
- `libSystem.Security.Cryptography.Native.Android.so` -- Crypto
- `libSystem.Native.so` -- POSIX/system calls
- `libSystem.IO.Compression.Native.so` -- Brotli + zlib
- `libSystem.Globalization.Native.so` -- ICU globalization

### Third-Party (2 libs, ~1.7 MB)
- `libe_sqlite3.so` -- SQLite database engine
- `libdatastore_shared_counter.so` -- AndroidX DataStore

## Business Logic Assessment

**No business logic lives in native .so files.** All native libraries are:
- .NET runtime infrastructure
- Platform abstraction layers (crypto, I/O, globalization)
- Third-party database engines
- App metadata / assembly containers

**Business logic is in the packed .NET assemblies** inside `libassemblies.x86_64.blob.so`, specifically:
- `AulaNative.dll` -- Core app logic
- `AulaNative.Droid.dll` -- Android platform layer
- `AulaNative.Droid.Private.dll` -- Private/enterprise variant

## Third-Party Native SDKs Identified

From the packed assembly names (not native .so, but .NET bindings):

| SDK | Purpose | Assembly |
|-----|---------|----------|
| Firebase Messaging | Push notifications | `Xamarin.Firebase.Messaging.dll` + related |
| Google Play Services | Cloud messaging, base services | `Xamarin.GooglePlayServices.*.dll` |
| SQLite | Local database | `libe_sqlite3.so` + `SQLitePCLRaw.*.dll` + `SQLite-net.dll` |
| MonkeyCache | Disk caching (uses SQLite) | `MonkeyCache.dll`, `MonkeyCache.SQLite.dll` |
| SixLabors ImageSharp | Image processing | `SixLabors.ImageSharp.dll` |
| IdentityModel OIDC | OAuth2/OpenID Connect auth | `IdentityModel.dll`, `IdentityModel.OidcClient.dll` |
| Plugin.Fingerprint | Biometric authentication | `Plugin.Fingerprint.dll` |
| Plugin.SecureStorage | Secure credential storage | `Plugin.SecureStorage.dll` |
| Newtonsoft.Json | JSON serialization | `Newtonsoft.Json.dll` |
| AutoMapper | Object mapping | `AutoMapper.dll` |
| Unity Container | Dependency injection | `Unity.Abstractions.dll`, `Unity.Container.dll` |
| AndroidX DataStore | Key-value preferences | `Xamarin.AndroidX.DataStore.*.dll` + `libdatastore_shared_counter.so` |
| Glide | Image loading/caching | `Xamarin.Android.Glide.*.dll` |
| Google Tink | Crypto (bundled but unused -- transitive dependency of AndroidX Security) | `Xamarin.Google.Crypto.Tink.Android.dll` |
| I18NPortable | Localization | `I18NPortable.dll` |
| Square OkIO | I/O primitives | `Square.OkIO.dll` |

## Dependency Graph (native .so linkage)

```
libmonodroid.so
  ├── libmonosgen-2.0.so (Mono runtime)
  │     ├── libm.so, libdl.so, liblog.so, libz.so, libc.so
  ├── libxamarin-app.so (type mappings)
  │     └── libc.so
  ├── libm.so, libdl.so, liblog.so, libc.so

libmono-component-marshal-ilgen.so
  └── libmonosgen-2.0.so

libSystem.*.so (BCL native)
  └── libm.so, libdl.so, libc.so, [liblog.so], [libz.so]

libe_sqlite3.so
  └── liblog.so, libc.so, libm.so, libdl.so

libdatastore_shared_counter.so
  └── libm.so, libdl.so, libc.so
```
