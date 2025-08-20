use dioxus::prelude::*;

#[component]
pub fn StatusBar() -> Element {
    rsx! {
        div { class: "status-bar",
            div { "ZXP Manager v1.0.0" }
        }
    }
}