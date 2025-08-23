# Dioxus 0.6 Rules and Best Practices

## Rules of Hooks

### Core Rules
1. **Hook order must be consistent**: Same hooks called in same order every render
2. **Hooks only in components/hooks**: Cannot use hooks in regular functions
3. **No conditional hooks**: Don't use hooks inside `if`, `for`, or closures
4. **Hook naming**: Custom hooks should start with `use_`

### ❌ Wrong
```rust
if condition {
    let state = use_signal(|| 0);  // Bad: conditional hook
}
```

### ✅ Correct  
```rust
let state = use_signal(|| 0);  // Always call hook
if condition {
    // Use the state here
}
```

## State Management Hooks

### `use_signal`
- **Purpose**: Basic mutable, tracked state
- **Best for**: Component-local state that triggers re-renders
- **Example**: `let mut count = use_signal(|| 0);`

### `use_memo`
- **Purpose**: Derived state with memoization
- **Best for**: Expensive computations based on other signals
- **Memoized**: Uses `PartialEq` to avoid recomputation
- **Example**: `let doubled = use_memo(move || count() * 2);`

### `use_resource`
- **Purpose**: Async derived state
- **Best for**: Data fetching, async operations
- **Not memoized**: Always reruns when dependencies change
- **Example**: `let data = use_resource(move || fetch_user_data(user_id()));`

### `use_effect`
- **Purpose**: Side effects when tracked values change
- **Best for**: Logging, DOM manipulation, external API calls
- **Example**: `use_effect(move || println!("Count changed: {}", count()));`

### `use_hook`
- **Purpose**: One-time initialization
- **Best for**: Setting up state that only runs once
- **Example**: `let timer = use_hook(|| Timer::new());`

## Context System

### When to Use Context vs Props
- **Props**: For direct parent-child communication, keeps components reusable
- **Context**: For deeply nested sharing, avoids "prop drilling"

### Context Hooks
- `use_context_provider(value)`: Provide context to children
- `use_context()`: Access context (panics if not found)
- `try_consume_context()`: Safe context access (returns `Option`)

### ✅ Proper Context Implementation Pattern

**Use Signal Directly - No Custom Wrappers Needed**

```rust
// ✅ CORRECT: Provide Signal directly
fn App() -> Element {
    let _context = use_context_provider(|| Signal::new(false));
    rsx! { ChildComponent {} }
}

// ✅ CORRECT: Use Signal directly
fn ChildComponent() -> Element {
    let refresh = use_context::<Signal<bool>>();
    
    // Natural usage - no complex syntax
    refresh.set(!refresh());  // Trigger update
    let _ = refresh();        // Create dependency
    
    rsx! { /* UI */ }
}
```

**❌ Don't Overcomplicate with Custom Structs**

```rust
// ❌ WRONG: Unnecessary wrapper struct
#[derive(Clone)]
struct MyContext {
    signal: Signal<bool>,
}

// ❌ WRONG: Complex access patterns
refresh.signal.set(!(refresh.signal)());
```

**Key Insight**: Dioxus `Signal` is already designed for cross-component sharing. Use built-in types directly instead of creating custom wrappers - the simpler approach is the correct one.

## Async Patterns

### `spawn()`
- **Purpose**: Background tasks that don't return values
- **Best for**: Fire-and-forget operations
- **Example**: `spawn(async move { send_analytics().await; });`

### `use_resource()`
- **Purpose**: Async state that reruns on dependency changes
- **Best for**: Data fetching with automatic refresh
- **Returns**: `Resource<T>` with loading states

### Async Event Handlers
- Can return futures directly
- Automatically spawned by Dioxus
- **Example**: `onclick: move |_| async move { save_data().await }`

## Event Handling

### Event Handler Patterns
- Use `move` closures to capture state
- Event objects provide detailed interaction data
- Support `stop_propagation()` and `prevent_default()`

### ✅ Good Pattern
```rust
button { 
    onclick: move |event| {
        event.stop_propagation();
        count.set(count() + 1);
    },
    "Click me" 
}
```

### Avoiding Borrowing Issues in Event Handlers

#### ❌ Common Problem: Borrowing in Event Handlers
```rust
// This creates borrowing conflicts and lifetime issues
let data = use_resource(|| fetch_data());

rsx! {
    if let Some(items) = &*data.read() {
        for item in items {
            button {
                // ❌ Error: borrowed value does not live long enough
                onclick: move |_| handle_click(item.id), 
                "{item.name}"
            }
        }
    }
}
```

#### ✅ Solution: Clone Data Before Event Handlers
```rust
// Clone data outside the closure to create owned values
let data = use_resource(|| fetch_data());

rsx! {
    if let Some(items) = &*data.read() {
        for item in items {
            {
                let item_id = item.id.clone(); // Clone outside closure
                let item_name = item.name.clone();
                rsx! {
                    button {
                        // ✅ Correct: use owned data in closure
                        onclick: move |_| handle_click(item_id.clone()),
                        "{item_name}"
                    }
                }
            }
        }
    }
}
```

#### Key Principles for Event Handlers:
1. **Clone before capture**: Always clone data before moving into closures
2. **Avoid borrowing**: Don't borrow from `use_resource` inside event handlers
3. **Use owned data**: Event handlers need `'static` lifetime, use owned values
4. **Pattern**: `let owned_data = borrowed_data.clone();` then `move |_| use(owned_data)`

#### Real-World Example:
```rust
// Reading from use_resource and handling in events
let plugins = use_resource(|| scan_plugins());

rsx! {
    if let Some(plugin_list) = &*plugins.read() {
        for plugin in plugin_list {
            {
                let plugin_path = plugin.path.clone(); // Clone outside
                rsx! {
                    button {
                        onclick: move |_| remove_plugin(plugin_path.clone()),
                        "Remove {plugin.name}"
                    }
                }
            }
        }
    }
}
```

## Dynamic Rendering

### Conditional Rendering
- Use standard Rust control flow (`if`, `match`)
- Return `None` for no render
- **Example**: `if logged_in { rsx! { "Welcome!" } } else { rsx! { "Login" } }`

### Lists
- Always provide unique `key` for list items
- Use `.iter().map()` or `for` loops in `rsx!`
- **Example**: `for item in items() { ListItem { key: "{item.id}", item } }`

## Architecture Best Practices

### State Hierarchy
1. **Local state**: `use_signal` in component
2. **Shared state**: Context for component trees
3. **Global state**: `Global::new()` sparingly

### Component Organization
- Keep business logic separate from UI (use separate modules)
- Pass functions as props for actions
- Use context for widely-shared state

### Error Handling
- Use `Result` types in async operations
- Handle loading states with `use_resource`
- Provide fallback UI for error cases

## Performance Tips

### Avoid Infinite Re-renders
- **Never mutate state in component body**
- Use `use_effect` for side effects after render
- Be careful with derived state dependencies

### Memoization
- Use `use_memo` for expensive computations
- Implement `PartialEq` for complex state types
- Consider `use_resource` over `use_memo` for async operations

## Our Plugin Manager Application

### Recommended Pattern
```rust
// State management
let plugins = use_signal(|| Vec::<Plugin>::new());
let loading = use_signal(|| true);

// Async plugin scanning
let scan_task = use_resource(move || scan_plugins());

// Event handlers
let install_plugin = move |_| spawn(async move {
    if let Ok(file) = rfd::FileDialog::new().pick_file() {
        install_zxp(file).await;
        // Trigger refresh by updating signal
    }
});
```

This follows Dioxus patterns while keeping our desktop app architecture clean and reactive.

## Global Signals Across Modules

### Context Provider Setup (main.rs)

Create global signals in your root App component:

```rust
#[component]
fn App() -> Element {
    // Create global signals using context providers
    let _refresh_context = use_context_provider(|| Signal::new(false));
    let _error_context = use_context_provider(|| Signal::new(String::new()));
    
    rsx! {
        // Your app content
        div { class: "app",
            Sidebar {}
            MainContent {}
            StatusBar {}
        }
    }
}
```

**Key Points:**
- Use `use_context_provider` to create globally accessible signals
- Prefix with `_` if you don't need the return value directly
- Initialize with appropriate default values

### Consuming Signals in Components

Access global signals in any component:

```rust
#[component]
pub fn StatusBar() -> Element {
    let error = use_context::<Signal<String>>();
    let refresh = use_context::<Signal<bool>>();
    
    rsx! {
        div { class: "status-bar",
            if !error().is_empty() {
                div { class: "error-message", "{error()}" }
            } else {
                div { "Normal status" }
            }
        }
    }
}
```

### Cross-Module Signal Operations

Create utility functions in separate modules:

```rust
// src/message.rs
use dioxus::prelude::*;
use std::time::Duration;

pub fn show_error(mut error: Signal<String>, message: String) {
    // Clone before capture pattern
    let error_clone = error.clone();
    error.set(message); // Set immediately
    
    // Async cleanup after delay
    spawn(async move {
        tokio::time::sleep(Duration::from_secs(4)).await;
        let mut error = error_clone; // Make mutable inside closure
        error.set(String::new()); // Clear after timeout
    });
}
```

**Key Pattern - Clone Before Capture:**
1. Clone signals before moving into closures: `let error_clone = error.clone();`
2. Use original for immediate action: `error.set(message);`
3. Move clone into async closure: `spawn(async move { ... })`
4. Make mutable inside closure: `let mut error = error_clone;`

### Using Cross-Module Functions

Import and use utility functions in components:

```rust
// src/components/sidebar.rs
use crate::message::show_error;

let install_handler = move |_| {
    let error = error.clone(); // Clone for closure
    spawn(async move {
        match install_plugin().await {
            Ok(_) => {
                // Success - no error to show
            }
            Err(e) => {
                let error_msg = format!("Installation failed: {}", e);
                show_error(error, error_msg);
            }
        }
    });
};
```

### Global Signals Best Practices

#### ✅ DO:
- Use `use_context_provider` in your root component
- Clone signals before moving into closures
- Make parameters `mut` when functions need to modify signals
- Use descriptive signal types: `Signal<String>`, `Signal<bool>`, etc.
- Keep utility functions focused and reusable

#### ❌ DON'T:
- Try to move signals between threads (`thread::spawn`)
- Forget to clone before capture in event handlers
- Create wrapper structs around signals (use `Signal<T>` directly)
- Pass signals as props when context is available