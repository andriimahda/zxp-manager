use dioxus::prelude::*;
use crate::data_operations::PluginType;
use crate::file_operations::remove_plugin;
use crate::message::show_error;

#[component]
pub fn PluginsPanel() -> Element {
    let refresh = use_context::<Signal<bool>>();
    let error = use_context::<Signal<String>>();
    
    // React to the shared refresh signal
    let plugins = use_resource(move || {
        let _ = refresh(); // Create dependency on the signal
        async move {
            crate::data_operations::scan_cep_plugins().unwrap_or_else(|e| {
                log::error!("Failed to scan plugins: {}", e);
                Vec::new()
            })
        }
    });


    rsx! {
        div { class: "section plugins-panel",

            div { class: "plugins-table-container",
                table { class: "plugins-table",
                    thead {
                        tr {
                            th { "Plugin" }
                            th { "Version" }
                            th { "Size" }
                            th { "Actions" }
                        }
                    }
                    tbody {
                        if let Some(plugin_list) = &*plugins.read() {
                            for plugin in plugin_list {
                                {
                                    let plugin_path = plugin.path.clone(); // Clone outside the closure
                                    let refresh = refresh.clone(); // Clone refresh for this button
                                    let error = error.clone(); // Clone error for this button
                                    rsx! {
                                        tr { key: "{plugin.path.display()}",
                                            td {
                                                "{plugin.name}"
                                                span {
                                                    class: if matches!(plugin.plugin_type, PluginType::Native) { "badge-native" } else { "badge-installed" },
                                                    if matches!(plugin.plugin_type, PluginType::Native) { "native" } else { "installed" }
                                                }
                                            }
                                            td { "{plugin.version}" }
                                            td { "{plugin.size}" }
                                            td { 
                                                button { 
                                                    class: "action-btn remove-btn",
                                                    disabled: !plugin.can_remove,
                                                    onclick: move |_| {
                                                        log::info!("Remove button clicked for: {:?}", plugin_path);
                                                        let mut refresh = refresh.clone();
                                                        let mut error = error.clone();
                                                        let plugin_path = plugin_path.clone();
                                                        spawn(async move {
                                                            log::info!("Starting plugin removal for: {:?}", plugin_path);
                                                            match remove_plugin(&plugin_path) {
                                                                Ok(_) => {
                                                                    log::info!("Plugin removed successfully: {:?}", plugin_path);
                                                                    error.set(String::new()); // Clear any previous errors
                                                                    refresh.set(!refresh()); // Trigger refresh
                                                                }
                                                                Err(e) => {
                                                                    let error_msg = format!("Failed to remove plugin: {}", e);
                                                                    log::error!("{}", error_msg);
                                                                    show_error(error, error_msg);
                                                                }
                                                            }
                                                        });
                                                    },
                                                    dangerous_inner_html: include_str!("../../assets/icons/trash.svg")
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            tr {
                                td { colspan: "4", "Loading plugins..." }
                            }
                        }
                    }
                }
            }
        }
    }
}