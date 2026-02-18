use dioxus::prelude::*;

use crate::components::{BotInvite, HeroHeader};

const HERO_CSS: Asset = asset!("assets/styling/hero.css");

//const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
pub fn Hero() -> Element {
    rsx! {
        document::Link {rel: "stylesheet", href: HERO_CSS }
        div { id: "hero", HeroHeader {} BotInvite {  } }
    }
}
