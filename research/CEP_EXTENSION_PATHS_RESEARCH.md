# Adobe CEP Extension Installation Paths Research

## Overview

Adobe CEP (Common Extensibility Platform) extensions can be installed at different levels depending on user permissions and target audience. Here are the official installation paths for different operating systems.

## Installation Directories

### macOS

**System-Level (All Users):**
- `/Library/Application Support/Adobe/CEP/extensions/`

**User-Level (Current User Only):**
- `~/Library/Application Support/Adobe/CEP/extensions/`

### Windows

**System-Level (All Users):**
- `C:\Program Files (x86)\Common Files\Adobe\CEP\extensions\` 
- `C:\Program Files\Common Files\Adobe\CEP\extensions\` (since CEP 6.1)

**User-Level (Current User Only):**
- `C:\Users\<USERNAME>\AppData\Roaming\Adobe\CEP\extensions\`

## Key Differences

### Permission Requirements

| Level | macOS | Windows | Admin Required |
|-------|--------|---------|----------------|
| System | `/Library/...` | `C:\Program Files\...` | Yes (sudo/admin) |
| User | `~/Library/...` | `%APPDATA%\Adobe\...` | No |

### Target Audience

- **System-Level**: Extensions available to all users on the machine
- **User-Level**: Extensions only available to the specific user who installed them

## Extension Discovery Priority

Adobe applications scan directories in this order:
1. System-level extensions directory
2. User-level extensions directory

If the same extension exists in both locations, the user-level version takes precedence.

## Implementation Considerations

### For ZXP Manager Application

1. **Cross-Platform Support**: Need to detect OS and use appropriate paths
2. **Permission Handling**: System-level installations require elevated privileges
3. **User Choice**: Allow users to choose installation level (system vs user)
4. **Directory Creation**: Create directories if they don't exist

### Directory Detection Logic

```rust
// macOS
fn get_cep_paths_macos() -> (PathBuf, PathBuf) {
    let system_path = PathBuf::from("/Library/Application Support/Adobe/CEP/extensions");
    let user_path = dirs::home_dir()
        .unwrap()
        .join("Library/Application Support/Adobe/CEP/extensions");
    (system_path, user_path)
}

// Windows
fn get_cep_paths_windows() -> (PathBuf, PathBuf) {
    let system_path = PathBuf::from("C:\\Program Files\\Common Files\\Adobe\\CEP\\extensions");
    let user_path = dirs::home_dir()
        .unwrap()
        .join("AppData\\Roaming\\Adobe\\CEP\\extensions");
    (system_path, user_path)
}
```

## Migration Notes

### CEP to UXP Transition

Adobe is gradually migrating from CEP to UXP (Unified Extensibility Platform):

**UXP Paths (for reference):**
- macOS: `~/Library/Application Support/Adobe/UXP/extensions/`
- Windows: `%APPDATA%\Adobe\UXP\extensions\`

### Version Compatibility

- **CEP 4.x-5.x**: Legacy paths (deprecated)
- **CEP 6.x+**: Current paths listed above
- **UXP**: Future replacement for CEP

## Sources

- Adobe CEP Getting Started Guides (GitHub)
- Adobe Community Forums
- Official Adobe CEP Documentation
- Developer community discussions

## Recommendations for ZXP Manager

1. **Default to User-Level**: Install to user directory by default (no admin required)
2. **Provide System Option**: Allow system-level install with appropriate permission prompts
3. **Scan Both Locations**: Display extensions from both system and user directories
4. **Handle Duplicates**: Show priority when same extension exists in both locations
5. **Cross-Platform**: Implement OS detection for appropriate path selection