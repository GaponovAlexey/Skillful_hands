use dioxus::prelude::*;

use crate::components::{ContactForm, Icon, ScrollLink};
use crate::pages::project_card;
use crate::Route;

/// Детальная страница услуги (Pencil DOYUn/Ir8HP/MJ0tT/ArMJd/sgGfF).
/// Один параметрический шаблон + данные на 4 услуги, выбор по slug.
#[component]
pub fn ServiceDetail(slug: String) -> Element {
    let data = service_by_slug(&slug);

    rsx! {
        main {
            // A. Detail Hero
            section { class: "sd-hero",
                div { class: "wrap sd-hero__inner",
                    div { class: "sd-hero__left",
                        div { class: "sd-crumb",
                            Link { to: Route::Services {}, class: "sd-crumb__root", "Services" }
                            Icon { name: "chevron-right".to_string(), size: 15 }
                            span { class: "sd-crumb__current", "{data.name}" }
                        }
                        h1 { class: "sd-hero__title", "{data.name}" }
                        p { class: "sd-hero__intro", "{data.intro}" }
                        div { class: "sd-hero__btns",
                            ScrollLink { target: "contact", class: "btn", "Get a free quote" }
                            a { href: "tel:+17782396704", class: "btn btn--outline", "Call (778) 239-6704" }
                        }
                    }
                    div {
                        class: "sd-hero__photo",
                        style: "background-image:url('{data.hero_img}')",
                    }
                }
            }

            // B. Related projects (реальные проекты — портфолио сразу под hero)
            if !data.related_projects.is_empty() {
                section { class: "sd-related",
                    div { class: "wrap sd-related__inner",
                        div { class: "sd-head",
                            span { class: "sd-eyebrow", "OUR WORK" }
                            h2 { class: "sd-h2", "Related projects" }
                        }
                        div { class: "sd-related__grid",
                            for slug in data.related_projects.iter().copied() {
                                if let Some((s , chip , img)) = project_card(slug) {
                                    Link {
                                        key: "{s}",
                                        to: Route::ProjectDetail { slug: s.to_string() },
                                        class: "sd-related__card",
                                        style: "background-image:url('{img}')",
                                        span { class: "sd-related__chip",
                                            "{chip}"
                                            Icon { name: "arrow-up-right".to_string(), size: 15 }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // C. Features
            section { class: "sd-features",
                div { class: "wrap sd-features__inner",
                    div { class: "sd-head",
                        span { class: "sd-eyebrow", "WHAT'S INCLUDED" }
                        h2 { class: "sd-h2", "{data.features_title}" }
                    }
                    div { class: "sd-feature-grid",
                        for f in data.features.iter() {
                            div { key: "{f.title}", class: "sd-feature",
                                span { class: "sd-feature__icon", Icon { name: f.icon.to_string(), size: 24 } }
                                div { class: "sd-feature__text",
                                    h3 { class: "sd-feature__title", "{f.title}" }
                                    p { class: "sd-feature__desc", "{f.desc}" }
                                }
                            }
                        }
                    }
                }
            }

            // D. Process (generic — идентичен на всех услугах)
            Process {}

            // E. Closing CTA
            section { class: "sd-cta",
                div { class: "wrap sd-cta__inner",
                    h2 { class: "sd-cta__title", "{data.closing_title}" }
                    ScrollLink { target: "contact", class: "btn btn--light", "Contact now" }
                }
            }

            // F. Contact
            ContactForm {}
        }
    }
}

// ---------- Process (generic, идентичен на всех услугах) ----------
#[component]
fn Process() -> Element {
    rsx! {
        section { class: "sd-process",
            div { class: "wrap sd-process__inner",
                div { class: "sd-head",
                    span { class: "sd-eyebrow", "HOW IT WORKS" }
                    h2 { class: "sd-h2", "A simple, no-surprises process" }
                }
                div { class: "sd-steps",
                    for step in PROCESS_STEPS.iter() {
                        div { key: "{step.0}", class: "sd-step",
                            div { class: "sd-step__num", "{step.0}" }
                            h3 { class: "sd-step__title", "{step.1}" }
                            p { class: "sd-step__desc", "{step.2}" }
                        }
                    }
                }
            }
        }
    }
}

// ============================================================
//  Данные услуг
// ============================================================

#[derive(Clone, PartialEq)]
struct Feature {
    icon: &'static str,
    title: &'static str,
    desc: &'static str,
}

#[derive(Clone, PartialEq)]
struct ServiceData {
    name: &'static str,
    intro: &'static str,
    hero_img: String,
    features_title: &'static str,
    features: Vec<Feature>,
    closing_title: &'static str,
    /// Связанные проекты (slug) — кросс-ссылки на страницы проектов.
    related_projects: Vec<&'static str>,
}

// Шаги процесса одинаковы на всех услугах — общий набор.
const PROCESS_STEPS: [(&str, &str, &str); 4] = [
    ("01", "Consultation", "We visit your property, listen to your goals, and assess the site."),
    ("02", "Design & Planning", "We map out a clear plan, design direction, and a transparent quote."),
    ("03", "Construction", "Our crew builds to spec — tidy, on schedule, and to a high standard."),
    ("04", "Project Completion", "We review every detail with you and hand over a result you're proud of."),
];

fn feature(icon: &'static str, title: &'static str, desc: &'static str) -> Feature {
    Feature { icon, title, desc }
}

fn service_by_slug(slug: &str) -> ServiceData {
    match slug {
        "outdoor-living" => outdoor_living(),
        "cottages-cabins" => cottages_cabins(),
        "landscaping" => landscaping(),
        // renovations + неизвестный slug → первая услуга
        _ => renovations(),
    }
}

fn renovations() -> ServiceData {
    ServiceData {
        name: "Renovations",
        intro: "Professional renovation services for residential properties — interior, exterior, and complete home renovations and restoration, backed by custom design and planning.",
        hero_img: asset!("/assets/img/foto_web/island-retreat-interior-after-07.jpg").to_string(),
        features_title: "Everything a quality renovation needs",
        features: vec![
            feature("ruler", "Custom design & planning", "We plan layout, grades, and finishes before any work begins."),
            feature("hammer", "Interior renovations", "Kitchens, baths, flooring, and full interior refreshes."),
            feature("layers", "Exterior renovations", "Siding, decks, facades, and weather-ready exterior work."),
            feature("sparkles", "Complete restoration", "Full home renovations and restoration, done right."),
        ],
        closing_title: "Ready to start your renovation project?",
        related_projects: vec!["rustic-utility-room", "island-retreat"],
    }
}

fn outdoor_living() -> ServiceData {
    ServiceData {
        name: "Outdoor Living Spaces",
        intro: "Custom decks, pergolas, and BBQ areas designed for durability, functionality, relaxation, and year-round outdoor living.",
        hero_img: asset!("/assets/img/foto_web/bbq-area-after-04.jpg").to_string(),
        features_title: "Built for how you live outdoors",
        features: vec![
            feature("hammer", "Custom decks", "Durable, beautiful decks built to fit your space."),
            feature("trees", "Pergolas & shade", "Pergolas and shade structures for comfort and style."),
            feature("sparkles", "BBQ & entertaining", "Built-in BBQ and entertaining areas for hosting."),
            feature("shield-check", "Built to last", "Quality materials that stand up to the coast climate."),
        ],
        closing_title: "Ready to build your outdoor living space?",
        related_projects: vec!["waterfront-bbq", "island-retreat"],
    }
}

fn cottages_cabins() -> ServiceData {
    ServiceData {
        name: "Cottages & Cabins",
        intro: "Custom cottages, cabins, guest houses, and other unique custom-built structures — including remote and hard-to-access sites.",
        hero_img: asset!("/assets/img/foto_web/island-retreat-exterior-after-03.jpg").to_string(),
        features_title: "Custom structures, built anywhere",
        features: vec![
            feature("house", "Cottages & cabins", "Custom cottages and cabins designed around your vision."),
            feature("ruler", "Guest houses", "Comfortable guest houses and bunkies."),
            feature("truck", "Remote-site capable", "Experienced with logistics in remote, hard-to-access locations."),
            feature("sparkles", "Custom structures", "One-of-a-kind custom-built structures."),
        ],
        closing_title: "Ready to build your cottage or cabin?",
        related_projects: vec!["island-retreat"],
    }
}

fn landscaping() -> ServiceData {
    ServiceData {
        name: "Landscaping",
        intro: "Landscaping and property improvement services for residential and waterfront properties across the Sunshine Coast.",
        hero_img: asset!("/assets/img/foto_web/scenery-01.jpg").to_string(),
        features_title: "Property improvement, inside out",
        features: vec![
            feature("leaf", "Residential landscaping", "Yards, plantings, and grounds that elevate your property."),
            feature("droplets", "Waterfront work", "Improvement projects for waterfront and shoreline properties."),
            feature("ruler", "Site & grounds", "Grading, hardscape, and site improvement."),
            feature("sparkles", "Property improvement", "Finishing touches that boost value and enjoyment."),
        ],
        closing_title: "Ready to transform your property?",
        related_projects: vec!["island-retreat"],
    }
}
