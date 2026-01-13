# Orchestrator Prompt: Create Vulcan-Todo Enhancement Task List

> **Purpose:** Create comprehensive task list for vulcan-todo enhancements in a new OpenCode session
> **Target Session:** Fresh OpenCode session (clean environment)
> **Estimated Time:** 30-45 minutes
> **Target Agent:** @orchestrator (multi-agent coordination)

---

## 📋 Background

### Current State
- **vulcan-todo** MCP server is fully functional and tested
- 8 enhancement tasks have been created in current session
- Current tasks represent the implementation plan (not actual implementation)

### What's Been Done
1. ✅ MCP server protocol fixed (ContentItem enum PascalCase, ToolResult field names)
2. ✅ All 9 MCP tools tested and working (create_task, list_tasks, complete_task, etc.)
3. ✅ Binary built and installed at `~/.local/bin/vulcan-todo`
4. ✅ OpenCode integration verified (server connected in sidebar)

### Current Task List
The current session has 19 tasks:
- **11 existing tasks:** From previous MCP development (test tasks, fixes, etc.)
- **8 new enhancement tasks:** Created just now for future implementation

---

## 🎯 Objective

Create a **comprehensive, well-organized task list** for implementing all vulcan-todo enhancements, organized by priority and logical dependencies.

### Requirements

1. **Session Context:** Start in a **fresh OpenCode session** (clean environment, no pre-existing tasks)
2. **MCP Tool Usage:** Use **vulcan-todo MCP tools** to create all tasks (not CLI)
3. **Task Organization:** Organize by **priority levels** with clear dependencies
4. **Task Granularity:** Break down large features into **specific, actionable tasks** (max 1-2 hours each)
5. **Comprehensive Coverage:** Include all features from the implementation plan:
   - Session/project support
   - TUI project management
   - Enhanced task rows
   - Task detail view
   - Advanced filtering
   - Statistics dashboard
   - Bulk operations
   - Enhanced sorting
6. **Tagging Strategy:** Use tags to group related tasks (e.g., `#phase1`, `#tui`, `#feature`)
7. **Acceptance Criteria:** Each task should have clear definition of "done"

---

## 📊 Task Breakdown by Phase

### Priority Level 1: Critical Foundation (Must Do First)

These are blocking prerequisites for all other work.

#### Phase 1: Project Field Implementation
**Estimated:** 2-3 hours

**Task 1.1:** Add project field to Task model
- **Description:** Add `project: Option<String>` field with default value "VulcanOS"
- **Acceptance:** `project` field exists, Task::new() handles default, backward compatible with existing tasks
- **Dependencies:** None

**Task 1.2:** Add get_by_project() to Store trait
- **Description:** Implement `get_by_project(&self, project: &str) -> Result<Vec<Task>>` method
- **Acceptance:** Method implemented in both JsonStore and MemoryStore, filtering works correctly
- **Dependencies:** Task 1.1

**Task 1.3:** Implement list_projects() in Store trait
- **Description:** Add `list_projects(&self) -> Result<Vec<String>>` method
- **Acceptance:** Returns unique project names from all tasks, handles empty list
- **Dependencies:** Task 1.2

**Task 1.4:** Add --project flag to CLI list command
- **Description:** Add `project: Option<String>` argument to List command struct
- **Acceptance:** Flag exists, filters by project when provided
- **Dependencies:** Task 1.3

**Task 1.5:** Test project filtering via CLI
- **Description:** Run `vulcan-todo list --project VulcanOS` and verify only project tasks shown
- **Acceptance:** Filtering works, non-project tasks excluded when project specified
- **Dependencies:** Task 1.4

---

### Priority Level 2: High (Core TUI Features)

Core TUI functionality needed for productive use.

#### Phase 2: TUI Project Management
**Estimated:** 4-5 hours

**Task 2.1:** Add project state to App struct
- **Description:** Add `project_filter: Option<String>` field
- **Acceptance:** Field exists, initialized to None, can be set/get
- **Dependencies:** Phase 1 complete

**Task 2.2:** Implement project selector overlay
- **Description:** Create overlay showing available projects with 0-9 numbering
- **Acceptance:** Overlay renders correctly, closes on Escape, 0 shows all tasks
- **Dependencies:** Task 2.1

**Task 2.3:** Add P keybinding for project selector
- **Description:** Press `P` toggles `show_project_selector`
- **Acceptance:** `P` key works, overlay opens/closes correctly
- **Dependencies:** Task 2.2

**Task 2.4:** Implement quick-select (0-9) in project selector
- **Description:** Pressing 0-9 selects corresponding project or 0 for all
- **Acceptance:** Quick select works, projects filter applied immediately
- **Dependencies:** Task 2.2

**Task 2.5:** Add project name to task row display
- **Description:** Render project tag in task list (e.g., "[VulcanOS] Task Title")
- **Acceptance:** Project displayed, color-coded, doesn't overlap other fields
- **Dependencies:** Phase 2.2

**Task 2.6:** Update status bar to show current project
- **Description:** Show project indicator in title bar (e.g., "VulcanOS Todo | 5 pending")
- **Acceptance:** Project name visible, updates when changed
- **Dependencies:** Task 2.5

**Task 2.7:** Test project switching in TUI
- **Description:** Navigate between projects, verify tasks filter correctly
- **Acceptance:** Project switching works smoothly, filters update, no crashes
- **Dependencies:** Task 2.6

#### Phase 3: Enhanced Task Row Display
**Estimated:** 2-3 hours

**Task 3.1:** Add due_date display to task row
- **Description:** Show due date in format " | Due: YYYY-MM-DD", color-coded (yellow=today, red=overdue)
- **Acceptance:** Due dates visible, color coding works, no date shown if None
- **Dependencies:** None

**Task 3.2:** Add project tag display in task row
- **Description:** Show "[ProjectName]" before task title when project set
- **Acceptance:** Project tag visible, correctly positioned, optional display
- **Dependencies:** Phase 2 complete

**Task 3.3:** Improve description truncation
- **Description:** Support multi-line descriptions, show "..." when truncated, reserve space for due/project
- **Acceptance:** Descriptions display nicely, don't cause layout issues
- **Dependencies:** None

**Task 3.4:** Show creation/completion dates in detail view
- **Description:** Add fields to detail view showing timestamps
- **Acceptance:** Dates formatted and displayed, not None values
- **Dependencies:** Phase 4

**Task 3.5:** Color-code due dates
- **Description:** Yellow for today, red for overdue, gray for future
- **Acceptance:** Color coding correct, calculated from current date
- **Dependencies:** Task 3.1

**Task 3.6:** Test enhanced task rows
- **Description:** Run TUI with various task configurations, verify layout
- **Acceptance:** All enhancements work, no regressions, layout stable
- **Dependencies:** Task 3.5

---

### Priority Level 3: Medium (Feature Enhancements)

Important but not critical for basic functionality.

#### Phase 4: Task Detail View
**Estimated:** 3-4 hours

**Task 4.1:** Add Detail ViewMode to App
- **Description:** Create Detail variant in ViewMode enum, handle in main render loop
- **Acceptance:** Detail view mode exists, renders when selected
- **Dependencies:** Task 3.6

**Task 4.2:** Implement detail view layout rendering
- **Description:** Create detailed view with all fields (title, status, priority, description, tags, dates, project)
- **Acceptance:** All fields display correctly, proper alignment
- **Dependencies:** Task 4.1

**Task 4.3:** Add inline editing for title
- **Description:** Press `e` enters edit mode for task title, Enter saves
- **Acceptance:** Title editing works in detail view, saves to store
- **Dependencies:** Task 4.2

**Task 4.4:** Add inline editing for description
- **Description:** Press `d` enters multi-line edit mode for description
- **Acceptance:** Description editing works, supports line breaks, saves correctly
- **Dependencies:** Task 4.2

**Task 4.5:** Add inline editing for tags
- **Description:** Press `t` opens tag editor, comma-separated input
- **Acceptance:** Tag editing works, tags parsed correctly, saves to store
- **Dependencies:** Task 4.2

**Task 4.6:** Add priority cycling (1-5 keys)
- **Description:** Press `p` cycles through priority levels with visual feedback
- **Acceptance:** Priority changes, visual indicator shows correct level
- **Dependencies:** Task 4.2

**Task 4.7:** Add due date picker
- **Description:** Press `D` opens date input (YYYY-MM-DD), validation included
- **Acceptance:** Date picker works, validates format, saves correctly
- **Dependencies:** Task 4.2

**Task 4.8:** Add project changer
- **Description:** Press `P` shows project selector, Enter changes project
- **Acceptance:** Project can be changed in detail view, filter applies
- **Dependencies:** Phase 2 complete

**Task 4.9:** Add Done/Undone toggle
- **Description:** Press `x` toggles task completion status
- **Acceptance:** Toggle works, updates in store, reflects in UI
- **Dependencies:** Task 4.2

**Task 4.10:** Add Delete button
- **Description:** Press `d` shows confirmation, Enter deletes
- **Acceptance:** Delete works, task removed from list, confirmation shown
- **Dependencies:** Task 4.2

**Task 4.11:** Test all detail view operations
- **Description:** Test all editing operations, verify data persistence
- **Acceptance:** All operations work, data saves correctly, no data loss
- **Dependencies:** Task 4.10

#### Phase 5: Advanced Filtering
**Estimated:** 4-5 hours

**Task 5.1:** Add tag_filter to App state
- **Description:** Add `tag_filter: Option<String>` to TaskFilter struct
- **Acceptance:** Field exists, initialized to None
- **Dependencies:** Phase 3 complete

**Task 5.2:** Update apply_filter() to handle tag filtering
- **Description:** Filter tasks by tag when tag_filter is set
- **Acceptance:** Tag filtering works, matching tasks shown
- **Dependencies:** Task 5.1

**Task 5.3:** Implement tag cycling (T key)
- **Description:** Press `T` cycles through available tags, current tag shown in status bar
- **Acceptance:** Tag cycling works, filters update as tags change
- **Dependencies:** Task 5.2

**Task 5.4:** Show tag filter in status bar
- **Description:** Display current tag filter (e.g., "Tag: #mcp | Filter: pending")
- **Acceptance:** Tag filter visible, updates on change, shows "All" when None
- **Dependencies:** Task 5.3

**Task 5.5:** Create filter builder modal (F key)
- **Description:** Press `F` opens modal with status, priority, tags, project checkboxes
- **Acceptance:** Filter builder renders, checkboxes/radios work, Apply button exists
- **Dependencies:** Task 5.1

**Task 5.6:** Implement filter application logic
- **Description:** Combine multiple filters (AND/OR logic), apply all at once
- **Acceptance:** Multiple filters work together correctly, can be cleared
- **Dependencies:** Task 5.5

**Task 5.7:** Add active filters display in status bar
- **Description:** Show which filters are active (e.g., "[Work] + [high] + [#mcp]")
- **Acceptance:** Active filters displayed correctly, update when filters change
- **Dependencies:** Task 5.6

**Task 5.8:** Test filter combinations
- **Description:** Test various filter combinations, verify correct results
- **Acceptance:** All combinations work, expected tasks returned, no false positives/negatives
- **Dependencies:** Task 5.7

#### Phase 6: Statistics Dashboard
**Estimated:** 4-5 hours

**Task 6.1:** Add Dashboard ViewMode to App
- **Description:** Create Dashboard variant, handle in main render
- **Acceptance:** Dashboard view mode exists, renders when activated
- **Dependencies:** Phase 5 complete

**Task 6.2:** Implement statistics calculations
- **Description:** Calculate pending/done/archived counts, by priority, by project
- **Acceptance:** All statistics calculated correctly, handle edge cases (empty lists)
- **Dependencies:** Task 6.1

**Task 6.3:** Create pie chart widget
- **Description:** Visual representation of task distribution by status
- **Acceptance:** Pie chart renders correctly, percentages shown, handles 0 tasks
- **Dependencies:** Task 6.2

**Task 6.4:** Create bar chart widget
- **Description:** Visual representation of tasks by priority
- **Acceptance:** Bar chart renders correctly, labels shown, scales with data
- **Dependencies:** Task 6.2

**Task 6.5:** Create activity timeline widget
- **Description:** Show recent task changes (created, completed, updated)
- **Acceptance:** Timeline shows last 10 changes, timestamps formatted correctly
- **Dependencies:** Task 6.2

**Task 6.6:** Create project breakdown widget
- **Description:** Show task count per project
- **Acceptance:** Project counts correct, handle projects with 0 tasks
- **Dependencies:** Task 6.2

**Task 6.7:** Implement dashboard layout with widgets
- **Description:** Layout showing multiple widgets (2x2 grid or similar)
- **Acceptance:** All widgets visible, aligned properly, responsive to terminal size
- **Dependencies:** Tasks 6.3, 6.4, 6.5, 6.6

**Task 6.8:** Add Tab navigation between dashboard sections
- **Description:** Press `Tab` cycles through widgets/sections
- **Acceptance:** Tab navigation works, focus changes visually
- **Dependencies:** Task 6.7

**Task 6.9:** Display dashboard in title bar
- **Description:** Change title bar to "VulcanOS Todo - Dashboard" when in dashboard view
- **Acceptance:** Title updates correctly on view change
- **Dependencies:** Task 6.8

**Task 6.10:** Test dashboard rendering and interaction
- **Description:** Run TUI in dashboard mode, verify all widgets work
- **Acceptance:** Dashboard displays correctly, all widgets functional, navigation works
- **Dependencies:** Task 6.9

---

### Priority Level 4: Low (Nice-to-Have)

Features that improve usability but aren't essential.

#### Phase 7: Bulk Operations
**Estimated:** 2-3 hours

**Task 7.1:** Add archive CLI command
- **Description:** `vulcan-todo archive --all` sets completed tasks to "archived" status
- **Acceptance:** Command works, tasks archived, can be undone if needed
- **Dependencies:** None

**Task 7.2:** Add clean CLI command
- **Description:** `vulcan-todo clean --done` and `--archived` delete respective tasks
- **Acceptance:** Command works, tasks deleted, confirmation prompt shown
- **Dependencies:** Task 7.1

**Task 7.3:** Implement bulk archive in TUI (Ctrl+X menu)
- **Description:** Menu with "Archive", "Delete", "Complete" options
- **Acceptance:** Bulk archive works, can select multiple tasks
- **Dependencies:** Task 4 complete

**Task 7.4:** Implement bulk delete in TUI
- **Description:** Select multiple tasks with Space, delete all at once
- **Acceptance:** Bulk delete works, confirmation required, all selected deleted
- **Dependencies:** Task 7.3

**Task 7.5:** Implement bulk complete in TUI
- **Description:** Select multiple tasks, mark all as complete
- **Acceptance:** Bulk complete works, updates all selected to "done" status
- **Dependencies:** Task 7.3

**Task 7.6:** Implement bulk change priority in TUI
- **Description:** Select tasks, set all to specific priority
- **Acceptance:** Bulk priority change works, can select priority level
- **Dependencies:** Task 7.3

**Task 7.7:** Implement bulk change project in TUI
- **Description:** Select tasks, change project for all selected
- **Acceptance:** Bulk project change works, project dropdown in menu
- **Dependencies:** Phase 2 complete

**Task 7.8:** Add confirmation prompts
- **Description:** Show "Delete X tasks? [y/N]" for destructive bulk operations
- **Acceptance:** Confirmations shown, can be cancelled, only executes on "y"
- **Dependencies:** Task 7.2-7.7

**Task 7.9:** Test all bulk operations
- **Description:** Test each bulk operation, verify multi-select works
- **Acceptance:** All bulk operations work, confirmations work, no data loss
- **Dependencies:** Task 7.8

**Task 7.10:** Update help documentation
- **Description:** Document new bulk operation keybindings and commands
- **Acceptance:** Help text updated with bulk operations
- **Dependencies:** Task 7.9

#### Phase 8: Enhanced Sorting
**Estimated:** 2-3 hours

**Task 8.1:** Add DueDate to SortBy enum
- **Description:** Sort tasks by due date (no due date last)
- **Acceptance:** DueDate sort implemented, correct ordering
- **Dependencies:** None

**Task 8.2:** Add CreatedDate to SortBy (rename Date)
- **Description:** Rename "Date" to "CreatedDate" for clarity, keep functionality
- **Acceptance:** CreatedDate sort works, existing "Date" references updated
- **Dependencies:** None

**Task 8.3:** Add Status to SortBy enum
- **Description:** Sort by status (pending first, then by priority)
- **Acceptance:** Status sort implemented, correct ordering
- **Dependencies:** Task 3.6

**Task 8.4:** Add ModifiedDate to SortBy
- **Description:** Track and sort by last modified timestamp
- **Acceptance:** Modified field added to Task model, sort works
- **Dependencies:** Task 8.2

**Task 8.5:** Add reverse sort field to App state
- **Description:** Add `reverse_sort: bool` to App struct
- **Acceptance:** Field exists, initialized to false
- **Dependencies:** None

**Task 8.6:** Implement reverse sort toggle (R key)
- **Description:** Press `R` flips reverse_sort state
- **Acceptance:** Reverse toggle works, list re-sorted immediately
- **Dependencies:** Task 8.5

**Task 8.7:** Add sort order cycling (O key)
- **Description:** Press `O` cycles through: Priority → Due Date → Alphabetical → Status → Modified Date
- **Acceptance:** Sort order cycling works, current sort shown in status bar
- **Dependencies:** Task 8.1-8.4

**Task 8.8:** Add visual sort indicator in status bar
- **Description:** Show "Sort: Due Date (R for reverse)" or similar
- **Acceptance:** Sort order visible, updates on change
- **Dependencies:** Task 8.7

**Task 8.9:** Implement DueDate sort logic
- **Description:** Tasks without due date last, sorted by date before due date
- **Acceptance:** Due date sort handles None values correctly, ordering as expected
- **Dependencies:** Task 8.1

**Task 8.10:** Implement Status sort logic
- **Description:** Pending before done, then by priority
- **Acceptance:** Status sort works correctly, pending tasks shown first
- **Dependencies:** Task 8.3

**Task 8.11:** Test all sort orders
- **Description:** Test each sort, with and without reverse toggle
- **Acceptance:** All sorts work correctly, reverse toggle functions
- **Dependencies:** Task 8.10

**Task 8.12:** Update help documentation
- **Description:** Document new sort orders and keybindings
- **Acceptance:** Help updated with new sort options and keybindings
- **Dependencies:** Task 8.11

---

## 🎯 Execution Strategy

### Orchestrator Coordination

The orchestrator should coordinate multiple agents working on different phases in parallel where possible:

**Parallelizable Phases:**
- Phase 2 (TUI Project Management) and Phase 3 (Enhanced Task Rows) - both TUI work
- Phase 4 (Detail View) and Phase 5 (Advanced Filtering) - both feature work
- Phase 6 (Dashboard) and Phase 7 (Bulk Operations) - independent features
- Phase 8 (Enhanced Sorting) - independent

**Dependencies Chain:**
- Phase 1 → All phases (foundation)
- Phase 2 → Phase 3 (shared App state)
- Phase 2 → Phase 4,5,6 (project filter in all)
- Phase 4 → Task 5 (detail view needs filtering)
- All phases → Documentation updates

### Agent Recommendations

**Phase 1 (Foundation):**
- Lead Agent: `@frontend-engineer` or `@build`
- Why: Core data model changes, affects all other phases
- Parallel: No (blocking)

**Phases 2-3 (TUI Core):**
- Lead Agent: `@ui-architect` or `@frontend-engineer`
- Why: TUI layout, state management, rendering
- Parallel: Yes (with Phase 3)

**Phases 4-5 (TUI Features):**
- Lead Agent: `@frontend-engineer`
- Why: Additional TUI views, filtering logic
- Parallel: Yes (with each other)

**Phase 6 (Dashboard):**
- Lead Agent: `@style-master` + `@frontend-engineer`
- Why: Complex layout, charts, visual elements
- Parallel: No (blocking)

**Phases 7-8 (Enhancements):**
- Lead Agent: `@build` or `@frontend-engineer`
- Why: Additional CLI commands, TUI features, sorting logic
- Parallel: Yes (with each other)

---

## 📝 Task Template

Each task should follow this format:

```markdown
### Task X.Y: [Brief Title]

**Priority:** Critical/High/Medium/Low
**Estimated:** 1-2 hours
**Phase:** Phase X: [Phase Name]

**Description:** 
Clear, specific description of what needs to be done.

**Acceptance Criteria:**
- [ ] Specific, measurable criteria
- [ ] Another specific, measurable criteria
- [ ] ...

**Dependencies:** 
- Task X.1, Task X.2 (list)

**Implementation Notes:**
Key technical details, file locations, approach hints.

**Testing Notes:**
- [ ] Test case 1
- [ ] Test case 2
```

---

## 🎯 Success Metrics

Track completion across all 8 phases (approximately 60+ subtasks):

- [ ] **Task Creation:** All tasks created via MCP in new session
- [ ] **Task Organization:** Tasks properly tagged, prioritized, dependent
- [ ] **Coverage:** All features from implementation plan represented
- [ ] **No Orphan Tasks:** Every task has a parent task or is phase 1
- [ ] **Priority Adherence:** Critical tasks first, dependencies respected
- [ ] **Quality:** Each task has clear acceptance criteria

**Estimated Total:** 60-65 subtasks  
**Expected Time:** 20-30 hours  
**Agent Strategy:** Parallel execution where possible, proper dependencies

---

## 🚀 Deliverable

**Final Output:**
- A task list in the **new OpenCode session** containing:
  - 60-65 well-defined tasks
  - Organized by phase and priority
  - Clear dependencies
  - Specific acceptance criteria
  - Proper tagging strategy (#phase1, #phase2, #tui, #filter, #dashboard, #bulk, #sort)

**Follow-Up:**
- After task list created, verify all tasks are present
- Check task count matches expectations
- Ensure no duplicate tasks
- Validate dependency chain integrity

---

**Prompt Version:** 1.0  
**Created:** 2026-01-05  
**For:** @orchestrator (to create task list using vulcan-todo MCP tools)
