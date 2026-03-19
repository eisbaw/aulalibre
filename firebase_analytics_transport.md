# Firebase Analytics Transport Protocol and Data Collection (TASK-80)

## Executive Summary

The Aula app does **not** use Firebase Analytics (google-analytics-for-firebase) as a product. It does
not log custom analytics events, screen views, or user properties. The only analytics-related code in
the APK comes from two Firebase SDK internals:

1. **Firebase Data Transport SDK** (v4.0.0) -- the generic event batching and delivery infrastructure
2. **FCM MessagingAnalytics** -- automatic notification delivery/open metrics built into the FCM SDK

There is zero analytics instrumentation in the Aula application code itself (neither in the
Java/Kotlin DEX layer nor in the .NET/Xamarin C# assemblies).

---

## 1. Firebase Data Transport SDK

### What It Is

The Firebase Data Transport SDK (`com.google.android.datatransport`) is a generic event batching and
delivery pipeline. It is **not** Firebase Analytics -- it is infrastructure that other Firebase SDKs
use to send telemetry back to Google. It operates entirely within SDK library code and requires no
app-level integration.

### Transport Endpoints

The transport backend uses two endpoints, which are obfuscated via `StringMerger.mergeStrings()`:

| Endpoint | URL | Purpose |
|----------|-----|---------|
| **DEFAULT** | `https://firebaselogging.googleapis.com/v0cc/log/batch?format=json_proto3` | Primary batch log endpoint |
| **LEGACY** | `https://firebaselogging-pa.googleapis.com/v1/firelog/legacy/batchlog` | Legacy batch log endpoint |

The legacy endpoint includes an embedded API key: `AIzaSyCckkiH8i2ZARwOs1LEzFKld15aOG8ozKo`
(standard Google API key for Firebase transport, not Aula-specific).

### Request Protocol

- **Method**: POST
- **Content-Type**: `application/json`
- **Content-Encoding**: gzip (both request and response)
- **User-Agent**: `datatransport/4.0.0 android/`
- **Authentication**: Optional `X-Goog-Api-Key` header; optional `Cookie: NID=<pseudonymousId>`
- **Retry**: Up to 5 retries with redirect following (301/302/307)
- **Timeout**: 30s connect, 130s read

### Request Body Structure

Requests are `BatchedLogRequest` containing one or more `LogRequest` objects, each grouped by
`transportName` (the log source identifier).

```
BatchedLogRequest
  └── LogRequest[]
       ├── source (string or int) -- log source identifier
       ├── requestTimeMs, requestUptimeMs
       ├── qosTier: DEFAULT
       ├── clientInfo
       │    ├── clientType: ANDROID_FIREBASE
       │    └── androidClientInfo  (see "Device Data Collected" below)
       └── logEvents[]
            ├── eventTimeMs, eventUptimeMs
            ├── timezoneOffsetSeconds
            ├── networkConnectionInfo { networkType, mobileSubtype }
            ├── payload (proto or JSON encoded bytes)
            ├── eventCode (optional)
            ├── complianceData (optional, privacy context)
            └── experimentIds (optional, clear + encrypted blobs)
```

### Log Sources Identified

Two named log sources are used:

| Log Source | Producer | Payload |
|------------|----------|---------|
| `FCM_CLIENT_EVENT_LOGGING` | `MessagingAnalytics` | `MessagingClientEventExtension` protobuf |
| `GDT_CLIENT_METRICS` | `Uploader` (transport SDK itself) | `ClientMetrics` protobuf |

### Device Data Collected (AndroidClientInfo)

Every transport request includes this device metadata, populated from `Build.*` and system services:

| Field | Source | Example |
|-------|--------|---------|
| `sdkVersion` | `Build.VERSION.SDK_INT` | 33 |
| `model` | `Build.MODEL` | "Pixel 7" |
| `hardware` | `Build.HARDWARE` | "tensor" |
| `device` | `Build.DEVICE` | "panther" |
| `product` | `Build.PRODUCT` | "panther" |
| `osBuild` | `Build.ID` | "TQ3A.230901.001" |
| `manufacturer` | `Build.MANUFACTURER` | "Google" |
| `fingerprint` | `Build.FINGERPRINT` | Full build fingerprint string |
| `country` | `Locale.getDefault().getCountry()` | "DK" |
| `locale` | `Locale.getDefault().getLanguage()` | "da" |
| `mccMnc` | `TelephonyManager.getSimOperator()` | "23801" (TDC DK) |
| `applicationBuild` | Package version code | "215040000" |

Additionally, each event includes:
- `timezoneOffsetSeconds` -- UTC offset
- `networkType` -- WiFi, mobile, none
- `mobileSubtype` -- LTE, EDGE, HSDPA, etc.

**Privacy note**: No IMEI, IMSI, phone number, Android ID, advertising ID, or user-identifiable data
is collected. The `fingerprint` field is the Android build fingerprint (build configuration string),
not a device fingerprint for tracking.

---

## 2. FCM Messaging Analytics (Notification Delivery Metrics)

### How It Works

`MessagingAnalytics` is part of the Firebase Cloud Messaging SDK. It automatically tracks
notification lifecycle events without any app-level code. The tracking is triggered by callbacks
in `FirebaseMessagingService` and `FcmLifecycleCallbacks`.

### Events Tracked

| Event | Trigger | Scion Event | Firelog Event |
|-------|---------|-------------|---------------|
| Notification received | `onMessageReceived()` | `_nr` | `MESSAGE_DELIVERED` |
| Notification opened | User taps notification | `_no` | `MESSAGE_OPEN` |
| Notification foreground | Message arrives while app in foreground | `_nf` | -- |
| Notification dismissed | User swipes away notification | `_nd` | -- |
| Campaign attribution | FCM message with analytics keys | `_cmp` | -- |

### Two Analytics Paths

FCM sends analytics through two independent channels:

#### Path 1: Scion (Google Analytics for Firebase connector)
- Sends events to `AnalyticsConnector.logEvent()` with origin `"fcm"`
- **However**: The Aula app only includes `firebase-measurement-connector` (the interface) -- the
  full Firebase Analytics SDK is **not present** in the APK
- The `AnalyticsConnector` interface has no implementation to receive these events
- Result: **Scion events are silently dropped** (the code logs "Unable to log event: analytics
  library is missing" at WARN level)

#### Path 2: Firelog (Data Transport)
- Sends `MessagingClientEventExtension` protobuf via the Data Transport SDK
- Log source: `FCM_CLIENT_EVENT_LOGGING`
- Only fires for `MESSAGE_DELIVERED` and `MESSAGE_OPEN` events
- Controlled by `shouldUploadFirelogAnalytics()` which checks for `google.c.a.e` key in the
  FCM payload (must be "1" to enable)
- Optional BigQuery export via `delivery_metrics_exported_to_big_query_enabled` manifest flag
  (not set in Aula's manifest)

### MessagingClientEvent Protobuf Schema

When a firelog analytics event fires, the protobuf payload contains:

```protobuf
message MessagingClientEvent {
  int64 project_number = 1;     // Firebase project number (811573413698)
  string message_id = 2;        // FCM message persistent ID
  string instance_id = 3;       // Firebase Installation ID (FID)
  MessageType message_type = 4; // DATA_MESSAGE, TOPIC, DISPLAY_NOTIFICATION
  SDKPlatform sdk_platform = 5; // ANDROID
  string package_name = 6;      // "com.netcompany.aulanativeprivate"
  string collapse_key = 7;      // FCM collapse key (if set)
  int32 priority = 8;           // 5=normal, 10=high
  int32 ttl = 9;                // Time-to-live
  string topic = 10;            // Topic (if sent to topic)
  int64 bulk_id = 11;           // Server-side batch ID
  Event event = 12;             // MESSAGE_DELIVERED or MESSAGE_OPEN
  string analytics_label = 13;  // Developer analytics label (if set)
  int64 campaign_id = 14;       // Firebase campaign ID (if campaign)
  string composer_label = 15;   // Firebase campaign name (if campaign)
}
```

**Privacy note**: This contains the Firebase Installation ID (a per-app-install pseudonymous
identifier, not tied to a user account) and the Firebase project number (a fixed project-level
identifier). No user content, names, message text, or PII is included.

### Scion Analytics Parameters (logged but silently dropped)

When the Scion path fires (which it cannot, since the analytics SDK is absent), the parameters
would be:

| Parameter | Key | Source |
|-----------|-----|--------|
| Composer ID | `_nmid` | `google.c.a.c_id` |
| Composer Label | `_nmn` | `google.c.a.c_l` |
| Message Label | `label` | `google.c.a.m_l` |
| Message Channel | `message_channel` | `google.c.a.m_c` |
| Topic | `_nt` | `from` key (topic prefix) |
| Message Time | `_nmt` | `google.c.a.ts` |
| Device Time | `_ndt` | `google.c.a.udt` |
| Message Type | `_nmc` | "data" or "display" |
| Source | `source` | "Firebase" |
| Medium | `medium` | "notification" |
| Campaign | `campaign` | Composer label |

---

## 3. GDT Client Metrics (Self-Monitoring)

### What It Is

The transport SDK sends its own health metrics as a separate log source (`GDT_CLIENT_METRICS`).
This is "meta-analytics": the transport layer reporting on its own performance.

### When It Fires

On every batch upload, the `Uploader` attaches a `ClientMetrics` payload reporting:

### ClientMetrics Protobuf Schema

```protobuf
message ClientMetrics {
  TimeWindow window = 1;                        // Reporting period
  repeated LogSourceMetrics log_source_metrics = 2;  // Per-source drop stats
  GlobalMetrics global_metrics = 3;             // Cache usage
  string app_namespace = 4;                     // Package name
}

message GlobalMetrics {
  StorageMetrics storage_metrics = 1;
}

message StorageMetrics {
  int64 current_cache_size_bytes = 1;  // Current SQLite DB size
  int64 max_cache_size_bytes = 2;      // Max allowed (10 MB)
}

message LogSourceMetrics {
  string log_source = 1;                        // e.g., "FCM_CLIENT_EVENT_LOGGING"
  repeated LogEventDropped log_event_dropped = 2;
}

message LogEventDropped {
  int64 events_dropped_count = 1;
  Reason reason = 3;  // MESSAGE_TOO_OLD, CACHE_FULL, PAYLOAD_TOO_BIG, etc.
}
```

### Drop Reasons

Events can be dropped for these reasons:

| Reason | Value | Description |
|--------|-------|-------------|
| `REASON_UNKNOWN` | 0 | Unknown |
| `MESSAGE_TOO_OLD` | 1 | Event exceeded 7-day cleanup age |
| `CACHE_FULL` | 2 | SQLite DB exceeded 10 MB limit |
| `PAYLOAD_TOO_BIG` | 3 | Single event exceeded 80 KB per row |
| `MAX_RETRIES_REACHED` | 4 | Upload failed after max retries |
| `INVALID_PAYLOD` | 5 | Encoding/serialization error |
| `SERVER_ERROR` | 6 | Server returned 5xx |

---

## 4. Event Cache (SQLite Persistence)

### Database Details

| Property | Value |
|----------|-------|
| **DB Name** | `com.google.android.datatransport.events` |
| **Schema Version** | 7 |
| **Max DB Size** | 10 MB (`MAX_DB_STORAGE_SIZE_IN_BYTES`) |
| **Max Blob per Row** | 80 KB (`MAX_BLOB_BYTE_SIZE_PER_ROW`) |
| **Event Cleanup Age** | 7 days (`DURATION_ONE_WEEK_MS = 604800000`) |
| **Upload Batch Size** | 200 events |
| **Lock Timeout** | 10 seconds |
| **Max Upload Retries** | 16 (with 50ms backoff) |

### Table Schema

```sql
-- Registered transport backends (e.g., "cct" for Clearcut/Firebase)
CREATE TABLE transport_contexts (
  _id INTEGER PRIMARY KEY,
  backend_name TEXT NOT NULL,
  priority INTEGER NOT NULL,
  next_request_ms INTEGER NOT NULL,
  extras BLOB                        -- CCTDestination serialized bytes
);

-- Queued events waiting for upload
CREATE TABLE events (
  _id INTEGER PRIMARY KEY,
  context_id INTEGER NOT NULL,       -- FK to transport_contexts
  transport_name TEXT NOT NULL,       -- Log source (e.g., "FCM_CLIENT_EVENT_LOGGING")
  timestamp_ms INTEGER NOT NULL,
  uptime_ms INTEGER NOT NULL,
  payload BLOB NOT NULL,             -- Protobuf or JSON encoded event
  code INTEGER,
  num_attempts INTEGER NOT NULL,
  payload_encoding TEXT,             -- "proto" or "json"
  inline BOOLEAN NOT NULL DEFAULT 1,
  product_id INTEGER,
  pseudonymous_id TEXT,
  experiment_ids_clear_blob BLOB,
  experiment_ids_encrypted_blob BLOB,
  FOREIGN KEY (context_id) REFERENCES transport_contexts(_id) ON DELETE CASCADE
);

-- Key-value metadata per event
CREATE TABLE event_metadata (
  _id INTEGER PRIMARY KEY,
  event_id INTEGER NOT NULL,
  name TEXT NOT NULL,
  value TEXT NOT NULL,
  FOREIGN KEY (event_id) REFERENCES events(_id) ON DELETE CASCADE
);

-- Large payload chunks (for events exceeding inline limit)
CREATE TABLE event_payloads (
  sequence_num INTEGER NOT NULL,
  event_id INTEGER NOT NULL,
  bytes BLOB NOT NULL,
  FOREIGN KEY (event_id) REFERENCES events(_id) ON DELETE CASCADE,
  PRIMARY KEY (sequence_num, event_id)
);

-- Drop statistics per log source
CREATE TABLE log_event_dropped (
  log_source VARCHAR(45) NOT NULL,
  reason INTEGER NOT NULL,
  events_dropped_count BIGINT NOT NULL,
  PRIMARY KEY (log_source, reason)
);

-- Last metrics upload timestamp
CREATE TABLE global_log_event_state (
  last_metrics_upload_ms BIGINT PRIMARY KEY
);
```

### Upload Scheduling

Events are uploaded via Android `JobScheduler` (API 21+) or `AlarmManager` (fallback). The scheduler:

1. Cleans up events older than 7 days
2. Checks network connectivity
3. Loads events in batches of 200
4. Decorates each event with device metadata (AndroidClientInfo)
5. Sends to `CctTransportBackend` via HTTP POST
6. On success: removes events from DB, resets client metrics
7. On failure: records failure count, schedules retry with backoff
8. If more events pending, reschedules immediately

---

## 5. .NET/Xamarin Side: No Analytics

The decompiled .NET assemblies (`AulaNative` and `AulaNative.Droid`) contain:

- **No AppCenter SDK** -- no `Microsoft.AppCenter.Analytics` references
- **No Firebase Analytics SDK** -- no `Firebase.Analytics` assembly
- **No custom analytics/tracking code** -- no `logEvent`, `TrackEvent`, `SendAnalytics`, or similar
- **No telemetry instrumentation** of any kind

The only Firebase-related assembly present is `Xamarin.Firebase.Messaging` (for push notifications)
and `Xamarin.Firebase.Measurement.Connector` (the interface-only connector, not the full analytics
SDK).

---

## 6. Key Findings

1. **Aula does NOT implement custom analytics**: No app-level event tracking, no user behavior
   analytics, no screen view tracking. The developers chose not to instrument the app with Firebase
   Analytics, AppCenter, or any other analytics SDK.

2. **Only Firebase SDK internal telemetry exists**: The FCM SDK automatically reports notification
   delivery/open metrics back to Google. This is standard Firebase SDK behavior that ships with any
   app using FCM, not a deliberate choice by Netcompany.

3. **Scion analytics path is dead**: The Firebase Analytics connector interface is present (required
   dependency of FCM), but without the full Firebase Analytics SDK, all Scion events are silently
   discarded with a warning log.

4. **Device metadata is collected but minimal**: The transport layer sends device model, OS version,
   locale, country, and carrier info. No advertising IDs, no user identifiers, no IMEI/IMSI.

5. **Transport endpoint is Google's standard Clearcut/Firelog**: Events go to
   `firebaselogging.googleapis.com`, Google's standard telemetry collection service. This is the
   same endpoint used by all Firebase-enabled Android apps.

6. **Event cache is bounded and ephemeral**: Max 10 MB SQLite DB, 7-day cleanup, 80 KB per event,
   200-event upload batches. Events are deleted after successful upload.

7. **No BigQuery export configured**: The `delivery_metrics_exported_to_big_query_enabled` manifest
   flag is not set, so FCM delivery metrics are not exported to BigQuery.

8. **The Firebase Installation ID (FID) is the only pseudonymous identifier**: This is a per-app-
   install ID managed by Firebase, used for FCM token registration and messaging analytics. It is
   not tied to a user's Google account or any PII.
