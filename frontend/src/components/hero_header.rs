use dioxus::prelude::*;

const LOGO: Asset = asset!("/assets/bot-logo-256.png");

#[component]
pub fn HeroHeader() -> Element {
    rsx! {
        div { id: "hero-header",
            div { id: "hero-title-row",
                img {
                    src: LOGO,
                    id: "hero-logo",
                    alt: "Discord Org Hub Logo",
                    width: "128",
                    height: "128",
                }
                div { id: "hero-title-text",
                    h1 { "Welcome to the Discord Org Hub!" }
                    p { class: "tagline",
                        "Manage your Discord Org with ease. Track seasons, members, and keep track of your Org history!"
                    }
                }
            }
        }
    }
}