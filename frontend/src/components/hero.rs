use dioxus::prelude::*;

use crate::components::HeroHeader;

//const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
pub fn Hero() -> Element {
    rsx! {
        div { id: "hero", HeroHeader {} }
    }
}
