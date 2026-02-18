use dioxus::prelude::*;

const WHY_CSS: Asset = asset!("/assets/styling/why_section.css");

#[component]
pub fn WhySection() -> Element {
    rsx! {
        document::Link {rel: "stylesheet", href: WHY_CSS}
        div {
            id: "why-heading",
            h2 { "Why Use Discord Org Hub?" },
        }
    }
}
