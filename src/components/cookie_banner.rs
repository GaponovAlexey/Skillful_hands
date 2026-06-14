use dioxus::prelude::*;

use crate::components::Icon;

/// Cookie-баннер (Pencil lpP7I). Fixed снизу-слева, закрывается на Accept/Decline.
#[component]
pub fn CookieBanner() -> Element {
    let mut visible = use_signal(|| true);
    if !visible() {
        return rsx! {};
    }
    rsx! {
        div { class: "cookie",
            div { class: "cookie__top",
                span { class: "cookie__icon", Icon { name: "cookie".to_string(), size: 22 } }
                div {
                    div { class: "cookie__title", "We value your privacy" }
                    p { class: "cookie__body",
                        "We use cookies to improve your browsing experience and analyze site traffic. See our Privacy Policy."
                    }
                }
            }
            div { class: "cookie__buttons",
                button { class: "btn", onclick: move |_| visible.set(false), "Accept all" }
                button { class: "btn btn--ghost", onclick: move |_| visible.set(false), "Decline" }
            }
        }
    }
}
