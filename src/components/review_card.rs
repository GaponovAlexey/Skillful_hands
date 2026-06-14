use dioxus::prelude::*;

use crate::components::Icon;

/// Карточка отзыва (Pencil Sq5op).
#[component]
pub fn ReviewCard(
    quote: String,
    name: String,
    role: String,
    #[props(default = 5)] stars: i32,
) -> Element {
    rsx! {
        div { class: "review-card",
            div { class: "review-card__stars",
                for i in 0..stars {
                    Icon { key: "{i}", name: "star".to_string(), size: 18 }
                }
            }
            p { class: "review-card__quote", "\u{201C}{quote}\u{201D}" }
            div { class: "review-card__author",
                div { class: "review-card__avatar" }
                div {
                    div { class: "review-card__name", "{name}" }
                    div { class: "review-card__role", "{role}" }
                }
            }
        }
    }
}
