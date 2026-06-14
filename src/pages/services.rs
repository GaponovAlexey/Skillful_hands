use dioxus::prelude::*;

use crate::components::{ContactForm, Icon, ScrollLink};
use crate::Route;

/// Страница услуг (Pencil a79RGt «Services Page» → ребренд Skillful Hands).
#[component]
pub fn Services() -> Element {
    rsx! {
        main {
            ServicesHero {}
            ServicesList {}
            ClosingCta {}
            ContactForm {}
        }
    }
}

// ---------- Services Hero ----------
#[component]
fn ServicesHero() -> Element {
    rsx! {
        section { class: "svc-hero",
            div { class: "wrap",
                div { class: "svc-hero__inner",
                    span { class: "svc-hero__eyebrow", "SERVICES" }
                    h1 { class: "svc-hero__title", "Check out our landscaping services" }
                    p { class: "svc-hero__sub",
                        "Transform your outdoor space with Skillful Hands' expert landscaping services. From custom designs to seasonal maintenance, we create beautiful, functional landscapes tailored to your needs."
                    }
                }
            }
        }
    }
}

// ---------- Services List ----------
#[component]
fn ServicesList() -> Element {
    // (num, title, slug, desc, image base URL). Зигзаг — по индексу строки.
    let rows = [
        (
            "01",
            "Construction",
            "construction",
            "Expertly crafting durable and stunning outdoor spaces with precision, quality materials, and seamless execution — bringing your vision to life with construction landscaping that lasts.",
            "https://images.unsplash.com/photo-1672627170267-fca17bb54156",
        ),
        (
            "02",
            "Fencing",
            "fencing",
            "Combining security, privacy, and elegance, our premium fencing solutions are built to last — enhancing your property with durability, style, and expert craftsmanship.",
            "https://images.unsplash.com/photo-1681853108586-f29b4ef5c0fb",
        ),
        (
            "03",
            "Landscape Lighting",
            "landscape-lighting",
            "Transform your property after dark with professional landscape lighting — stunning accent, path, hardscape, and underwater lighting from industry-leading brands.",
            "https://images.unsplash.com/photo-1707189856923-46dd41ea2bdc",
        ),
        (
            "04",
            "Irrigation & Maintenance",
            "irrigation-maintenance",
            "Keep your outdoor space pristine year-round with expert maintenance — lawn care, seasonal clean-ups, and customized programs to enhance beauty and value effortlessly.",
            "https://images.unsplash.com/photo-1779888793240-898dec4b9c08",
        ),
        (
            "05",
            "Civil & Trucking",
            "civil-trucking",
            "Ensure your project runs smoothly with expert civil and trucking services — excavation, earthmoving, drainage solutions, and commercial transport for all your construction needs.",
            "https://images.unsplash.com/photo-1630288214173-a119cf823388",
        ),
    ];
    rsx! {
        section { class: "svc-list",
            div { class: "wrap",
                for (i , (num , title , slug , desc , img)) in rows.into_iter().enumerate() {
                    div {
                        key: "{slug}",
                        // Чётный индекс (строки 1,3,5) — фото слева (исходный порядок);
                        // нечётный (2,4) — reverse, фото уезжает вправо.
                        class: if i % 2 == 1 { "svc-row svc-row--rev" } else { "svc-row" },
                        div {
                            class: "svc-row__photo",
                            style: "background-image:url('{img}?crop=entropy&cs=tinysrgb&fit=max&fm=jpg&q=80&w=1080')",
                        }
                        div { class: "svc-row__text",
                            div { class: "svc-row__top",
                                span { class: "svc-row__num", "{num}" }
                                h2 { class: "svc-row__title", "{title}" }
                            }
                            p { class: "svc-row__desc", "{desc}" }
                            Link {
                                to: Route::ServiceDetail { slug: slug.to_string() },
                                class: "see-service",
                                "See service"
                                Icon { name: "arrow-right".to_string(), size: 16 }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ---------- Closing CTA ----------
#[component]
fn ClosingCta() -> Element {
    rsx! {
        section { class: "svc-cta",
            div { class: "wrap",
                div { class: "svc-cta__inner",
                    h2 { class: "svc-cta__title",
                        "Looking for a qualified, reliable and honest landscaping team?"
                    }
                    ScrollLink { target: "contact", class: "btn btn--light", "Contact now" }
                }
            }
        }
    }
}
