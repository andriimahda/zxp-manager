use dioxus::prelude::*;
use std::time::Duration;
use tokio_util::sync::CancellationToken;
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub struct Message {
    pub content: String,
    pub msg_type: MessageType,
}

#[derive(Clone, Debug)]
pub enum MessageType {
    Success,
    Error,
    Info,
    None, // For empty/cleared state
}

// Global signal for the current message - accessible from anywhere
pub static MESSAGE: GlobalSignal<Message> = Signal::global(|| Message {
    content: String::new(),
    msg_type: MessageType::None,
});

// Global signal to hold the cancellation token for the current timer
// This allows us to cancel the previous message's timeout when a new message appears
static MESSAGE_CANCEL_TOKEN: GlobalSignal<Option<CancellationToken>> = Signal::global(|| None);

// Global refresh trigger signal - accessible from anywhere
pub static REFRESH_TRIGGER: GlobalSignal<bool> = Signal::global(|| false);

// Global signal for last installed plugin (for animation)
pub static LAST_INSTALLED_PLUGIN: GlobalSignal<Option<PathBuf>> = Signal::global(|| None);

pub fn mark_plugin_as_newly_installed(path: PathBuf) {
    *LAST_INSTALLED_PLUGIN.write() = Some(path);
}

pub fn clear_newly_installed_plugin() {
    *LAST_INSTALLED_PLUGIN.write() = None;
}

pub fn trigger_refresh() {
    // Read current value, then write opposite
    let current = REFRESH_TRIGGER();
    *REFRESH_TRIGGER.write() = !current;
}

pub fn show_message(content: String, msg_type: MessageType) {
    // Step 1: Cancel any existing timer to prevent old messages from interfering
    // If there's already a message timer running, we cancel it immediately
    if let Some(existing_token) = MESSAGE_CANCEL_TOKEN.read().as_ref() {
        existing_token.cancel(); // This stops the previous timer from clearing the message
        log::debug!("Cancelled previous message timer");
    }
    
    // Step 2: Create a new cancellation token for this message's timer
    // This token will be used to cancel this timer if another message appears
    let cancel_token = CancellationToken::new();
    *MESSAGE_CANCEL_TOKEN.write() = Some(cancel_token.clone());
    
    // Step 3: Display the new message immediately
    // User sees the new message right away, regardless of what was showing before
    *MESSAGE.write() = Message { content, msg_type: msg_type.clone() };
    log::debug!("Displayed new {:?} message: {}", msg_type, MESSAGE.read().content);
    
    // Step 4: Start the timeout timer in a separate async task
    spawn(async move {
        // Determine how long this message type should be displayed
        let timeout_secs = match msg_type {
            MessageType::Success => 3, // Success messages disappear quickly
            MessageType::Error => 4,   // Error messages stay a bit longer
            MessageType::Info => 5,    // Info messages stay longest
            MessageType::None => 0,    // None should not happen, but handle it
        };
        
        // Step 5: Wait for either the timeout OR cancellation
        // tokio::select! runs both futures and returns when the first one completes
        tokio::select! {
            // Branch A: Timer completed naturally - clear the message
            _ = tokio::time::sleep(Duration::from_secs(timeout_secs)) => {
                log::debug!("Message timer completed, clearing message");
                *MESSAGE.write() = Message {
                    content: String::new(),
                    msg_type: MessageType::None,
                };
                // Clean up our token since we're done with it
                *MESSAGE_CANCEL_TOKEN.write() = None;
            }
            
            // Branch B: Timer was cancelled - do nothing
            _ = cancel_token.cancelled() => {
                log::debug!("Message timer was cancelled by newer message");
                // Don't clear the message - a newer message is now showing
                // The newer message has its own timer that will handle clearing
            }
        }
    });
}

pub fn show_error(content: String) {
    show_message(content, MessageType::Error);
}

pub fn show_success(content: String) {
    show_message(content, MessageType::Success);
}

pub fn show_info(content: String) {
    show_message(content, MessageType::Info);
}