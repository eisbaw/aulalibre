---
id: TASK-0123
title: 'Make --manual login the default, rename auto to --wip-auto'
status: To Do
assignee: []
created_date: '2026-04-08 07:07'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The localhost redirect flow (auto mode) is rejected by the Aula OIDC provider because http://localhost:{port}/callback is not a registered redirect URI. Only https://app-private.aula.dk is accepted. The --manual flag should become the default login behavior, and the current auto flow should be gated behind --wip-auto to signal it is not yet functional. Update README to reflect the new login usage.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Default login (no flags) uses the manual copy-paste flow
- [ ] #2 --wip-auto flag triggers the localhost callback server flow
- [ ] #3 --manual flag is removed (manual is now the default)
- [ ] #4 README updated to document the new login command usage
- [ ] #5 Help text updated for auth login subcommand
<!-- AC:END -->
