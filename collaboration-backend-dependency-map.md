# Collaboration Backend Dependency Map - Stage 0

## Overview
This document provides a comprehensive mapping of all collaboration backend dependencies in the Odinforce codebase. The analysis identifies communication crates, audio/video infrastructure, collaboration features, and their integration points throughout the codebase.

## 1. Communication Crates

### client (4,160 LOC)
**Purpose**: Core client for communication with the collaboration server
**Dependencies**:
- rpc (with gpui feature)
- http_client, http_client_tls
- credentials_provider
- telemetry_events, telemetry
- async-tungstenite (WebSocket)
- Platform-specific: tokio-native-tls (Windows/macOS), tokio-rustls (Linux)

**Used by** (33 crates):
- agent, assistant_context_editor, auto_update, auto_update_ui
- call, channel, command_palette, copilot
- dap, debugger_ui, editor, eval
- extension_host, extensions_ui, feedback
- inline_completion, inline_completion_button, install_cli
- language_model, language_models, lsp, project
- recent_projects, semantic_index, supermaven, workspace, zed
- And others

**Critical Integration Points**:
- Authentication and session management
- WebSocket connections for real-time features
- HTTP client for API calls
- Telemetry and error reporting

### rpc (2,099 LOC)
**Purpose**: RPC protocol implementation for client-server communication
**Dependencies**:
- proto
- async-tungstenite
- Optional: gpui (for UI integration)

**Used by** (11 crates):
- assistant_context_editor, channel, collab
- debugger_ui, editor, extension_cli
- language, notifications, project
- remote_server, title_bar

**Critical Integration Points**:
- Message serialization/deserialization
- Connection management
- Protocol definitions

### proto (1,363 LOC)
**Purpose**: Protocol buffer definitions for communication
**Dependencies**:
- prost (protobuf runtime)
- Build dependency: prost-build

**Used by**:
- rpc (primary consumer)
- remote_server
- collab (server-side)

**Critical Integration Points**:
- Defines all message types for client-server communication
- Build script generates Rust code from .proto files

## 2. Audio/Video/Call Infrastructure

### call (2,329 LOC)
**Purpose**: Voice/video calling functionality
**Dependencies**:
- client
- audio
- livekit_client
- project

**Used by** (4 crates):
- git_ui
- title_bar
- workspace
- zed

**Critical Integration Points**:
- Room management
- Participant state
- Screen sharing integration

### livekit_client (2,679 LOC)
**Purpose**: LiveKit WebRTC client integration
**Dependencies**:
- livekit_api
- libwebrtc, livekit (external dependencies)
- Platform-specific: scap (Linux), coreaudio-rs (macOS)

**Used by**:
- call

**Critical Integration Points**:
- WebRTC audio/video streams
- Track management
- Connection state

### livekit_api (293 LOC)
**Purpose**: LiveKit server API client
**Dependencies**:
- reqwest (HTTP client)
- jsonwebtoken (for authentication)
- protobuf for LiveKit protocol

**Used by**:
- livekit_client

### audio (144 LOC)
**Purpose**: Audio playback functionality
**Dependencies**:
- rodio (audio playback library)
- gpui

**Used by**:
- call
- zed

## 3. Collaboration Features

### channel (3,022 LOC)
**Purpose**: Channel-based collaboration (chat, shared buffers)
**Dependencies**:
- client
- rpc
- language

**Used by** (3 crates):
- collab
- notifications
- zed

**Critical Integration Points**:
- Channel buffer synchronization
- Message history
- Presence information

### remote (2,708 LOC)
**Purpose**: Client-side remote development support
**Dependencies**:
- rpc (with gpui feature)
- askpass (SSH authentication)

**Used by** (6 crates):
- extension_host
- project
- recent_projects
- title_bar
- workspace
- zed

**Critical Integration Points**:
- SSH connection management
- Remote file system operations
- Remote process execution

### remote_server (3,602 LOC)
**Purpose**: Server daemon for remote development
**Dependencies**:
- client
- remote
- rpc
- proto
- extension, extension_host
- Multiple language/LSP dependencies

**Used by**: None (standalone binary)

**Critical Integration Points**:
- Headless operation
- LSP proxy
- File system operations
- Extension system integration

## 4. Extension/Update Systems

### extension (1,957 LOC)
**Purpose**: Core extension system functionality
**Dependencies**:
- http_client
- WASM toolchain (wasm-encoder, wasmparser)

**Used by**:
- extension_host
- extensions_ui
- language_extension
- remote_server
- zed

### extension_host (7,898 LOC)
**Purpose**: Host environment for running extensions
**Dependencies**:
- client
- extension
- remote
- WASM runtime (wasmtime)

**Used by**:
- extensions_ui
- language
- remote_server
- zed

### auto_update (1,189 LOC)
**Purpose**: Application auto-update functionality
**Dependencies**:
- client
- http_client
- release_channel

**Used by**:
- auto_update_ui
- zed

## 5. Major Integration Points

### workspace (depends on):
- call
- client
- remote

### project (depends on):
- client
- remote
- rpc

### zed (main app, depends on):
- audio
- auto_update, auto_update_ui
- call
- channel
- client
- extension, extension_host, extensions_ui
- remote

## 6. Test Infrastructure

### collab (47,602 LOC - test server)
**Purpose**: Test collaboration server
**Dependencies**:
- channel
- rpc
- All server-side infrastructure

**Test files using collaboration**:
- channel_guest_tests.rs
- random_channel_buffer_tests.rs
- channel_tests.rs
- Various integration tests

## 7. Build/Config Dependencies

### Proto compilation:
- proto/build.rs: Compiles zed.proto
- livekit_api/build.rs: Compiles LiveKit protocol

### Platform-specific code:
- **Windows**: tokio-native-tls, windows crate
- **macOS**: tokio-native-tls, cocoa, coreaudio-rs
- **Linux**: tokio-rustls, scap (screen capture)

## 8. Dependency Graph Summary

### Core Dependencies (must be addressed first):
1. **proto** (1,363 LOC) - Foundation for all communication
2. **rpc** (2,099 LOC) - Built on proto
3. **client** (4,160 LOC) - Built on rpc, used by 33 crates

### Second Tier (depends on core):
1. **channel** (3,022 LOC) - Depends on client, rpc
2. **call** (2,329 LOC) - Depends on client, audio, livekit_client
3. **remote** (2,708 LOC) - Depends on rpc

### Third Tier (specialized features):
1. **livekit_client** (2,679 LOC) - Used by call
2. **extension_host** (7,898 LOC) - Depends on client, remote
3. **auto_update** (1,189 LOC) - Depends on client

### Standalone/Support:
1. **audio** (144 LOC) - Minimal dependencies
2. **livekit_api** (293 LOC) - HTTP-based API
3. **extension** (1,957 LOC) - Core extension functionality
4. **remote_server** (3,602 LOC) - Server binary

## Removal Strategy Recommendations

Based on this analysis, the recommended removal order would be:

1. **Phase 1**: Remove call/video features
   - Remove call, livekit_client, livekit_api
   - Update workspace, zed to remove call dependencies

2. **Phase 2**: Remove channel features
   - Remove channel crate
   - Update notifications, zed

3. **Phase 3**: Remove remote development
   - Remove remote, remote_server
   - Update project, workspace, extension_host

4. **Phase 4**: Remove auto-update
   - Remove auto_update, auto_update_ui
   - Update zed

5. **Phase 5**: Remove core communication
   - Remove client, rpc, proto
   - This requires the most work as 33 crates depend on client

6. **Phase 6**: Clean up remaining dependencies
   - Remove collab test infrastructure
   - Clean up any remaining server-communication code

Total estimated lines of code to remove/modify: ~85,000+ LOC