# Server-Driven Module and Widget Configuration System

Analysis of how the Aula app dynamically configures available features, bottom tab bar items, and embedded widgets based on server-provided profile configuration.

## Overview

Aula uses a server-driven configuration system where the backend controls which modules (first-party features) and widgets (third-party/supplementary features) are available to each user. The configuration is delivered as part of the user profile response and determines:

1. Which tabs appear in the bottom navigation bar
2. Which widgets are embedded on the overview and calendar pages
3. Which widgets get their own full-page tab
4. Which editor plugins are available for rich text editing
5. Which modules are available within group dashboards

The entire system hangs off `Profile.PageConfiguration`, a property on the user's `Profile` object populated from the `/api/v1/?method=profiles.getProfilesByLogin` response.

## Data Model Hierarchy

```
Profile
  └── PageConfiguration
        ├── ModuleConfigurations: List<ModuleConfigurationDto>
        │     └── ModuleDto (Id, Name, Icon, Url, Type, Ordering, CanBePlacedOnGroup)
        ├── WidgetConfigurations: List<WidgetConfigurationDto>
        │     └── WidgetDto (Id, Name, Icon, IconEmployee, IconHover, Url, Type,
        │                     UsableForGroups, Ordering, WidgetId, WidgetVersion,
        │                     CanAccessOnMobile)
        └── EditorPluginDetails: List<EditorPluginDetail>
              └── (Name, MunicipalCode, InstitutionType)
```

### ModuleConfigurationDto

Represents a first-party module feature bound to a user's institution configuration.

| Field                  | Type       | Description |
|------------------------|------------|-------------|
| `Id`                   | `int`      | Unique configuration ID |
| `Module`               | `ModuleDto`| The module definition (type, name, icon, etc.) |
| `Order`                | `int`      | Display ordering position |
| `AggregatedDisplayMode`| `string`   | `"Shown"` or other value to hide. Controls visibility. |

The `Shown` property is derived: `AggregatedDisplayMode == "Shown"`. This means the server can hide a module for a specific institution/user by setting `AggregatedDisplayMode` to anything other than `"Shown"`.

### ModuleDto

| Field             | Type     | Description |
|-------------------|----------|-------------|
| `Id`              | `int`    | Module definition ID |
| `Name`            | `string` | Localized display name |
| `Icon`            | `string` | Icon URL/resource identifier |
| `Url`             | `string` | URL for web-based modules |
| `Type`            | `string` | Machine-readable type key (see catalog below) |
| `Ordering`        | `int`    | Default ordering |
| `CanBePlacedOnGroup` | `bool` | Whether this module can appear on group dashboards |

### WidgetConfigurationDto

| Field                  | Type                    | Description |
|------------------------|-------------------------|-------------|
| `Id`                   | `int`                   | Unique configuration ID |
| `Widget`               | `WidgetDto`             | The widget definition |
| `Placement`            | `WidgetPlacementEnum`   | Bitmask of where to show (own page, overview, calendar) |
| `AggregatedDisplayMode`| `string`                | `"Shown"` or hidden |
| `Order`                | `int`                   | Display ordering |

`ShouldOpenBrowser` is true when the widget has a valid URL and `Widget.Type == "sso"`. SSO widgets open in the external browser rather than an in-app WebView.

### WidgetDto

| Field             | Type     | Description |
|-------------------|----------|-------------|
| `Id`              | `int`    | Widget definition ID |
| `Name`            | `string` | Display name |
| `Icon`            | `string` | Icon URL for guardians |
| `IconEmployee`    | `string` | Icon URL for employees (role-differentiated) |
| `IconHover`       | `string` | Selected/hover state icon |
| `Url`             | `string` | Widget base URL |
| `Type`            | `string` | `"sso"` for SSO widgets, other values for in-app |
| `UsableForGroups` | `bool`   | Whether available on group dashboards |
| `Ordering`        | `int`    | Default ordering |
| `WidgetId`        | `string` | Widget identifier for SSO token requests |
| `WidgetVersion`   | `string` | Version for mobile URL construction |
| `CanAccessOnMobile` | `bool` | Gate for mobile availability |

The `WidgetMobileUrl` is constructed as:
```
{PortalUrl}widgetMobile/W{WidgetId}V{WidgetVersion}?{institutionFilter}&{childFilter}&{group}
```

## Module Type Catalog

The app defines these module type constants in `ModuleType`:

| Constant                    | Type String               | Description |
|-----------------------------|---------------------------|-------------|
| `TYPE_OVERVIEW`             | `"overview"`              | Activity feed / front page |
| `TYPE_CALENDAR`             | `"calendar"`              | Calendar events |
| `TYPE_MESSAGE`              | `"messages"`              | Messaging / threads |
| `TYPE_FILE_DOCUMENT`        | `"documents"`             | Secure documents |
| `TYPE_COME_GO`              | `"presence"`              | Come & go (attendance) |
| `TYPE_GALLERY`              | `"gallery"`               | Photo gallery |
| `TYPE_MORE`                 | `"more-item"`             | Virtual "More" overflow menu (not server-provided) |
| `TYPE_PERSONAL_REFERENCE_DATA` | `"personal_reference_data"` | Personal data / stamdata |
| `TYPE_CONTACT_LIST`         | `"contacts"`              | Contact list |
| `TYPE_ACTIVITY_LIST`        | `"activity_list"`         | Activity list (staff-only) |
| `TYPE_GROUPS`               | `"groups"`                | Groups (hardcoded, always added) |

### Module Lists

Four predefined lists control which modules are recognized by the app:

| List                                | Contents | Purpose |
|-------------------------------------|----------|---------|
| `AllModules`                        | overview, calendar, messages, documents, gallery, presence | Core modules known to the app |
| `ModulesAllowedToDeleteNotifications`| overview, calendar, messages, documents, gallery, presence | Modules whose notifications can be cleared |
| `SupportedModules`                  | overview, calendar, messages, gallery, documents, presence, personal_reference_data, contacts | Modules the app can render (used for filtering) |
| `SupportedModulesIngroup`           | overview, calendar, gallery | Modules available within group dashboards |

Note: `personal_reference_data` and `contacts` are in `SupportedModules` but not in `AllModules`, meaning they are supported for display but not considered "core" modules.

## Widget Placement System

Widget placement uses a flags enum with bitwise operations:

```csharp
[Flags]
enum WidgetPlacementEnum {
    OwnPage        = 1,   // Full-page tab in bottom bar
    RightOfOverview = 2,  // Embedded on overview page (alias: OnOverview = 2)
    RightOfCalendar = 4,  // Embedded on calendar page
    BelowCalendar   = 8,  // Below calendar content
    OnOverview      = 2,  // Alias for RightOfOverview
    OnCalendar      = 0xC // RightOfCalendar | BelowCalendar (4 + 8 = 12)
}
```

The `CheckWidgetOn` extension method performs bitwise AND to test placement:
```csharp
(value & compareValue) > 0
```

This means a single widget can appear in multiple locations. For example, a widget with `Placement = 3` (OwnPage | RightOfOverview) would appear both as its own tab and embedded on the overview page.

### Filtered Widget Collections on PageConfiguration

| Property                              | Filter                           | Description |
|---------------------------------------|----------------------------------|-------------|
| `AllWidgets`                          | `CanAccessOnMobile && Shown`     | All visible mobile widgets |
| `FilteredFullPageWidgetConfigurations`| `Shown && Placement.CheckWidgetOn(OwnPage)` | Widgets that get their own tab |
| `FilteredOverviewWidgetConfigurations`| `Shown && Placement.CheckWidgetOn(RightOfOverview)` | Widgets embedded in overview |
| `FilteredCalendarWidgetConfigurations`| `Shown && Placement.CheckWidgetOn(OnCalendar)` | Widgets embedded in calendar |

## Data Flow: Server Response to Bottom Tab Bar

### 1. Profile Loading

The profile is loaded via `profiles.getProfilesByLogin`. The JSON response includes `pageConfiguration` with `moduleConfigurations` and `widgetConfigurations` arrays. This is deserialized into `Profile.PageConfiguration`.

### 2. Filtering (PageConfiguration)

`OrderedModuleConfigurations` applies two filters:
- Only modules whose `Module.Type` is in `ModuleType.SupportedModules`
- Only modules where `Shown` is true (i.e., `AggregatedDisplayMode == "Shown"`)
- Ordered by `Order` field

`FilteredModuleConfigurations` further adjusts ordering based on the user's local front page setting (see below), placing the user's preferred module first.

`AllWidgets` filters to `CanAccessOnMobile && Shown`, ordered by `Order`.

### 3. Bottom Bar Construction (MainActivity)

`MainActivity.InitBottomBar()` calls `MapModuleAndWidget()` which:

1. Iterates `FilteredModuleConfigurations`, creating a `BottomNavigationItemDTO` for each with `Id = module.Module.Type`
2. Hardcodes a "Groups" item (`Id = "groups"`) -- always present
3. Iterates `AllWidgets` (full-page widgets), creating items with `Id = "Widget_{index}"` and attaching the `WidgetConfigurationDto`

This produces a flat ordered list of all navigable items.

### 4. Bottom Bar Layout (AulaMainPageBottomNavigationView)

The bottom bar can show at most 4 items on phones, 5 on tablets. If there are more items:
- Items up to `MaximumBottomItems - 1` are shown directly
- The last slot becomes a "More" overflow menu item
- Remaining items are accessible via `MoreMenuActivity`

### 5. User Customization (EditShortcuts)

Users can reorder bottom bar items. The custom order is persisted via `SecureStorageHelper` under `EditShortcutsViewModel.ServiceName`. On load, `ReadBottomTabData()` reconciles saved order with current server configuration, handling cases where modules have been added or removed server-side.

### 6. Change Detection

`ProfileManager.CheckIfModulesOrWidgetsChanged()` uses hash code comparison on `Profile.GetHashCode()` (which delegates to `PageConfiguration.GetHashCode()` which hashes both `ModuleConfigurations` and `WidgetConfigurations`). When a change is detected, the bottom bar is rebuilt.

## FrontPage Setting (Local Override)

Users can choose which module appears first when opening the app. This is a **client-side preference** stored in local SQLite (not on the server).

```csharp
enum FrontPageSettingConfigurationEnum {
    ActivityFeed = 1,          // -> "overview" (default)
    Messages = 2,              // -> "messages"
    CalendarOverview = 3,      // -> "calendar"
    ImportantDates = 4,        // -> "calendar"
    Document = 5,              // -> "documents"
    ComeGo = 6,                // -> "presence"
    Gallery = 7,               // -> "gallery"
    ContactList = 8,           // -> "contacts"
    PersonalReferenceData = 9  // -> "personal_reference_data"
}
```

The mapping in `PageConfiguration.GetConfigurationString()` translates this enum to a module type string. `FilteredModuleConfigurations` then sorts modules so the preferred module appears first in the list.

Note: `CalendarOverview` and `ImportantDates` both map to `"calendar"`, meaning the distinction between them is only meaningful for the calendar view's internal state, not for module ordering.

## EditorPluginDetail

`PageConfiguration.EditorPluginDetails` is a list of `EditorPluginDetail` objects with:

| Field              | Type                   | Description |
|--------------------|------------------------|-------------|
| `Name`             | `string`               | Plugin identifier/name |
| `MunicipalCode`    | `string`               | Municipal code this plugin is enabled for |
| `InstitutionType`  | `InstitutionTypeEnum`  | School, Daycare, Municipality, or Central |

This allows the server to control which rich-text editor plugins (e.g., for inserting special content types in messages/posts) are available per municipality and institution type. The `EditorPluginDetails` are delivered alongside module and widget configurations but serve a different purpose -- they configure the message/post editor rather than navigation.

Notable: `EditorPluginDetail` is only referenced in `PageConfiguration` itself. No consumer code was found in the decompiled source that reads or acts on `EditorPluginDetails`, suggesting either: (a) it is consumed by the WebView-based portal editor embedded in the app, or (b) it is a server-side artifact not yet consumed on mobile.

## Group-Level Configuration

Groups have their own module/widget configuration that is independent of the profile-level system:

### GroupModule
| Field            | Type       | Description |
|------------------|------------|-------------|
| `Id`             | `long`     | Configuration ID |
| `ShowOnDashboard`| `bool`     | Whether to display on the group dashboard |
| `Module`         | `ModuleDto`| Reuses the same ModuleDto model |

### GroupWidget
| Field            | Type       | Description |
|------------------|------------|-------------|
| `Id`             | `long`     | Configuration ID |
| `ShowOnDashboard`| `bool`     | Whether to display on the group dashboard |
| `Widget`         | `WidgetDto`| Reuses the same WidgetDto model |

Groups filter their modules to `SupportedModulesIngroup` (overview, calendar, gallery only) and `ShowOnDashboard == true`. Group widgets filter to `CanAccessOnMobile && ShowOnDashboard`.

Key difference from profile-level: Groups use `ShowOnDashboard` (boolean) instead of `AggregatedDisplayMode` (string). This is a simpler visibility model -- the group admin directly toggles module/widget visibility rather than the server computing an aggregated display mode across multiple configuration levels.

## Aggregated Display Mode

The name `AggregatedDisplayMode` strongly implies the server aggregates visibility from multiple configuration levels. The likely hierarchy (based on Aula web portal patterns) is:

1. **Municipality level** -- the municipality enables/disables modules globally
2. **Institution level** -- individual schools can further restrict
3. **User role level** -- different visibility for guardians vs. employees vs. children

The server computes the final `"Shown"` or hidden state and sends it down. The app treats this as an opaque boolean -- it never reasons about why something is hidden, only whether it is.

## Security Implications

1. **Module visibility is server-enforced**: The app filters based on `AggregatedDisplayMode`, but the string values come from the server. A modified client could ignore this filtering, though the backend API endpoints likely perform their own authorization.

2. **Widget type controls rendering path**: The `Type == "sso"` check determines whether a widget opens in an external browser (with SSO token) or in an in-app WebView. This distinction affects the security context -- SSO widgets get a dedicated Aula token via `aulaToken.getAulaToken`, while in-app widgets share the app's session cookies.

3. **CanAccessOnMobile gate**: Widgets have an explicit `CanAccessOnMobile` flag that the app respects. This allows the server to have widgets that are web-portal-only.

4. **Role-differentiated icons**: Widgets serve different icons to employees vs. guardians (`Icon` vs `IconEmployee`), indicating role-awareness in the widget system.

## Source Files

| File | Description |
|------|-------------|
| `AulaNative/AulaNative.Models.ProfileModels/PageConfiguration.cs` | Central configuration holder with filtering/ordering logic |
| `AulaNative/AulaNative.Models.ProfileModels/ModuleConfigurationDto.cs` | Module configuration DTO |
| `AulaNative/AulaNative.Models.ProfileModels/WidgetConfigurationDto.cs` | Widget configuration DTO |
| `AulaNative/AulaNative.Models.ProfileModels/ModuleDto.cs` | Module definition |
| `AulaNative/AulaNative.Models.ProfileModels/WidgetDto.cs` | Widget definition with SSO URL generation |
| `AulaNative/AulaNative.Models.ProfileModels/ModuleType.cs` | Module type constants and support lists |
| `AulaNative/AulaNative.Models.ProfileModels/EditorPluginDetail.cs` | Editor plugin configuration |
| `AulaNative/AulaNative.Enums.Widget/WidgetPlacementEnum.cs` | Widget placement flags |
| `AulaNative/AulaNative.Enums.Widget/WidgetPlacementEnumUtils.cs` | Bitwise placement check |
| `AulaNative/AulaNative.Enums.App/FrontPageSettingConfigurationEnum.cs` | Local front page preference |
| `AulaNative/AulaNative.Models.Modules/GroupModule.cs` | Group-level module config |
| `AulaNative/AulaNative.Models.Widgets/GroupWidget.cs` | Group-level widget config |
| `AulaNative/AulaNative.Models.Widgets/MobileWidgetArguments.cs` | In-app widget query parameters |
| `AulaNative/AulaNative.Models.Widgets/SsoWidgetDirectLinkArguments.cs` | SSO widget query parameters |
| `AulaNative/AulaNative.Services.Singleton/ProfileManager.cs` | Profile singleton with module/widget helpers |
| `AulaNative/AulaNative.ViewModels.AulaMainPage/AulaMainPageViewModel.cs` | Bottom bar data management |
| `AulaNative.Droid/AulaNative.Droid/MainActivity.cs` | Android main activity with bottom bar init |
| `AulaNative.Droid/AulaNative.Droid.Views.MainPage/AulaMainPageBottomNavigationView.cs` | Bottom navigation rendering |
| `AulaNative/AulaNative.SQLiteData.Managers/SQLiteAppSettingManager.cs` | Local settings persistence |
