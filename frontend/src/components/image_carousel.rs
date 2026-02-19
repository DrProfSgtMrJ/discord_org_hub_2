use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{LdChevronLeft, LdChevronRight};

const CAROUSEL_CSS: Asset = asset!("/assets/styling/image_carousel.css");

#[component]
pub fn ImageCarousel(images: &'static [Asset]) -> Element {
    let mut current_index = use_signal(|| 0usize);
    let mut slide = use_motion(0.0f32);
    let translate_x = slide.get_value();
    let show_controls = images.len() > 1;

    let go_prev = move |_| {
        let idx = current_index();
        let new_idx = if idx == 0 { images.len() - 1 } else { idx - 1 };
        current_index.set(new_idx);
        slide.animate_to(
            -(new_idx as f32) * 100.0,
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 300.0,
                damping: 28.0,
                mass: 1.0,
                ..Default::default()
            })),
        );
    };

    let go_next = move |_| {
        let idx = current_index();
        let new_idx = (idx + 1) % images.len();
        current_index.set(new_idx);
        slide.animate_to(
            -(new_idx as f32) * 100.0,
            AnimationConfig::new(AnimationMode::Spring(Spring {
                stiffness: 300.0,
                damping: 28.0,
                mass: 1.0,
                ..Default::default()
            })),
        );
    };

    rsx! {
        document::Link { rel: "stylesheet", href: CAROUSEL_CSS }
        div { class: "carousel",
            div {
                class: "carousel-track",
                style: "transform: translateX({translate_x}%);",
                for image in images {
                    img { class: "carousel-image", src: *image }
                }
            }
            if show_controls {
                button { class: "carousel-btn carousel-prev", onclick: go_prev,
                    Icon { icon: LdChevronLeft }
                }
                button { class: "carousel-btn carousel-next", onclick: go_next,
                    Icon { icon: LdChevronRight }
                }
                div { class: "carousel-dots",
                    for i in 0..images.len() {
                        span { class: if i == current_index() { "carousel-dot carousel-dot-active" } else { "carousel-dot" } }
                    }
                }
            }
        }
    }
}
