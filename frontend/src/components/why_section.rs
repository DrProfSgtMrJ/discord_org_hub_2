use dioxus::prelude::*;

const WHY_CSS: Asset = asset!("/assets/styling/why_section.css");

struct WhyCardImage {
    image: Asset,
    class: &'static str,
}
struct WhyCard {
    icon: &'static str,
    title: &'static str,
    description: &'static str,
    images: &'static [WhyCardImage],
}

const WHY_CARDS: &[WhyCard] = &[WhyCard {
    icon: "📖",
    title: "Never Lose History",
    description: "Preserve your season history. Let others view your vast seasons quickly!",
    images: &[
        WhyCardImage {
            image: asset!("/assets/bot_seasons_example.png"),
            class: "why-card-image-seasons-example",
        },
        WhyCardImage {
            image: asset!("/assets/bot_season_info_example.png"),
            class: "why-card-image-seasons-example",
        },
    ],
}];
#[component]
pub fn WhySection() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: WHY_CSS }
        section { id: "why",
            h2 { id: "why-heading", "Why Discord Org Hub?" }
            div { id: "why-grid",
                for card in WHY_CARDS {
                    div { class: "why-card",
                        div { class: "why-card-content",
                            span { class: "why-icon", "{card.icon}" }
                            h3 { class: "why-title", "{card.title}" }
                            p { class: "why-desc", "{card.description}" }
                        }
                        if !card.images.is_empty() {
                            div { class: "why-card-images",
                                for image in card.images {
                                    img { class: image.class, src: image.image }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
