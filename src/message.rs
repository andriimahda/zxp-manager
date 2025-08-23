use dioxus::prelude::*;
use std::time::Duration;

pub fn show_error(mut error: Signal<String>, message: String) {
    // Follow the same pattern as remove button: clone before capture
    let error_clone = error.clone();
    error.set(message);
    
    // Use Dioxus's spawn with tokio sleep for desktop
    spawn(async move {
        tokio::time::sleep(Duration::from_secs(4)).await;
        let mut error = error_clone; // Make mutable inside closure
        error.set(String::new());
    });
}