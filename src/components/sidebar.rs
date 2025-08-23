use dioxus::prelude::*;
use crate::file_operations::{select_zxp_file, install_zxp};
use crate::message::show_error;

#[component]
pub fn Sidebar() -> Element {
    let refresh = use_context::<Signal<bool>>();
    let error = use_context::<Signal<String>>();
    
    let install_handler = move |_| {
        let mut refresh = refresh.clone();
        let mut error = error.clone();
        spawn(async move {
            match select_zxp_file() {
                Ok(zxp_path) => {
                    log::info!("Selected ZXP file: {:?}", zxp_path);
                    match install_zxp(&zxp_path) {
                        Ok(_) => {
                            log::info!("ZXP installation successful");
                            error.set(String::new()); // Clear any previous errors
                            refresh.set(!refresh()); // Trigger refresh
                        }
                        Err(e) => {
                            let error_msg = format!("Installation failed: {}", e);
                            log::error!("{}", error_msg);
                            show_error(error, error_msg);
                        }
                    }
                }
                Err(e) => {
                    log::info!("File selection cancelled or failed: {}", e);
                    // Don't show cancellation as error - it's user choice
                }
            }
        });
    };

    rsx! {
        div { class: "section sidebar",
            div { class: "install-section",

                div { class: "drop-zone",
                    span { class: "drop-icon", dangerous_inner_html: include_str!("../../assets/icons/download.svg") }
                    div { class: "drop-text", "Drop ZXP files here" }
                    div { class: "drop-subtext", "or click to browse" }
                    button { 
                        class: "browse-btn",
                        onclick: install_handler,
                        "Browse Files" 
                    }
                }
            }

            div { class: "settings-section",

                div { class: "setting-item",
                    label { class: "setting-label", "CEP Extensions Path" }
                    div { class: "setting-value", "~/Library/Application Support/Adobe/CEP/extensions/" }
                }

                div { class: "setting-item",
                    label { class: "setting-label", "User Extensions Path" }
                    div { class: "setting-value", "~/Library/Application Support/Adobe/CEP/extensions/" }
                }

                div { class: "setting-item",
                    label { class: "setting-label", "Test Error Timeout" }
                    button { 
                        class: "browse-btn",
                        onclick: move |_| {
                            let error = error.clone();
                            show_error(error, "Test error message - should disappear in 4 seconds".to_string());
                        },
                        "Test Error" 
                    }
                }

            }
        }
    }
}