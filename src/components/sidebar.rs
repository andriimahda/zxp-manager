use dioxus::prelude::*;

#[component]
pub fn Sidebar() -> Element {
    rsx! {
        div { class: "section sidebar",
            div { class: "install-section",
                h2 { class: "section-title",
                    span { dangerous_inner_html: include_str!("../../assets/icons/box.svg") }
                    "Install Plugin"
                }

                div { class: "drop-zone",
                    span { class: "drop-icon", dangerous_inner_html: include_str!("../../assets/icons/download.svg") }
                    div { class: "drop-text", "Drop ZXP files here" }
                    div { class: "drop-subtext", "or click to browse" }
                    button { class: "browse-btn", "Browse Files" }
                }
            }

            div { class: "settings-section",
                h3 { "Settings & Paths" }

                div { class: "setting-item",
                    label { class: "setting-label", "CEP Extensions Path" }
                    div { class: "setting-value", "~/Library/Application Support/Adobe/CEP/extensions/" }
                }

                div { class: "setting-item",
                    label { class: "setting-label", "User Extensions Path" }
                    div { class: "setting-value", "~/Library/Application Support/Adobe/CEP/extensions/" }
                }

                div { class: "setting-item",
                    label { class: "setting-label", "Installed Plugins" }
                    div { class: "setting-value", "3 active" }
                }
            }
        }
    }
}