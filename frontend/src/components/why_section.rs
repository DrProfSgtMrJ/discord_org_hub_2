use dioxus::prelude::*;
use dioxus_motion::prelude::*;

use crate::components::ImageCarousel;

const WHY_CSS: Asset = asset!("/assets/styling/why_section.css");

#[derive(Debug, Clone, PartialEq)]
pub struct WhyCard {
    icon: &'static str,
    title: &'static str,
    description: &'static str,
    images: &'static [Asset],
    back_title: &'static str,
    back_description: &'static str,
    back_bullet_points: &'static [&'static str],
}

const WHY_CARDS: &[WhyCard] = &[WhyCard {
    icon: "📖",
    title: "Never Lose History",
    description: "Preserve your season history. Let others view your vast seasons quickly!",
    images: &[asset!("/assets/bot_seasons_example.png"), asset!("/assets/bot_season_info_example.png")],
    back_title: "Commands",
    back_description: "View commands",
    back_bullet_points: &["/seasons", "/season_info <season_id>"],
}];

#[component]
pub fn AnimatedWhyCard(card: &'static WhyCard) -> Element {
    let mut expanded = use_signal(|| false);

    // Animates from 0.0 (front) to 1.0 (expanded)
    let mut slide = use_motion(0.0f32);

    let translate_x = slide.get_value() * -50.0; // slides left by 50%

    let handle_click = move |_| {
        let expanded_value = *expanded.read();
        let target = if expanded_value { 0.0f32 } else { 1.0f32 };
        expanded.set(!expanded_value);
        slide.animate_to(
            target,
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 200.0,
                damping: 22.0,
                mass: 1.0,
                ..Default::default()
            })),
        );
    };

    rsx! {
        div { class: "why-item",
            // animated card (left)
            div {
                class: "why-card why-card-animated",
                onclick: handle_click,
                div {
                    class: "why-card-track",
                    style: "transform: translateX({translate_x}%);",

                    // Front panel
                    div { class: "why-card-panel why-card-front",
                        div {class: "why-card-content",
                            span { class: "why-icon", "{card.icon}" }
                            h3 { class: "why-title", "{card.title}" }
                            p { class: "why-desc", "{card.description}"}
                            span { class: "why-card-hint", "Click to expand ->" }
                        }
                    }

                    // Back panel
                    div { class: "why-card-panel why-card-back",
                        div { class: "why-card-content",
                            h1 { class: "why-title", "{card.back_title}" }
                            p { class: "why-desc", "{card.back_description}"}
                            if !card.back_bullet_points.is_empty() {
                                ul { class: "why-tem-list-ul",
                                    for point in card.back_bullet_points {
                                        code { class: "why-tem-list-li", "{point}" }
                                    }
                                }
                            }
                            span { class: "why-card-hint", "<- Click to collapse" }
                        }
                    }
                }
            }

            // Carousel (right)
            div { class: "why-carousel-wrapper",
                ImageCarousel {  images: card.images }
            }
        }
    }
}

#[component]
pub fn WhySection() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: WHY_CSS }
        section { id: "why",
            h2 { id: "why-heading", "Why Discord Org Hub?" }
            div { id: "why-grid",
                for card in WHY_CARDS {
                    AnimatedWhyCard { card }
                }
            }
        }
    }
}
