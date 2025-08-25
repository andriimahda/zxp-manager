use crate::data_operations::{Plugin, PluginType};
use crate::file_operations::remove_plugin;
use crate::message::{
    LAST_INSTALLED_PLUGIN, REFRESH_TRIGGER, clear_newly_installed_plugin, show_error, show_success,
    trigger_refresh,
};
use dioxus::prelude::*;
use std::path::PathBuf;

#[component]
fn PluginHeader() -> Element {
    rsx! {
        div { class: "plugin-grid-row plugins-header",
            div { class: "header-cell plugin-header", "Plugin" }
            div { class: "header-cell version-header", "Version" }
            div { class: "header-cell size-header", "Size" }
            div { class: "header-cell actions-header", "Remove" }
        }
    }
}

#[component]
fn PluginBadge(plugin_type: PluginType) -> Element {
    rsx! {
        span {
            class: if matches!(plugin_type, PluginType::Native) { "badge-native" } else { "badge-installed" },
            if matches!(plugin_type, PluginType::Native) { "native" } else { "installed" }
        }
    }
}

#[component]
fn RemoveButton(plugin_path: PathBuf, can_remove: bool) -> Element {
    rsx! {
        button {
            class: "remove-btn",
            disabled: !can_remove,
            onclick: move |_| {
                log::info!("Remove button clicked for: {:?}", plugin_path);
                let plugin_path = plugin_path.clone();
                spawn(async move {
                    log::info!("Starting plugin removal for: {:?}", plugin_path);
                    match remove_plugin(&plugin_path) {
                        Ok(_) => {
                            log::info!("Plugin removed successfully: {:?}", plugin_path);
                            show_success("Plugin removed successfully!".to_string());
                            trigger_refresh();
                        }
                        Err(e) => {
                            let error_msg = format!("Failed to remove plugin: {}", e);
                            log::error!("{}", error_msg);
                            show_error(error_msg);
                        }
                    }
                });
            },
            dangerous_inner_html: include_str!("../../assets/icons/trash.svg")
        }
    }
}

#[component]
fn PluginCard(plugin: Plugin, is_newly_installed: bool) -> Element {
    rsx! {
        div {
            key: "{plugin.path.display()}",
            class: if is_newly_installed { "plugin-grid-row plugin-card newly-added" } else { "plugin-grid-row plugin-card" },
            div { class: "plugin-info",
                div { class: "plugin-name",
                    "{plugin.name}"
                    PluginBadge { plugin_type: plugin.plugin_type }
                }
            }
            div { class: "plugin-version", "{plugin.version}" }
            div { class: "plugin-size", "{plugin.size}" }
            div { class: "plugin-actions",
                RemoveButton { plugin_path: plugin.path, can_remove: plugin.can_remove }
            }
        }
    }
}

#[component]
pub fn PluginsPanel() -> Element {
    let plugins = use_resource(move || {
        let _ = REFRESH_TRIGGER();
        async move {
            crate::data_operations::scan_cep_plugins().unwrap_or_else(|e| {
                log::error!("Failed to scan plugins: {}", e);
                Vec::new()
            })
        }
    });

    let last_installed = LAST_INSTALLED_PLUGIN();

    {
        let last_installed_clone = last_installed.clone();
        use_effect(move || {
            if last_installed_clone.is_some() {
                clear_newly_installed_plugin();
            }
        });
    }

    rsx! {
        div { class: "section plugins-panel",
            PluginHeader {}
            div { class: "plugins-grid",
                if let Some(plugin_list) = &*plugins.read() {
                    for plugin in plugin_list {
                        PluginCard {
                            plugin: plugin.clone(),
                            is_newly_installed: last_installed.as_ref() == Some(&plugin.path)
                        }
                    }
                } else {
                    div { class: "loading-message", "Loading plugins..." }
                }
            }
        }
    }
}

