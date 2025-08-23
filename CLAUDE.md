# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

ZXP Manager is a desktop application built with **Rust** and **Dioxus 0.6** for managing Adobe CEP (Common Extensibility Platform) extensions. It provides a GUI for installing, viewing, and removing ZXP plugin files for Adobe Creative Suite applications.

## Development Commands

```bash
# Development server
dx serve                    # Start development server
dx serve --platform desktop # Explicit desktop platform

# Building
cargo build                 # Debug build
cargo build --release       # Production build

# Code quality
cargo clippy               # Linting
cargo test                 # Run tests
```

## Architecture Overview

### Core Components Structure
```
App (main.rs) - Root component with global state providers
├── Sidebar - Install interface & settings display
├── PluginsPanel - Plugin table with remove actions  
└── StatusBar - Status/error messages & plugin count
```

### State Management Pattern
- **Global State**: Dioxus Context API with reactive signals
- **Cross-Component Communication**: `Signal<bool>` for refresh, `Signal<String>` for errors
- **Context Providers**: Established in main.rs App component

```rust
// Access shared state in components:
let refresh = use_context::<Signal<bool>>();
let error = use_context::<Signal<String>>();
```

### Key Modules

**`data_operations.rs`** - Plugin discovery and manifest parsing
- `scan_cep_plugins()` - Discovers plugins in CEP directory
- `parse_manifest_xml()` - Extracts metadata from CSXS/manifest.xml
- `can_remove_plugin()` - Checks file ownership permissions

**`file_operations.rs`** - File system operations
- `select_zxp_file()` - Native file picker
- `install_zxp()` - ZIP extraction to CEP directory  
- `remove_plugin()` - Directory removal with permissions

**`components/`** - UI components using Dioxus patterns

### Data Flow Patterns

1. **Reactive Updates**: Signal changes trigger `use_resource()` re-execution across components
2. **Async Operations**: Use `spawn()` for fire-and-forget, `use_resource()` for data fetching
3. **Error Handling**: Result types with custom enums → Context signals → UI display

### Adobe CEP Integration

- **Target Path**: `/Library/Application Support/Adobe/CEP/extensions/` (macOS)
- **Plugin Types**: Native (com.adobe.*) vs Third-party
- **Permissions**: User-owned removable, root-owned disabled (planned: elevated removal)

### Development Guidelines

Follow patterns in `DIOXUS_RULES_AND_BEST_PRACTICES.md`:
- Consistent hook order, no conditional hooks
- Direct Signal usage without wrapper structs  
- Clone-before-capture for event handlers
- Simplified Context provider/consumer patterns

### Git Commit Guidelines

- **Never include Claude or Anthropic attribution** in commit messages
- Keep commits clean and professional without AI co-author references

### Typical Development Flow

1. **Component Updates**: Modify component → refresh signal → automatic UI re-render
2. **New Features**: Add to appropriate module → wire up Context signals → update UI
3. **File Operations**: Extend `file_operations.rs` → handle errors → update state signals
4. **Plugin Discovery**: Extend `data_operations.rs` → update Plugin struct → refresh display