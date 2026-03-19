---
id: TASK-0076
title: Analyze server-driven module/widget configuration system
status: In Progress
assignee:
  - '@claude'
created_date: '2026-03-18 23:31'
updated_date: '2026-03-19 06:21'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The app bottom tab bar and feature availability are dynamically controlled by Profile.PageConfiguration.ModuleConfigurations and WidgetConfigurations from the server. Understanding this system reveals how Aula controls feature rollout per-institution.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Map complete module type catalog and which lists (AllModules, SupportedModules, SupportedModulesIngroup) they belong to
- [x] #2 Document the ModuleConfigurationDto and WidgetConfigurationDto data models and their server JSON shape
- [x] #3 Explain how modules and widgets flow from server Profile response to bottom tab bar rendering
- [x] #4 Document the AggregatedDisplayMode visibility mechanism and Shown filtering
- [x] #5 Map widget placement system (OwnPage, RightOfOverview, OnCalendar) and how widgets embed in different surfaces
- [x] #6 Document FrontPageSetting local override mechanism
- [x] #7 Document EditorPluginDetail and its role in the configuration system
- [x] #8 Identify how groups use a separate module/widget config (GroupModule, GroupWidget)
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Read all model classes (done): PageConfiguration, ModuleConfigurationDto, WidgetConfigurationDto, ModuleDto, WidgetDto, ModuleType, WidgetPlacementEnum, EditorPluginDetail
2. Trace consumption: MainActivity, AulaMainPageViewModel, AulaMainPageBottomNavigationView, ProfileManager
3. Document Group-level module/widget system (GroupModule, GroupWidget, SupportedModulesIngroup)
4. Document FrontPageSettingConfigurationEnum local override
5. Write analysis document: module_widget_configuration.md
6. Run just e2e, commit
<!-- SECTION:PLAN:END -->
