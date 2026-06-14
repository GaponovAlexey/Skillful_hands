use dioxus::prelude::*;

use crate::components::{Icon, Logo, ScrollLink};
use crate::Route;

/// Подвал (Pencil G5CvcW). На каждой странице через Layout.
#[component]
pub fn Footer() -> Element {
    let services = [
        ("Construction", "construction"),
        ("Fencing", "fencing"),
        ("Landscape Lighting", "landscape-lighting"),
        ("Irrigation", "irrigation-maintenance"),
        ("Civil & Trucking", "civil-trucking"),
    ];

    rsx! {
        footer { class: "footer",
            div { class: "footer__inner",
                div { class: "footer__top",
                    // Бренд
                    div { class: "footer__brand",
                        Logo {}
                        p { class: "footer__tagline",
                            "Expert landscaping, irrigation, and fencing across Kelowna and Vancouver Island since 2011."
                        }
                        div { class: "footer__socials",
                            a { class: "footer__social", href: "#", "aria-label": "Facebook", Icon { name: "facebook".to_string() } }
                            a { class: "footer__social", href: "#", "aria-label": "Instagram", Icon { name: "instagram".to_string() } }
                        }
                    }
                    // Services
                    div { class: "footer__col",
                        span { class: "footer__col-title", "SERVICES" }
                        for (label , slug) in services {
                            Link { to: Route::ServiceDetail { slug: slug.to_string() }, "{label}" }
                        }
                    }
                    // Company
                    div { class: "footer__col",
                        span { class: "footer__col-title", "COMPANY" }
                        ScrollLink { target: "about", "About Us" }
                        ScrollLink { target: "projects", "Projects" }
                        ScrollLink { target: "testimonials", "Reviews" }
                        a { href: "#", "Careers" }
                    }
                    // Contact
                    div { class: "footer__col",
                        span { class: "footer__col-title", "CONTACT" }
                        a { href: "tel:+17782396704", "(778) 239 6704" }
                        span { "hello@skillfulhands.ca" }
                        span { "Kelowna & Vancouver Island" }
                    }
                }
                div { class: "footer__bottom",
                    span { "© 2026 Skillful Hands Solutions Ltd. All rights reserved." }
                    span {
                        Link { to: Route::Privacy {}, "Privacy Policy" }
                        "   ·   "
                        Link { to: Route::Terms {}, "Terms of Service" }
                    }
                }
            }
        }
    }
}
