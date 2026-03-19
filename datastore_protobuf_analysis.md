# DataStore & Protobuf Serialization Analysis (TASK-78)

## Key Finding: kotlinx.serialization.protobuf Is NOT Present

Despite the task hypothesis, **kotlinx.serialization.protobuf is not bundled in the Aula APK**. The
`kotlinx.serialization` package is only present via `androidx.savedstate.serialization` (standard
AndroidX library code). No kotlinx.serialization.protobuf classes exist in either DEX file or in the
smali disassembly.

The protobuf usage in this app comes from two unrelated sources:

1. **AndroidX DataStore Preferences** -- uses Google's protobuf-lite (repackaged under
   `androidx.datastore.preferences.protobuf`) to serialize key-value preferences
2. **Firebase/Google transport layer** -- bundles `.proto` definitions for analytics transport

## DataStore Usage

### Architecture

The app bundles AndroidX DataStore v1.1.7 (core, preferences, and datastore modules). DataStore is
used exclusively through the **Preferences API** -- not the typed Proto DataStore API. This means:

- Data is stored as generic key-value pairs in `.preferences_pb` files
- The protobuf schema is the standard `PreferencesProto.PreferenceMap` from AndroidX
- There are **no custom protobuf message definitions** for DataStore

### Who Uses DataStore

Only Firebase/Google library code uses DataStore. No Aula application code (neither Java/Kotlin in
the DEX files nor C#/.NET in the Xamarin assemblies) directly references DataStore.

| Consumer | DataStore Name | Purpose |
|----------|---------------|---------|
| `com.google.firebase.datastorage.JavaDataStorage` | `"FirebaseHeartBeat" + suffix` | Firebase heartbeat/SDK analytics tracking |
| `com.google.firebase.heartbeatinfo.HeartBeatInfoStorage` | (via JavaDataStorage) | Stores heartbeat dates, counts, user agent groupings |

### Preference Keys Used by Firebase

From `HeartBeatInfoStorage`:

| Key Name | Type | Purpose |
|----------|------|---------|
| `fire-global` | Long | Timestamp of last global heartbeat |
| `fire-count` | Long | Count of stored heartbeats (limit: 30) |
| `last-used-date` | String | ISO date of last stored heartbeat |
| *(dynamic SDK names)* | Set\<String\> | Dates when each SDK sent a heartbeat |

### PreferencesProto Schema

The protobuf schema used for DataStore Preferences serialization (standard AndroidX, not
Aula-specific):

```protobuf
// Reconstructed from decompiled PreferencesProto.java
// Package: androidx.datastore.preferences

message PreferenceMap {
  map<string, Value> preferences = 1;
}

message Value {
  oneof value {
    bool boolean = 1;
    float float = 2;
    double double = 3;
    int32 integer = 4;
    int64 long = 5;
    string string = 6;
    StringSet string_set = 7;
    bytes bytes = 8;
  }
}

message StringSet {
  repeated string strings = 1;
}
```

This is the standard AndroidX DataStore Preferences protobuf schema. It supports boolean, float,
double, int, long, string, string set, and byte array value types.

## Bundled .proto Files

Three `.proto` files are embedded in the APK. All are standard Google/Firebase definitions, not
Aula-specific:

### 1. `client_analytics.proto`
- **Package**: `firebase.transport`
- **Purpose**: Firebase transport client metrics (cache sizes, dropped events, etc.)
- **Messages**: `ClientMetrics`, `TimeWindow`, `GlobalMetrics`, `StorageMetrics`,
  `LogSourceMetrics`, `LogEventDropped`

### 2. `messaging_event.proto`
- **Package**: `reporting`
- **Purpose**: Firebase Cloud Messaging event tracking
- **Messages**: `MessagingClientEvent` -- tracks message delivery, opens, types, priorities

### 3. `messaging_event_extension.proto`
- **Package**: `reporting`
- **Purpose**: Wrapper for `MessagingClientEvent`
- **Messages**: `MessagingClientEventExtension`

## Conclusion

The original hypothesis was incorrect. The Aula app does **not** use kotlinx.serialization.protobuf
for typed DataStore serialization. The protobuf presence is entirely from:

1. AndroidX DataStore Preferences using protobuf-lite internally for its generic key-value storage
2. Firebase SDK using proto definitions for its analytics transport protocol

The Aula app itself (the .NET/Xamarin layer) does not interact with DataStore directly. All
DataStore usage is confined to Firebase library internals for heartbeat tracking. The app's own
preferences and settings are likely managed through other mechanisms on the .NET side (e.g.,
`Xamarin.Essentials.Preferences` which uses Android `SharedPreferences` under the hood, or the app's
own server-driven configuration).
