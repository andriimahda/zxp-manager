# Elevated Plugin Removal Plan - macOS Authentication Dialog

## Current State
- ✅ User-owned plugins: Remove successfully
- ❌ Root-owned plugins: "Permission denied" error
- ✅ UI: Buttons disabled for root plugins

## Target State
- ✅ All plugins removable with macOS password dialog when needed

## Implementation

### 1. Enhanced File Operations

```rust
pub fn remove_plugin(plugin_path: &Path) -> Result<(), FileOperationError> {
    // Try normal removal first
    match fs::remove_dir_all(plugin_path) {
        Ok(_) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
            remove_plugin_elevated(plugin_path)
        }
        Err(e) => Err(FileOperationError::from(e))
    }
}

pub fn remove_plugin_elevated(plugin_path: &Path) -> Result<(), FileOperationError> {
    let script = format!(
        r#"do shell script "rm -rf '{}'" with administrator privileges"#,
        shell_escape(plugin_path.display().to_string())
    );
    
    let output = std::process::Command::new("osascript")
        .args(["-e", &script])
        .output()
        .map_err(|_| FileOperationError::ElevationFailed)?;
    
    if output.status.success() {
        Ok(())
    } else {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        if error_msg.contains("User canceled") {
            Err(FileOperationError::UserCancelled)
        } else {
            Err(FileOperationError::ElevationFailed)
        }
    }
}

fn shell_escape(input: String) -> String {
    input.replace('\\', "\\\\")
         .replace('"', "\\\"")
         .replace('$', "\\$")
         .replace('`', "\\`")
}
```

### 2. New Error Types

```rust
pub enum FileOperationError {
    // ... existing variants
    ElevationFailed,
    UserCancelled,
}
```

### 3. UI Changes

**Remove disabled buttons:**
```rust
button { 
    class: "action-btn remove-btn",
    // Remove: disabled: !plugin.can_remove,
    onclick: move |_| { /* handle with elevation */ },
}
```

**Update error handling:**
```rust
match remove_plugin(&plugin_path) {
    Ok(_) => {
        error.set(String::new());
        refresh.set(!refresh());
    }
    Err(FileOperationError::UserCancelled) => {
        // Don't show error - user choice
    }
    Err(FileOperationError::ElevationFailed) => {
        error.set("Failed to remove plugin with administrator privileges".to_string());
    }
    Err(e) => {
        error.set(format!("Failed to remove plugin: {}", e));
    }
}
```

### 4. Remove Ownership Checking

**Simplify Plugin struct:**
```rust
pub struct Plugin {
    pub name: String,
    pub version: String,
    pub size: String,
    pub path: PathBuf,
    pub plugin_type: PluginType,
    // Remove: pub can_remove: bool,
}
```

## Implementation Steps

1. **Add elevated removal function**
2. **Update error types** 
3. **Remove disabled button logic**
4. **Test with root-owned plugins**

## Expected Flow

1. User clicks remove button
2. Try normal removal
3. If permission denied → show macOS password dialog
4. Remove with sudo privileges
5. Refresh plugin list