use dioxus::prelude::*;

use crate::components::Icon;
use crate::Route;

/// Карточка услуги (Pencil h0vaM4). Кликабельна → страница услуги.
#[component]
pub fn ServiceCard(icon: String, title: String, desc: String, slug: String) -> Element {
    rsx! {
        Link { to: Route::ServiceDetail { slug }, class: "service-card",
            span { class: "service-card__icon", Icon { name: icon, size: 26 } }
            h3 { class: "service-card__title", "{title}" }
            p { class: "service-card__desc", "{desc}" }
        }
    }
}
