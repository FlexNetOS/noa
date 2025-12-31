---
id: task-309
title: Improve TUI empty state when task filters return no results
status: To Do
assignee:
  - '@codex'
created_date: '2025-10-27 21:36'
labels: []
dependencies: []
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
## Context
When a user applies search text or filters in the TUI task list that yield no matching tasks, the list pane continues to display the previous set of tasks. Only the details pane shows the "No tasks match your current filters" empty state (see screenshot `codex-clipboard-zzg1oI.png`). This is confusing because it looks like the old tasks are still selectable even though they no longer match.

## Reproduction Steps
1. Open the Backlog.md TUI task list view.
2. Enter a search term that does not match any tasks (e.g. `blabasdasdasdasddas`).
3. Observe that the details pane shows an empty state, but the task list on the left still shows the old results instead of clearing.

## Suggested Approach
- Detect when the current search/filters return an empty result set and clear the list pane selection/history.
- Render an empty-state message in the list pane (instead of showing stale tasks) so the layout matches the details pane messaging.
- Consider extracting existing copy so both panes reuse the same message source if appropriate.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 With a search/filter that yields zero matching tasks, the list pane no longer displays any stale tasks from previous results.
- [ ] #2 When no matches are available, the list pane displays a clear empty-state message (e.g. "No tasks match your current filters") inline where tasks would normally appear.
- [ ] #3 Selecting or moving focus in the list pane while filters return zero results does not re-surface stale tasks until filters actually yield matches.
<!-- AC:END -->
