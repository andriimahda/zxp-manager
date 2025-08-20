use dioxus::prelude::*;

mod data_operations;
mod file_operations;
mod components {
    pub mod sidebar;
    pub mod plugins_panel;
    pub mod status_bar;
}

use components::sidebar::Sidebar;
use components::plugins_panel::PluginsPanel;
use components::status_bar::StatusBar;

static FAVICON: Asset = asset!("/assets/favicon.ico");
static THEMES_CSS: Asset = asset!("/assets/themes.css");
static MAIN_CSS: Asset = asset!("/assets/main.css");
static INTER_FONT: Asset = asset!("/assets/fonts/Inter-VariableFont_opsz,wght.ttf");
static GOOGLE_SANS_CODE_FONT: Asset = asset!("/assets/fonts/GoogleSansCode-VariableFont_wght.ttf");

fn main() {
    use dioxus::desktop::{Config, tao::dpi::LogicalSize, tao::window::WindowBuilder};

    dioxus::LaunchBuilder::desktop()
        .with_cfg(
            Config::default().with_window(
                WindowBuilder::new()
                    .with_title("ZXP Manager")
                    .with_inner_size(LogicalSize::new(1200.0, 800.0))
                    .with_min_inner_size(LogicalSize::new(1000.0, 700.0))
                    .with_resizable(true),
            ),
        )
        .launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Style {
            r#"
            @font-face {{
                font-family: "Inter";
                src: url("{INTER_FONT}") format("truetype");
                font-weight: 100 900;
                font-style: normal;
                font-display: swap;
            }}
            @font-face {{
                font-family: "Google Sans Code";
                src: url("{GOOGLE_SANS_CODE_FONT}") format("truetype");
                font-weight: 100 900;
                font-style: normal;
                font-display: swap;
            }}
            "#
        }

        document::Stylesheet { href: THEMES_CSS }
        document::Stylesheet { href: MAIN_CSS }

        div { class: "container",
            div { class: "main-content",
                Sidebar {}
                PluginsPanel {}
            }
            StatusBar {}
        }
    }
}

