# Sessions, Projects, Storage & TUI Enhancements Plan

> **Date:** January 5, 2026
> **Project:** vulcan-todo - VulcanOS Task Manager
> **Status:** Ready for Implementation

---

## 📊 Current State Analysis

### Sessions/Projects Differentiation

**Current Implementation:**
- Tasks have a `scope` field (format: `"session:<uuid>"` or `"global"`)
- `OPENCODE_SESSION_ID` environment variable is read for session detection
- Single centralized storage: `~/.config/vulcan-todo/tasks.json`
- Scope filtering via `get_by_scope()` and `get_global()` methods in Store trait

**How It Works:**
1. OpenCode starts with session ID (e.g., "session-abc123")
2. MCP server reads `OPENCODE_SESSION_ID` env var
3. New tasks created via MCP use session scope: `"session:abc123"`
4. Tasks can be filtered by scope: `get_by_scope("session:abc123")`
5. Global tasks (no scope or `"global"`) are visible in all sessions

**Current Limitation:**
- No project concept - only session scopes
- All tasks mixed in one file (regardless of session/project)
- TUI has no way to switch between different projects or scopes

---

## 🎯 Architectural Decisions Needed

### 1. Session vs Project Relationship

**Question:** How should "projects" relate to "sessions"?

**Options:**

| Option | Description | Pros | Cons |
|---------|-------------|-------|-------|
| **A: Separate Concepts** | Projects are manual organization (e.g., "Work", "Personal", "VulcanOS"), sessions are OpenCode instances | Clear mental model | Duplicate concepts |
| **B: Projects = Sessions** | Rename "scope" to "project" for clarity | Simpler terminology | Breaks MCP integration (sessions are distinct from projects) |
| **C: Projects Within Sessions** | Each OpenCode session can have multiple projects | Flexible, matches real workflows | More complex data model |
| **D: Keep Current** | Sessions as OpenCode instances, add optional project field | Compatible with existing MCP | Requires UI to expose both concepts |

**Recommendation:** **Option D** - Keep session scope, add optional project field

**Rationale:**
- Maintains MCP compatibility (sessions are OpenCode-specific)
- Adds flexibility for manual organization without breaking existing flow
- Projects can be used in TUI for filtering, sessions used automatically by MCP
- Clean separation: Sessions = OpenCode instances, Projects = Manual organization

### 2. Storage Architecture

**Current State:**
- Single JSON file: `~/.config/vulcan-todo/tasks.json`
- All tasks (all sessions/projects) in one file
- Simple, portable, works with MCP

**Storage Options Evaluated:**

| Option | Description | Pros | Cons |
|---------|-------------|-------|-------|
| **A: Single File (current)** | One `tasks.json` with scope/project fields | Simple, portable, MCP-compatible | All tasks mixed, harder to backup specific projects |
| **B: Per-Project Files** | `work-tasks.json`, `personal-tasks.json` | Easy git per project, clear backup | Breaks MCP (single store), hard to share across sessions |
| **C: Per-Directory .todo Files** | `.todo` file in each project directory | Git-native, workflow-friendly | Complex to manage with single store, MCP issues |
| **D: Hybrid** | Single file + optional per-project overrides | Flexible, maintainable | Complex implementation |

**Recommendation:** **Option A** - Keep single file storage

**Rationale:**
- **MCP Compatibility:** MCP server expects single store instance
- **No Breaking Changes:** Existing tasks work immediately
- **Simplicity:** Single source of truth, easier to reason about
- **Portability:** Easy to backup, sync across machines
- **Projects as Metadata:** Projects are just tags/filters, not separate storage

---

## 📋 Implementation Plan

### Phase 1: Project Field Implementation
**Estimate:** 2-3 hours

**Objective:** Add optional `project` field to Task model for manual organization

**Tasks:**
1. [ ] Add `project: Option<String>` field to Task struct
2. [ ] Add default `"VulcanOS"` as default project
3. [ ] Update Task::new_with_scope() to accept project parameter
4. [ ] Add `get_by_project()` method to Store trait
5. [ ] Implement in both JsonStore and MemoryStore
6. [ ] Add `list_projects()` method to Store trait
7. [ ] Add `--project <name>` flag to CLI commands (add, edit, list)
8. [ ] Test project filtering via CLI
9. [ ] Verify backward compatibility (tasks without project field work)

**Deliverables:**
- Project field added to Task model
- Project filtering working in CLI
- List of available projects command

---

### Phase 2: TUI Project Management
**Estimate:** 4-5 hours

**Objective:** Add project selector overlay to TUI for switching between projects

**Tasks:**
1. [ ] Add `project_filter: Option<String>` to App state
2. [ ] Add `show_project_selector: bool` to App state
3. [ ] Implement project selector overlay (similar to help overlay)
4. [ ] Display available projects with numbered options (0-9 for quick select)
5. [ ] Add `P` keybinding to toggle project selector
6. [ ] Add quick-select (0-9) in project selector
7. [ ] Add `0` to show all/global tasks (no project filter)
8. [ ] Add project name to task row display
9. [ ] Add project indicator in status bar (e.g., "📂 VulcanOS | 5 pending")
10. [ ] Update title bar to show current project
11. [ ] Test project switching in TUI
12. [ ] Ensure project selection persists across TUI restart

**UI Design:**
```
╭─────────────────────────────╮
│     Select Project:        │
│                          │
│  0. All (Global)        │
│  1. VulcanOS             │
│  2. Personal             │
│  3. Work                │
│                          │
│  Press 0-9 to select      │
│  Esc to cancel            │
╰─────────────────────────────╯
```

**Keybindings:**
- `P` - Show project selector overlay
- `0` - Show all tasks (clear project filter)
- `1-9` - Quick select project (if selector open)
- `Escape` - Close selector

---

### Phase 3: Enhanced Task Row Display
**Estimate:** 2-3 hours

**Objective:** Show due date, project, and improve description display in task list

**Tasks:**
1. [ ] Add `due_date` display in task row (format: " | Due: YYYY-MM-DD")
2. [ ] Add `project` tag display in task row (format: "[ProjectName]")
3. [ ] Improve description truncation with multi-line support
4. [ ] Show creation/completion dates in detail view
5. [ ] Color-code due dates (yellow = today, red = overdue)
6. [ ] Add project indicator to priority display
7. [ ] Truncate long descriptions with "..." indicator
8. [ ] Test with various task lengths and field combinations

**Enhanced Task Row Format:**
```
[✓] [VulcanOS] 🔴 HIGH Task Title | Due: 01-10 | #tag1 #tag2
```

**Before:**
```
[✓] 🔴 HIGH Task Title | #tag1 #tag2
```

---

### Phase 4: Task Detail View
**Estimate:** 3-4 hours

**Objective:** Add full task detail view with inline editing capabilities

**Tasks:**
1. [ ] Add `Detail` to ViewMode enum
2. [ ] Implement detail view rendering (all fields displayed)
3. [ ] Show full description (no truncation)
4. [ ] Show all metadata (created, completed, due dates, tags, project)
5. [ ] Add inline editing for each field:
   - Press `e` to edit title
   - Press `d` to edit description (multi-line)
   - Press `t` to edit tags (comma-separated)
   - Press `p` to cycle priority
   - Press `D` to set due date
   - Press `P` to change project
6. [ ] Add field-specific help overlay
7. [ ] Implement "Save Changes" confirmation
8. [ ] Add "Discard Changes" option
9. [ ] Add "Done/Undone" toggle
10. [ ] Add "Delete" button
11. [ ] Test all field editing operations

**Detail View Layout:**
```
╭───────────────────────────────────────────────────────────╮
│ Task Details                                      │
├───────────────────────────────────────────────────────────┤
│                                                  │
│  Title: Fix MCP protocol issues                    │
│                                                  │
│  Status: Pending                                     │
│  [e] Edit  [d] Desc  [t] Tags  [p] Pri  [D] Due  │
│                                                  │
│  [P] Project  [D] Due Date: 2026-01-05          │
│                                                  │
│  Description:                                      │
│  Change ContentItem enum to PascalCase to comply with  │
│  MCP protocol specification. Update all variants to use  │
│  uppercase names (Text, Image, Resource).            │
│                                                  │
│  Tags: mcp, bug, fix, enhancement, protocol        │
│                                                  │
│  Created: 2026-01-05 21:00 UTC                 │
│  Completed: N/A                                  │
│  Due: N/A                                        │
│                                                  │
│  [x] Done  [d] Delete  [Esc] Back                │
╰───────────────────────────────────────────────────────────╯
```

**Keybindings:**
- `Enter/Space` - Return to list
- `Esc` - Return to list (discard changes)
- `x` - Toggle complete/undone
- `d` - Delete task

---

### Phase 5: Advanced Filtering
**Estimate:** 4-5 hours

**Objective:** Add tag filter and filter builder modal to TUI

**Tasks:**
1. [ ] Add `tag_filter: Option<String>` to App state
2. [ ] Update TaskFilter struct to include tag
3. [ ] Apply tag filter in `apply_filter()` method
4. [ ] Implement tag cycling (press `T` to cycle through available tags)
5. [ ] Show current tag filter in status bar
6. [ ] Create filter builder modal (press `F` to open)
7. [ ] Filter builder UI:
   - Status: Pending, Done, All (checkboxes)
   - Priority: None, Low, Med, High, Urgent (radio buttons)
   - Tags: Tag selector with multi-select
   - Project: Project dropdown
   - Search: Search query input
   - Combinations: AND/OR logic
8. [ ] Apply filters immediately or on "Apply" button
9. [ ] Show active filters in status bar (e.g., "Filter: [Work] + [high]")
10. [ ] Test filter combinations

**Filter Builder Layout:**
```
╭─────────────────────────╮
│  Build Filter        │
├─────────────────────┤
│                      │
│  Status:             │
│   [✓] Pending       │
│   [✓] Done          │
│                      │
│  Priority:            │
│   (○) None          │
│   (●) High          │
│                      │
│  Tags:               │
│   [✓] #mcp         │
│   [✓] #bug          │
│   [ ] #fix          │
│                      │
│  Search:             │
│  [ protocol        ] │
│                      │
│  [Tab] Next [Esc] Cancel [Enter] Apply │
╰─────────────────────────╯
```

**Keybindings:**
- `F` - Open filter builder
- `T` - Cycle through tag filters
- `0` - Clear all filters
- `Escape` - Cancel/close filter builder

---

### Phase 6: Statistics Dashboard
**Estimate:** 4-5 hours

**Objective:** Add dashboard view with task statistics and visualizations

**Tasks:**
1. [ ] Add `Dashboard` to ViewMode enum
2. [ ] Implement dashboard layout with multiple widgets
3. [ ] Calculate statistics:
   - Tasks by status (Pending/Done/Archived)
   - Tasks by priority (None/Low/Med/High/Urgent)
   - Tasks by project
   - Completion rate (last 7/30 days)
   - Tasks due this week
   - Tasks overdue
4. [ ] Create pie chart widget (task distribution by status)
5. [ ] Create bar chart widget (tasks by priority)
6. [ ] Create activity timeline widget (recent task changes)
7. [ ] Create project breakdown widget
8. [ ] Add "Refresh" button to update dashboard
9. [ ] Navigation between dashboard sections (Tab key)
10. [ ] Display dashboard in status bar title

**Dashboard Layout:**
```
╭──────────────────────────────────────────────────────────────────────────╮
│ VulcanOS Todo - Dashboard                                          │
├────────────────────────────────────────────────────────────────────────────┤
│  ┌──────────┐  ┌──────────┐  ┌────────────┐      │
│  │ Status    │  │ Priority  │  │ Projects    │      │
│  ├──────────┤  ├──────────┤  ├────────────┤      │
│  │ Pending: 5│  │ Urgent: 1│  │ VulcanOS: 3 │      │
│  │ Done: 12  │  │ High: 3  │  │ Personal: 2│      │
│  │ Archived: 2│  │ Med: 2   │  │ Work: 1    │      │
│  └──────────┘  └──────────┘  └────────────┘      │
│                                                        │
│  ┌───────────────────────────────────────────────────────────────┐      │
│  │ Tasks Due This Week                                   │      │
│  ├───────────────────────────────────────────────────────────┤      │
│  │ 1. Fix MCP protocol                                │      │
│  │ 2. Add session/project support                        │      │
│  │ 3. Verify MCP server works                             │      │
│  └───────────────────────────────────────────────────────────┘      │
│                                                        │
│  ┌───────────────────────────────────────────────────────────────┐      │
│  │ Recent Activity                                        │      │
│  ├───────────────────────────────────────────────────────────┤      │
│  │ [15:30] Task created: Add session support           │      │
│  │ [16:15] Task completed: MCP protocol test              │      │
│  │ [17:00] Task due tomorrow                              │      │
│  └───────────────────────────────────────────────────────────┘      │
│                                                        │
│  Press [Tab] Next Section | [Esc] Back to List | [D] Refresh │
╰────────────────────────────────────────────────────────────────────────╯
```

**Keybindings:**
- `D` - Open dashboard view
- `Tab` - Cycle between dashboard sections
- `Escape` - Return to list view

---

### Phase 7: Bulk Operations
**Estimate:** 2-3 hours

**Objective:** Add commands for bulk task operations

**Tasks:**
1. [ ] Add `archive` CLI command with `--all` flag
2. [ ] Add `clean` CLI command with `--done` and `--archived` flags
3. [ ] Implement bulk archive in TUI (Ctrl+X menu)
4. [ ] Implement bulk delete in TUI (multi-select with Space)
5. [ ] Implement bulk complete in TUI
6. [ ] Implement bulk change priority
7. [ ] Implement bulk change project
8. [ ] Add confirmation prompts for destructive operations
9. [ ] Test all bulk operations
10. [ ] Update help documentation

**CLI Commands:**
```bash
# Archive all completed tasks
vulcan-todo archive --all

# Delete all done tasks
vulcan-todo clean --done

# Delete all archived tasks
vulcan-todo clean --archived
```

**Bulk Action Menu (TUI):**
```
╭────────────────────────╮
│  Bulk Actions       │
├─────────────────────┤
│  (✓) Complete     │
│  (✓) Delete       │
│  (✓) Archive      │
│  ( ) Priority >    │
│  ( ) Project >     │
│  ( ) Tag >        │
│                     │
│  [Esc] Cancel      │
│  [Enter] Execute    │
╰────────────────────────╯
```

**Keybindings:**
- `Ctrl+X` - Open bulk actions menu
- `Space` - Select/deselect task
- `Enter` - Execute selected bulk action

---

### Phase 8: Enhanced Sorting
**Estimate:** 2-3 hours

**Objective:** Add additional sort orders and reverse toggle

**Tasks:**
1. [ ] Add `DueDate` to SortBy enum
2. [ ] Add `CreatedDate` to SortBy enum (rename Date to CreatedDate)
3. [ ] Add `Status` to SortBy enum
4. [ ] Add `ModifiedDate` to SortBy enum
5. [ ] Add `reverse: bool` field to App state
6. [ ] Implement reverse sort toggle (press `R` to flip order)
7. [ ] Add sort order cycling (press `O` to cycle through options)
8. [ ] Add visual indicator of current sort in status bar
9. [ ] Implement DueDate sort (tasks without due date last)
10. [ ] Implement Status sort (pending before done)
11. [ ] Test all sort orders

**Sort Options:**
- Priority (current)
- Date (created date)
- Alphabetical (current)
- Due Date (new)
- Status (new)
- Modified Date (new)
- + Reverse modifier (all sorts)

**Sort Cycling:**
```
Priority → Due Date → Alphabetical → Status → Modified Date → Priority
```

**Status Bar Indicator:**
```
Sort: Priority (R to cycle, Shift+R for reverse)
```

---

## 📝 CLI Project Commands

### New Commands to Add

```rust
/// List all projects
#[command(name = "projects")]
Projects { },

/// Add task to specific project
AddProject {
    #[arg(long, short = 'p')]
    project: Option<String>,
    
    #[arg(long, short = 't')]
    title: String,
    
    #[arg(long, short = 'd')]
    description: Option<String>,
    
    #[arg(long, short = 'p')]
    priority: Option<PriorityArg>,
    
    #[arg(long, short = 't')]
    tags: Vec<String>,
    
    #[arg(long, short = 'D')]
    due: Option<String>,
}
```

### CLI Usage Examples

```bash
# List all projects
vulcan-todo projects

# List tasks for specific project
vulcan-todo --project Personal list

# Add task to project
vulcan-todo --project Work add "Submit report" --priority high

# Switch default project (for TUI)
vulcan-todo project --set Work

# List all tasks (shows current project filter)
vulcan-todo list --project VulcanOS
```

---

## 🎯 MCP Integration

### Project-Aware MCP Tools

**Current Tools (session-aware):**
- `list_tasks` - Can filter by status, priority, search
- `create_task` - Automatically uses session scope
- `get_task`, `update_task`, etc. - Work with scoped tasks

**Recommended Addition:**
Add `project` parameter to relevant MCP tools:

```rust
"list_tasks": {
    "inputSchema": {
        "properties": {
            // ... existing properties
            "project": {
                "type": "string",
                "description": "Filter by project name"
            }
        }
    }
}
```

**MCP Tool Usage:**
```json
{
    "method": "tools/call",
    "params": {
        "name": "list_tasks",
        "arguments": {
            "project": "VulcanOS",
            "status": "pending",
            "priority": "high"
        }
    }
}
```

---

## 📊 Summary

### Implementation Timeline

| Phase | Tasks | Hours | Dependencies |
|--------|-------|--------|-------------|
| Phase 1: Project Field | 9 | None |
| Phase 2: TUI Project Mgmt | 12 | Phase 1 |
| Phase 3: Enhanced Task Row | 8 | Phase 1 |
| Phase 4: Task Detail View | 11 | Phase 1,3 |
| Phase 5: Advanced Filtering | 10 | Phase 1 |
| Phase 6: Statistics Dashboard | 10 | Phase 1,2,3 |
| Phase 7: Bulk Operations | 8 | Phase 1 |
| Phase 8: Enhanced Sorting | 8 | None |

**Total Estimated Time:** 20-26 hours (8 phases)

---

## ✅ Success Criteria

Each phase is considered complete when:

- [ ] All tasks implemented
- [ ] Tested via CLI
- [ ] Tested via TUI
- [ ] Documentation updated
- [ ] Backward compatibility verified (existing tasks work)
- [ ] No regressions introduced

---

## 🚀 Next Steps

1. **Review Plan** - User approves plan and priorities
2. **Start Implementation** - Begin with Phase 1 (Project Field)
3. **Sprint Execution** - Implement phases in order of priority
4. **Testing & Feedback** - Test after each phase, gather feedback
5. **Documentation Updates** - Update README.md and user guides
6. **Release & Deploy** - Tag version, update ISO when ready

---

## 📚 References

- Existing Implementation: `src/models/task.rs`, `src/store/`, `src/ui/app.rs`
- MCP Protocol: `src/mcp/protocol.rs`, `src/mcp/server.rs`, `src/mcp/tools.rs`
- CLI Interface: `src/cli.rs`
- Previous Documentation: `SESSION_IMPLEMENTATION.md`, `MCP_TEST_RESULTS.md`, `FINAL_STATUS.md`

---

**Document Version:** 1.0  
**Last Updated:** 2026-01-05  
**Author:** @orchestrator
