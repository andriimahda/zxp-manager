use dioxus::prelude::*;
use crate::message::{MESSAGE, REFRESH_TRIGGER};

#[component]
pub fn StatusBar() -> Element {
    // React to refresh signal to count plugins
    let plugin_count = use_resource(move || {
        let _ = REFRESH_TRIGGER(); // Create dependency on global signal
        async move {
            match crate::data_operations::scan_cep_plugins() {
                Ok(plugins) => plugins.len(),
                Err(_) => 0,
            }
        }
    });
    
    // Read message once to avoid multiple borrows
    let current_message = MESSAGE.read();
    
    rsx! {
        div { class: "status-bar",
            if !current_message.content.is_empty() {
                div { 
                    class: "message",
                    "data-type": "{current_message.msg_type:?}",
                    "{current_message.content}" 
                }
            } else {
                // Show normal status
                match &*plugin_count.read() {
                    Some(count) => rsx! { 
                        div { "ZXP Manager v1.0.0 | Plugins installed: {count}" }
                    },
                    None => rsx! { 
                        div { "ZXP Manager v1.0.0 | Loading..." }
                    }
                }
            }
        }
    }
}