use dioxus::prelude::*;
use crate::data_operations::PluginType;

#[component]
pub fn PluginsPanel() -> Element {
    let plugins = use_resource(|| async {
        crate::data_operations::scan_cep_plugins().unwrap_or_else(|e| {
            log::error!("Failed to scan plugins: {}", e);
            Vec::new()
        })
    });

    rsx! {
        div { class: "section plugins-panel",
            h2 { class: "section-title",
                span { dangerous_inner_html: include_str!("../../assets/icons/package-open.svg") }
                "Installed Plugins"
            }

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
                                        button { class: "action-btn remove-btn",
                                            dangerous_inner_html: include_str!("../../assets/icons/trash.svg")
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