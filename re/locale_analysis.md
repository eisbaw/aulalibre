# Locale and Localization Analysis

Analysis of satellite resource assemblies and locale support in Aula Android app v2.15.4.

## Summary

Aula is a **Danish-only** application. All user-facing strings are in Danish, served from a single
embedded JSON locale file (`dk.json`). The 13 satellite resource assemblies found in the blob are
**framework-level** resources from Microsoft.VisualStudio.Validation -- they contain no Aula content
and exist solely because the NuGet package ships with localized validation error messages.

## Satellite Resource Assemblies

### Microsoft.VisualStudio.Validation.resources (13 assemblies)

These are the only satellite resource assemblies in the 363-assembly blob. They are locale variants
of `Microsoft.VisualStudio.Validation.resources.dll`, a Microsoft library used for argument/state
validation (Requires, Verify, Assumes patterns).

| Assembly (blob index) | Locale Code | Language |
|----------------------|-------------|----------|
| .resources.dll (base) | cs | Czech |
| _idx175.dll | de | German |
| _idx176.dll | es | Spanish |
| _idx177.dll | fr | French |
| _idx178.dll | it | Italian |
| _idx179.dll | ja | Japanese |
| _idx180.dll | ko | Korean |
| _idx181.dll | pl | Polish |
| _idx182.dll | pt-BR | Portuguese (Brazil) |
| _idx183.dll | ru | Russian |
| _idx184.dll | tr | Turkish |
| _idx185.dll | zh-Hans | Chinese (Simplified) |
| _idx186.dll | zh-Hant | Chinese (Traditional) |

These assemblies contain localized strings for developer-facing validation messages (e.g.,
"Argument must not be null"). They are **not used for UI localization** and are never visible
to end users. They are included automatically as part of the NuGet dependency.

Notable: Danish (`da`) is NOT among these framework locales -- it was not shipped by Microsoft
for this library.

## Aula Application Localization

### Architecture: I18NPortable with JSON backend

Aula uses the **I18NPortable** library (v21.5 KB) with the **I18NPortable.JsonReader** plugin
for all UI string localization.

**Initialization** (in `AulaNative.LocalesService`):
```csharp
public const string DEFAULT_LOKALE_CODE = "da-DK";

public static void Initialize()
{
    I18N.Current
        .SetNotFoundSymbol("$")
        .SetFallbackLocale("dk")
        .AddLocaleReader(new JsonKvpReader(), ".json")
        .Init(typeof(LocalesService).Assembly);
}
```

Key observations:
- **Fallback locale**: `"dk"` (matches the embedded `AulaNative.Locales.dk.json`)
- **Not-found symbol**: `"$"` -- missing keys render with a `$` prefix in the UI
- **Initialization**: Called from `MainApplication.OnCreate()` in the Android entry point
- **Locale switching**: `SetInternationalization(string conf)` exists but only one locale file is embedded

### The dk.json Locale File

Single embedded resource: `AulaNative.Locales.dk.json`

| Metric | Value |
|--------|-------|
| Total translation keys | 2,189 |
| Unique keys referenced in code | 1,749 |
| Unused/dead keys | ~440 |
| Missing keys (used but undefined) | 7 |
| Language | Danish |
| File size | 2,373 lines |

### Feature Areas by Key Prefix

The translation keys reveal the app's feature set and relative complexity:

| Prefix | Count | Feature Area |
|--------|-------|-------------|
| CALENDAR | 398 | Calendar/scheduling (largest module) |
| MESSAGE | 183 | Messaging |
| ACCESS | 167 | Accessibility labels |
| MOBILE | 166 | Mobile-specific UI |
| COME | 162 | Come/Go (attendance tracking) |
| DOCUMENTS | 104 | Document management |
| GALLERY | 74 | Photo/media gallery |
| SUCCESS | 57 | Success toast messages |
| PRESENCE | 52 | Presence/check-in |
| ACTIVITY | 51 | Activity tracking |
| NOTIFICATION | 46 | Push notifications |
| HELP | 40 | Help guides |
| PROFILE | 32 | User profiles |
| SEARCH | 31 | Search functionality |
| GROUP | 26 | Group management |
| POSTS | 25 | Posts/announcements |
| STAMKORT | 23 | Master data cards |
| CONSENT | 12 | Consent management |
| LOGIN | 11 | Authentication |
| SECURE | 11 | Secure documents/ESDH |
| VACATION | 8 | Vacation registration |

### Missing Translation Keys (7 keys)

These keys are referenced in code via `Extensions.Translate()` but not defined in `dk.json`:

1. `CALENDAR_TOOLBAR_EVENT_TYPES_ENUM_` -- appears to be a dynamic prefix (concatenated with enum values)
2. `MESSAGE_CREATE_MESSAGE_BLOCKED_CHANNEL_WARNING_TEXT_A` -- truncated key name
3. `NEW_POST_SHORT_LABE` -- typo (should be `NEW_POST_SHORT_LABEL`?)
4. `PROFILE_DELETE_PICTURE_WARNING_CHILD_AND_EMPLOYEE`
5. `PROFILE_DELETE_PICTURE_WARNING_EMPLOYEE_DELETE_FOR_CHILD`
6. `PROFILE_DELETE_PICTURE_WARNING_GUARDIAN`
7. `PROFILE_DELETE_PICTURE_WARNING_GUARDIAN_DELETE_FOR_CHILD`

Missing keys render with the `$` prefix in the UI (per `SetNotFoundSymbol("$")`).

### Hardcoded Culture References

Beyond I18NPortable, the code contains hardcoded Danish culture references:

```csharp
// In StringUtils.cs
public static readonly CultureInfo CultureInfo = new CultureInfo("da-DK");
public static readonly StringComparer DanishStringComparer = StringComparer.Create(CultureInfo, true);

// In DateUtils.cs
public static readonly CultureInfo Dk = new CultureInfo("da-DK");

// In ComeGoEmployeeDashboardWeekOverviewDomainService.cs
CultureInfo val = new CultureInfo("da-DK");

// In ThreadSubscriptionInfoViewModel.cs
return new CultureInfo("da-DK").DateTimeFormat.GetDayName(...);
```

The `da-DK` culture is used for:
- String comparison and sorting (Danish alphabetical order: ...X, Y, Z, AE, OE, AA)
- Date formatting (Danish month/day names)
- Number formatting

## Android Resource Layer

### Default strings.xml (values/)

Contains 217 strings, all from **AndroidX/framework libraries** (prefixed `abc_`, `common_`, etc.).
No Aula-specific strings at the Android resource layer.

### Localized values-* directories

Only English variants exist, and only for the `config_en` split APK:
- `values-en-rAU/` (English Australia)
- `values-en-rCA/` (English Canada)
- `values-en-rGB/` (English Great Britain)
- `values-en-rIN/` (English India)
- `values-en-rXC/` (English pseudo-locale for testing)

These contain identical copies of the AndroidX framework strings and are part of the
Google/AndroidX libraries, not Aula.

## Conclusions

1. **Aula supports exactly one language: Danish.** There is a single locale file (`dk.json`)
   with 2,189 translation keys. No other locale files exist.

2. **The 13 satellite resource assemblies are framework artifacts**, not Aula content.
   They belong to `Microsoft.VisualStudio.Validation` and contain developer-facing validation
   messages in 13 languages. These are dead weight in the APK.

3. **The localization infrastructure supports multiple languages** (I18NPortable is designed
   for multi-locale apps), but Aula only ships with Danish. The `SetFallbackLocale("dk")`
   and `SetInternationalization()` method suggest the architecture was designed to potentially
   support other languages, but this has not been implemented.

4. **Danish culture is deeply hardcoded** beyond just UI strings. Date formatting, string
   comparison, and sorting all use `da-DK` CultureInfo directly, meaning adding a second
   language would require refactoring beyond just adding a new JSON locale file.

5. **7 translation keys are missing**, which would display with a `$` prefix. Some appear
   to be typos or dynamic key construction that may work at runtime via string concatenation.
