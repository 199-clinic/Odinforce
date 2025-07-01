# Remove Collab Panel Plan

## Notes
- The user wants to remove the Collab panel carefully to avoid impacting the rest of the codebase, especially keymaps.
- The primary goal is to reduce the number of lines of code.
- **Planning-only phase** – no production code should be changed without prior analysis and checklist updates.
- Backend collaboration code and features will be **fully deleted** (no feature flags or conditional compilation).
- Earlier attempts caused breakages; this roadmap uses fine-grained, validated steps with roll-back points.
- Collab UI panel removal is verified; remaining work is mainly in obsolete test modules and backend crates.
- Removing the collab backend will **not** affect AI/LLM features (they live in separate crates).

## Task List

### Initial Analysis (done)
- [x] Identify all files and code sections related to the Collab panel (`*collab*` search).
- [x] Deep dependency & linkage mapping for UI, keymaps, commands, Vim, settings, docs, tests, scripts.
- [x] Reconcile assistant plan with granular user plan.

### Collab UI Panel Removal Checklist (all done & validated)
- [x] Delete `crates/collab_ui/**` (all files).
- [x] Remove from `[workspace] members` in root `Cargo.toml`.
- [x] Remove dependency in `crates/zed/Cargo.toml`.
- [x] Update `crates/zed/src/main.rs` (remove init, deep link, imports).
- [x] Update `crates/zed/src/zed.rs` (panel/chat loads, actions, imports).
- [x] Delete `crates/title_bar/src/collab.rs` and scrub import.
- [x] Update `crates/zed/src/zed/app_menus.rs` (remove menu item).
- [x] Remove all `collab_panel::*` keybindings from `assets/keymaps/*.json`.
- [x] Remove Vim command in `crates/vim/src/command.rs`.
- [x] Remove settings schemas/defaults for collab/chat/notification panels.
- [x] Remove Storybook selector, example stories, related scripts, Dockerfiles, k8s templates.
- [x] Update docs mentioning collab panel or keybindings.
- [x] Validate after each chunk (`cargo check`, keymap-lint, unit tests, CI).
- [x] Rollback strategy documented.

### UI-side Clean-up
- [x] Removed obsolete collab UI test modules to speed progress.
- [x] Remove/adjust any settings schema, documentation, or Storybook references.
- [ ] Run workspace build/tests periodically (lighter validation after major milestones).

### Backend Removal – Fast Path
- [ ] Perform a comprehensive mapping of all Collab backend dependencies (API, DB, services, tests, docs, scripts).

**Fast-Path Steps**
1. Scrub workspace for any references to backend collab crates (`client`, `rpc`, `proto`, `call`, `livekit_*`, `audio`, `channel`, `remote*`, `headless`, etc.). A quick `cargo tree --invert client` plus a repo-wide `grep -R "::client"`/`"use .*rpc"` is enough.
2. Delete the listed crate directories (`crates/client/`, `crates/rpc/`, `crates/proto/`, `crates/call/`, `crates/livekit_client/`, `crates/livekit_api/`, `crates/audio/`, `crates/channel/`, `crates/remote*/`, `crates/headless/`, …) and remove their entries from `Cargo.toml` + workspace members in one commit.
3. Run `cargo check --all-targets`; fix any compilation errors by deleting orphaned code paths or swapping to local equivalents.
4. Remove infra/deployment files tied to collaboration (Dockerfiles, k8s manifests, CI scripts, migrations, server configs).
5. Final sweep: `grep -R "collab"`, `"server"`, `"auth"` to confirm no remnants.
6. Push. Once green, backend collaboration code is gone.
- [ ] Stage 0: Comprehensive mapping and planning for backend removal—identify all dependencies, integration points, authentication, infra, and impacts (NO feature flags or conditional compilation; all code must be deleted, not hidden)
  - [ ] Map and plan removal for:
    - [ ] Communication crates: client, rpc, proto (auth, websocket, http)
    - [ ] Audio/video/call infra: call, livekit_client, livekit_api, audio
    - [ ] Collaboration features: channel, remote, remote_server, headless
    - [ ] Extension/update systems: extension, auto_update (ensure local-only)
    - [ ] Additional integration points: telemetry, workspace, remote editing, project handling, authentication init, etc.
    - [ ] Test infrastructure: integration tests, fixtures, CI jobs
    - [ ] Build/config: feature flags, env vars, build scripts
    - [ ] Platform-specific code: macOS, Linux, Windows collab/audio
    - [ ] Confirm settings are local JSON, not DB

#### Stage 1 – Refactor client-side deps
- [ ] Remove/rewrite all client-side collab dependencies in `zed` and other crates (no hiding; only deletion/refactor; remove authentication, server fetch, remote editing, channel buffer logic, etc.)
  - [ ] Discovery
    - [ ] Use `cargo tree`, `grep -R`, and IDE tools to identify all collab-related dependencies and usages in `zed` and dependent crates.
    - [ ] List all modules, functions, and UI flows that directly or indirectly use collab/client/rpc/proto/call/channel/remote code.
  - [ ] Authentication/UI removal
    - [ ] Remove or refactor authentication UI, login dialogs, invite flows, and related menu/command entries.
    - [ ] Delete or replace any settings or config fields related to authentication or sharing.
  - [ ] Networking & communication
    - [ ] Strip WebSocket/HTTP client code, including connection setup, event handlers, and message serialization for collab.
    - [ ] Remove background tasks, timers, and async flows that fetch or sync with remote servers for collab features.
  - [ ] Remote/collab feature logic
    - [ ] Remove channel buffer logic, remote editing hooks, and any state machines for collab sessions.
    - [ ] Delete or refactor extension/update systems that depend on collab (ensure local-only operation).
  - [ ] Integration points
    - [ ] Remove all references to collab from extension loading, project handling, telemetry, and workspace logic.
    - [ ] Delete or refactor plugin registration, service locators, and macro invocations related to collab.
  - [ ] Lightweight validation – run build/tests after completing significant chunks.

#### Stage 2 – Remove backend crates (one per commit)
- [ ] Delete `crates/collab/`, `crates/client/`, `crates/rpc/`, `crates/proto/`, `crates/call/`, `crates/livekit_client/`, `crates/livekit_api/`, `crates/audio/`, `crates/channel/`, `crates/remote/`, `crates/remote_server/`, `crates/headless/`, etc. (all backend code, binaries, migrations, tests, docs, scripts, infra; remove all server dependencies, e.g. PostgreSQL, LiveKit)
  - [ ] Discovery
    - [ ] Use `git ls-files` and `grep -R` to list all files in the above crates and related backend code.
    - [ ] Identify integration points in other crates that may break if these are removed; plan refactoring/removal.
  - [ ] Deletion (sub-commits per crate)
    - [ ] Remove each crate directory and its references from `Cargo.toml` and workspace members, one at a time.
    - [ ] Delete related binaries, scripts, test files, and migration files.
    - [ ] Remove server dependencies (e.g., PostgreSQL, LiveKit) from build scripts, CI, and documentation.
  - [ ] Lightweight validation – run build/tests after clusters of crate removals.

#### Stage 3 – Infrastructure and deployment cleanup
- [ ] Identify and remove all infrastructure/deployment files related to collaboration, including:
  - [ ] Dockerfiles (e.g., `Dockerfile-collab*`)
  - [ ] Kubernetes manifests, Helm charts, or k8s templates referencing collab services
  - [ ] Shell scripts, CI/CD scripts, or Makefiles containing collab logic (`script/*collab*`, `ci/*collab*`, etc.)
  - [ ] Database migrations, schema files, or server config files for collab
  - [ ] Any cloud deployment or orchestration configs referencing collab
- [ ] Occasional validation – perform build/tests after batches of infra deletions.

#### Stage 4 – Remove all remaining integration points
- [ ] Systematically search for and remove code that integrates with collab backend, including:
  - [ ] Entrypoints in `main.rs`, `zed.rs`, or any crate root
  - [ ] Window/title bar hooks, menus, and UI panel registration
  - [ ] Keymap, Vim, and command palette entries
  - [ ] Settings and configuration options, including schema and defaults
  - [ ] Authentication, extension loading, and remote editing hooks
  - [ ] Any macro invocations, plugin registration, or service locators for collab
  - [ ] Platform-specific code (macOS, Linux, Windows)
- [ ] Analyze dependencies carefully before deletion;
- [ ] Run build/tests after major integration-point removals.

#### Stage 5 – Replace with local alternatives and ensure feature completeness
- [ ] For each removed collab-dependent feature, determine if a local (non-collab) alternative is needed:
  - [ ] Extension loading: ensure it works locally without collab
  - [ ] LSP and language features: confirm local-only operation
  - [ ] Settings: verify all settings are local and not server-fetched
  - [ ] Remove all authentication and server fetch logic
  - [ ] Ensure no database or server dependency remains
- [ ] Document any replaced or modified flows for future maintainers.
- [ ] Validate by running the app in all supported modes (offline, local workspace, etc.).

#### Stage 6 – Final validation & audit
- [ ] Grep for any remaining references to collab, server, or auth code (e.g., `grep -R collab`, `grep -R server`, `grep -R auth`)
- [ ] Build and test the full workspace with all targets and features
- [ ] Run CI and deployment pipelines to ensure no regressions
- [ ] Confirm that no commented-out, dead, or broken code remains
- [ ] Review code size and dependency graph to ensure maximal lean-ness
- [ ] Only after all checks pass, consider the backend removal complete
- [ ] Keep each logical chunk in its own commit for easy rollback

### Current Goal
Finish collab UI test cleanup before backend mapping

## Current Goal
Finish **collab UI test cleanup** (delete or rewrite obsolete tests) so the whole workspace builds & tests green, then begin backend mapping.
