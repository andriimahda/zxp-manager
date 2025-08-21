use dioxus::prelude::*;

#[component]
pub fn StatusBar() -> Element {
    let error = use_context::<Signal<String>>();
    let refresh = use_context::<Signal<bool>>();
    
    // React to refresh signal to count plugins
    let plugin_count = use_resource(move || {
        let _ = refresh(); // Create dependency on refresh
        async move {
            match crate::data_operations::scan_cep_plugins() {
                Ok(plugins) => plugins.len(),
                Err(_) => 0,
            }
        }
    });
    
    rsx! {
        div { class: "status-bar",
            if !error().is_empty() {
                // Show error message
                div { class: "error-message", "{error()}" }
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