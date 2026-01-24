# Codebase Concerns

**Analysis Date:** 2026-01-23

## Tech Debt

**Incomplete Sprint Date Parsing:**
- Issue: Sprint start/end date CLI arguments are parsed but not processed
- Files: `vulcan-todo/src/main.rs` (line 510)
- Impact: Users cannot set sprint dates via CLI; dates always default to `None`
- Fix approach: Implement date parsing logic (validate ISO 8601 format, handle timezone)
- Severity: Medium - affects sprint planning features

**Mutex Lock Panics in Store Operations:**
- Issue: Multiple `.unwrap()` calls on `Mutex::lock()` could panic if a thread poisoned the mutex
- Files: `vulcan-todo/src/store/json_store.rs` (lines 80, 103, 139, 147, 164, 545-859)
- Impact: Single corrupted operation could crash entire task service
- Fix approach: Replace `.unwrap()` with proper error handling via `map_err(|_| StoreError::LockPoisoned)` pattern (already exists in vault store)
- Severity: High - affects reliability of task storage

**Test Code Contains Panicking Unwraps:**
- Issue: Test code (lines 994-1053) uses `.unwrap()` on fallible operations
- Files: `vulcan-todo/src/store/json_store.rs` (test section)
- Impact: Tests crash on unexpected failures instead of reporting errors
- Fix approach: Replace test unwraps with proper `Result` assertions or use `?` operator
- Severity: Low (test-only) but indicates broader testing patterns

**Incomplete Error Type Coverage:**
- Issue: SQLite operations in vault store use generic SQLite errors; no custom error variants for specific failure modes
- Files: `vulcan-vault/src/store/sqlite_store.rs` (defines `StoreError` but lacks specificity)
- Impact: Makes error diagnosis harder; callers can't distinguish between lock poisoning, schema issues, and data corruption
- Fix approach: Add variants: `SchemaInitFailed`, `EmbeddingMismatch`, `CorruptedData`, `VectorIndexFailed`
- Severity: Medium - affects debugging and recovery

## Known Bugs

**Dotfiles Copy/Symlink Mismatch:**
- Symptom: dotfiles/ uses GNU Stow for live symlinks, but build.sh deletes and copies directly to archiso
- Files: `scripts/build.sh` (lines 93, 96-123), `CLAUDE.md` warning section
- Trigger: Running `./scripts/build.sh` copies files instead of respecting symlink structure
- Workaround: Always check dotfiles sources before rebuilding; ensure dotfiles/ and archiso skeleton are in sync
- Risk: Archiso skeleton can drift from live dotfiles; user changes may not be reflected in next ISO build

**Swift Exit on Missing Directories During Sync:**
- Symptom: If a dotfiles subdirectory is missing, build script silently skips it with `if [[ -d ... ]]` check
- Files: `scripts/build.sh` (lines 96-123)
- Trigger: User accidentally deletes `dotfiles/hypr/` subdirectory
- Workaround: Build will succeed but the ISO will be missing the configuration
- Risk: Silent configuration loss; ISO builds without reporting missing configs

**Wallpaper Copy Globbing Issues:**
- Symptom: Wallpaper copy uses unquoted glob which fails if no files match
- Files: `scripts/build.sh` (line 128): `cp "$PROJECT_DIR/branding/wallpapers"/*.png ... 2>/dev/null`
- Trigger: No PNG/SVG files in `branding/wallpapers/`
- Workaround: Error is suppressed; script continues but ISO lacks wallpapers
- Risk: Cryptic missing assets; users unaware wallpapers failed to copy

**Potential Deadlock in Json Store Reload:**
- Symptom: `load()` method clears cache and releases locks, but concurrent access could create race condition
- Files: `vulcan-todo/src/store/json_store.rs` (lines 78-113)
- Trigger: Rapid concurrent read/write operations during migration
- Workaround: File locking provides safety but caching invalidation pattern is complex
- Risk: In rare cases with high concurrency, stale data could be returned

## Security Considerations

**Unsafe Memory Transmutation in SQLite Extension Loading:**
- Risk: `std::mem::transmute()` on function pointers for sqlite-vec initialization
- Files: `vulcan-vault/src/store/sqlite_store.rs` (lines 30-32, 46-49)
- Current mitigation: Transmute is marked `unsafe` and only used for known sqlite extension pattern
- Recommendations:
  1. Add comments explaining why this transmute is safe (sqlite3 public API contract)
  2. Consider wrapping in a helper function with safety docs
  3. Audit that sqlite-vec version matches compiled extension

**File Permissions in Backup Script:**
- Risk: Scripts being copied from backup retain whatever permissions they had
- Files: `scripts/backup-vulcan-config.sh` (line 258): `chmod +x` mitigates but only for specific scripts
- Current mitigation: Backup script explicitly `chmod +x` restored scripts
- Recommendations:
  1. Audit `.config/` files for executable bits (config files shouldn't be executable)
  2. Add permission validation post-restore: ensure `~/.config/*` are not executable
  3. Consider using `umask` during restore operations

**Unquoted Variables in Backup Script:**
- Risk: Unquoted `$dir` and `$script` variables in loops could expand to multiple paths if names contain spaces
- Files: `scripts/backup-vulcan-config.sh` (lines 237-273)
- Current mitigation: Bash typical usage avoids spaces in config directory names
- Recommendations:
  1. Quote all variable expansions: `"$dir"` not `$dir`
  2. Use `-z` check for empty string to prevent errors

**Rebuild Command String Injection:**
- Risk: `--path` CLI argument passed to store without validation
- Files: `vulcan-vault/src/main.rs` (line 26), `vulcan-todo/src/main.rs` (lines 36-40)
- Current mitigation: Paths are only used with `Path::from()` which is type-safe
- Recommendations:
  1. Add path validation: reject paths containing `..` or starting with `/`
  2. Document expected path structure in help text

## Performance Bottlenecks

**Full File Re-serialization on Every Store Write:**
- Problem: JSON store reads entire file, deserializes, modifies, re-serializes, and writes full file
- Files: `vulcan-todo/src/store/json_store.rs` (lines 115-143)
- Cause: Simple JSON-based approach without incremental updates
- Current impact: Operations take milliseconds but database grows unbounded
- Improvement path:
  1. For immediate: Add periodic compaction (cleanup completed tasks, archive old sprints)
  2. Long-term: Migrate to SQLite like vault does (vault has better concurrent access patterns)
  3. For vault: Consider compression for large markdown files in chunks

**Mutex Contention on Vault Vector Operations:**
- Problem: All SQLite operations serialize through single mutex lock
- Files: `vulcan-vault/src/store/sqlite_store.rs` (line 62): `lock_conn(&self)`
- Cause: SQLite through `rusqlite` doesn't support concurrent writers
- Current impact: Vector search operations block write operations
- Improvement path:
  1. Profile actual contention (query patterns from MCP tools)
  2. If contention exists, implement read-replica pattern for searches
  3. Consider WAL (write-ahead logging) mode for SQLite

**Recursive Directory Walking for Note Discovery:**
- Problem: Every `rebuild` command scans entire vault directory
- Files: `vulcan-vault/src/main.rs` (lines 231-260)
- Cause: No tracking of modified notes, always reprocesses everything
- Current impact: Rebuilding embeddings takes minutes for large vaults
- Improvement path:
  1. Track note modification times in metadata table
  2. Only reprocess notes modified since last rebuild (unless `--force`)
  3. Use inotify/fsnotify to detect changes in real-time

## Fragile Areas

**Dotfiles Stow Synchronization:**
- Files: `dotfiles/*/` directory structure, `scripts/build.sh`, `archiso/airootfs/etc/skel/.config/`
- Why fragile:
  1. Two copies of configs exist (live stow-managed + archiso skeleton)
  2. No automated sync; manual copying required
  3. Easy to update one without updating the other
  4. CLAUDE.md warns not to delete `.config/` subdirectories, but no enforcement
- Safe modification:
  1. Always edit in `dotfiles/*/` (symlinked to live system)
  2. Before building ISO, run `./scripts/build.sh` to sync
  3. Never directly edit `archiso/airootfs/etc/skel/.config/`
  4. Test live changes before rebuilding
- Test coverage: No automated tests verify sync completeness

**Build Script Dependency Checking:**
- Files: `scripts/build.sh` (lines 48-65)
- Why fragile:
  1. Only checks for binary existence, not version compatibility
  2. archiso version mismatches can cause silent build failures
  3. mkarchiso behavior changed in recent Arch updates
- Safe modification:
  1. Test builds on known-good archiso versions
  2. Add version range check: `mkarchiso --version | grep "^69\|^70"`
- Test coverage: Manual testing only

**Wallpaper Dependency:**
- Files: `scripts/build.sh` (lines 125-130), `branding/wallpapers/`
- Why fragile:
  1. Wallpaper copy silently fails if directory empty
  2. No default fallback wallpapers included
  3. ISO builds successfully without wallpapers (hard to detect)
- Safe modification:
  1. Ensure `branding/wallpapers/` always contains at least one PNG
  2. Add warning if less than 2 wallpapers found
  3. Include fallback solid-color wallpapers
- Test coverage: None

**Hyprland/Hyprlock Config Format Sensitivity:**
- Files: `dotfiles/hypr/.config/hypr/*.conf`, `scripts/prepare.sh` (generates templates)
- Why fragile:
  1. Hyprland parser very strict about syntax
  2. Comments, spacing, and quotes can break parsing
  3. Version incompatibilities between Hyprland and config syntax
- Safe modification:
  1. Test configs with `hyprctl parse` after editing
  2. Keep backup of working config before major edits
  3. Use templating carefully in prepare.sh (lines 197-316)
- Test coverage: Manual testing only

## Scaling Limits

**Task JSON File Growth:**
- Current capacity: 10,000+ tasks reasonable, 100,000+ tasks slow
- Limit: At ~1KB per completed task record, 100,000 tasks = 100MB file
- Full file I/O becomes prohibitive around 50,000 tasks
- Scaling path:
  1. Implement pruning: archive completed tasks older than 90 days
  2. Migrate to SQLite (similar to vault) for better concurrent access
  3. Add pagination to list operations

**Vault Embedding Vector Limits:**
- Current capacity: 10,000 notes with chunks reasonable
- Limit: Each chunk stores 768-dim float vector (~6KB); 10,000 notes * 10 chunks = 600MB vector storage
- sqlite-vec performance degrades with ~100,000+ vectors
- Scaling path:
  1. Implement chunk pruning: drop stale embeddings after 6 months
  2. Archive old projects to separate vault files
  3. Consider quantization (reduce embedding dimensions)

**Concurrent MCP Requests:**
- Current capacity: Single mutex serializes all vault operations
- Limit: More than 5-10 concurrent requests will show latency
- Scaling path:
  1. Profile with load testing (simulate multiple Claude threads)
  2. If contention exists, implement connection pooling
  3. Consider async SQLite library (sqlx) instead of rusqlite

## Dependencies at Risk

**outdated archiso Dependency:**
- Risk: Arch Linux updates mkarchiso frequently with breaking changes
- Impact: Builds may fail silently or produce non-functional ISOs
- Current status: Script checks for `mkarchiso` but not version
- Migration plan:
  1. Pin archiso version in `prepare.sh` if using explicit version
  2. Add changelog check comment for breaking changes
  3. Consider vendoring critical build files

**sqlite-vec Stability:**
- Risk: sqlite-vec is relatively new ecosystem; potential API breaking changes
- Impact: Vault embedding operations could break with updates
- Current status: Using `sqlite-vec = "0.1"`, floating minor versions
- Migration plan:
  1. Pin to specific version: `"0.1.2"` (exact version)
  2. Monitor sqlite-vec GitHub for deprecation notices
  3. Plan migration to newer sqlite vector library if needed

**Deprecated hyprland Configuration Keys:**
- Risk: Hyprland actively evolving; config keys marked deprecated each release
- Impact: Config files could stop working with Hyprland updates
- Current status: No version pinning on Hyprland
- Migration plan:
  1. Document current tested Hyprland version in README
  2. Add migration notes in docs when updating configs
  3. Set specific Hyprland version in pacman.conf if needed

**Broken T2 Hardware Support Chain:**
- Risk: apple-t2-audio-config, tiny-dfr depend on unmaintained kernel modules
- Impact: T2 hardware features could stop working with Linux updates
- Current status: Using arch-mact2 repository (external)
- Migration plan:
  1. Monitor t2linux project for upstream kernel integration
  2. Have fallback configurations (audio via USB, no Touch Bar)
  3. Document hardware graceful degradation path

## Missing Critical Features

**No Task Data Backup Strategy:**
- Problem: Tasks stored in single JSON file; no versioning or recovery
- Blocks: Can't recover if tasks.json corrupted or accidentally deleted
- Current state: Users manually backup `~/.config/vulcan-todo/`
- Solution approach:
  1. Implement automatic daily backups to `~/.local/share/vulcan-todo/backups/`
  2. Keep last 30 days of backups
  3. Add restore command: `vulcan-todo restore <date>`

**No Vault Incremental Indexing:**
- Problem: Every rebuild command reprocesses all notes, no incremental updates
- Blocks: Can't have real-time embedding updates as notes are edited
- Current state: Manual `vulcan-vault rebuild` after note changes
- Solution approach:
  1. Watch vault directory with inotify
  2. Queue modified notes for async embedding processing
  3. Update embeddings in background while user works

**No Sprint Burndown Metrics:**
- Problem: Can list sprint tasks but no velocity, burndown, or completion projection
- Blocks: Can't track sprint health or predict completion
- Current state: Manual script required to calculate burndown
- Solution approach:
  1. Add `sprint stats` command showing velocity and trend
  2. Store task completion timestamps for historical analysis
  3. Implement predictive completion date based on velocity

**No MCP Server Process Management:**
- Problem: No systemd service or supervisor for long-running MCP servers
- Blocks: Servers crash without restart; no logging to journalctl
- Current state: Manual `vulcan-todo --mcp` or custom wrapper scripts
- Solution approach:
  1. Create systemd user service: `vulcan-todo-mcp.service`, `vulcan-vault-mcp.service`
  2. Add socket activation for stdio MCP mode
  3. Wire stderr/stdout to journalctl

## Test Coverage Gaps

**No Integration Tests for Store Persistence:**
- What's not tested: File locking behavior under concurrent access, crash recovery scenarios
- Files: `vulcan-todo/src/store/json_store.rs`, `vulcan-vault/src/store/sqlite_store.rs`
- Risk: Concurrency bugs only surface under real load; silent data loss possible
- Priority: High - affects data integrity
- Suggested tests:
  1. Concurrent reads while write in progress
  2. Crash simulation (kill process mid-write) and verify recovery
  3. Disk full scenarios (no space to write file)
  4. File permission changes (read-only file)

**No Build Script Validation Tests:**
- What's not tested: Package manifest correctness, ISO boot functionality, dotfiles sync
- Files: `scripts/build.sh`, `scripts/prepare.sh`
- Risk: ISO builds can succeed but be non-functional (missing packages, broken configs)
- Priority: High - affects release quality
- Suggested tests:
  1. Dry-run validation of packages.x86_64 (check for invalid package names)
  2. Script lint checking (shellcheck)
  3. QEMU boot test (test-iso.sh exists but may not be comprehensive)

**No MCP Protocol Compliance Tests:**
- What's not tested: Tool input validation, error message format, schema compliance
- Files: `vulcan-todo/src/mcp/tools.rs`, `vulcan-vault/src/mcp/tools.rs`
- Risk: Tools could return malformed responses; Claude integration could break
- Priority: Medium - affects AI agent integration
- Suggested tests:
  1. Validate all tool schemas match spec
  2. Test error responses format correctly
  3. Fuzz test with invalid inputs

**No Configuration Migration Tests:**
- What's not tested: Loading old task/note formats, schema upgrades
- Files: `vulcan-todo/src/models/task.rs`, `vulcan-vault/src/models/note.rs`
- Risk: Schema changes could break existing user data
- Priority: Medium - affects backward compatibility
- Suggested tests:
  1. Load tasks from previous versions
  2. Verify automatic migration works
  3. Test corrupted data handling

---

*Concerns audit: 2026-01-23*
