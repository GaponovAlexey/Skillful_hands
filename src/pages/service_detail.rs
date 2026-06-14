use dioxus::prelude::*;

use crate::components::{ContactForm, Icon, ScrollLink};
use crate::Route;

/// Детальная страница услуги (Pencil DOYUn/Ir8HP/MJ0tT/ArMJd/sgGfF).
/// Один параметрический шаблон + данные на 5 услуг, выбор по slug.
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
                            a { href: "tel:+17782396704", class: "btn btn--outline", "Call (778) 239 6704" }
                        }
                    }
                    div {
                        class: "sd-hero__photo",
                        style: "background-image:url('{data.hero_img}')",
                    }
                }
            }

            // B. Features
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

            // C. Gallery
            section { class: "sd-gallery",
                div { class: "wrap sd-gallery__inner",
                    h2 { class: "sd-h2", "{data.gallery_title}" }
                    div { class: "sd-gallery__row",
                        for img in data.gallery.iter() {
                            div {
                                key: "{img}",
                                class: "sd-gallery__photo",
                                style: "background-image:url('{img}')",
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
    gallery_title: &'static str,
    gallery: Vec<String>,
    closing_title: &'static str,
}

// Шаги процесса одинаковы на всех 5 услугах — общий набор.
const PROCESS_STEPS: [(&str, &str, &str); 4] = [
    ("01", "Consultation", "We visit your property, listen to your goals, and assess the space."),
    ("02", "Plan & quote", "You get a clear design direction and a transparent, itemized quote."),
    ("03", "We build", "Our crew gets to work — tidy, on schedule, and to spec."),
    ("04", "Walkthrough", "We review every detail together and make sure you're thrilled."),
];

fn feature(icon: &'static str, title: &'static str, desc: &'static str) -> Feature {
    Feature { icon, title, desc }
}

/// Unsplash-фото: базовый id + общий query (как на Home). Hero — шире (w=1600).
fn hero(id: &str) -> String {
    format!("https://images.unsplash.com/{id}?crop=entropy&cs=tinysrgb&fit=max&fm=jpg&q=80&w=1600")
}
fn photo(id: &str) -> String {
    format!("https://images.unsplash.com/{id}?crop=entropy&cs=tinysrgb&fit=max&fm=jpg&q=80&w=1080")
}

fn service_by_slug(slug: &str) -> ServiceData {
    match slug {
        "fencing" => fencing(),
        "landscape-lighting" => landscape_lighting(),
        "irrigation-maintenance" => irrigation_maintenance(),
        "civil-trucking" => civil_trucking(),
        // construction + неизвестный slug → первая услуга
        _ => construction(),
    }
}

fn construction() -> ServiceData {
    ServiceData {
        name: "Construction",
        intro: "Expertly crafting durable and stunning outdoor spaces with precision, quality materials, and seamless execution — built to last for years to come.",
        hero_img: hero("photo-1659468551190-6aeeb5b5cfed"),
        features_title: "Everything that goes into a lasting build",
        features: vec![
            feature("ruler", "Site assessment & design", "We survey grades, drainage, and soil before a single stake goes in."),
            feature("layers", "Quality materials", "Premium stone, concrete, and timber sourced to last decades."),
            feature("hammer", "Expert execution", "Seasoned crews who build to spec and finish on schedule."),
            feature("sparkles", "Clean handover", "A spotless site and a walkthrough so you know exactly what you got."),
        ],
        gallery_title: "Recent construction projects",
        gallery: vec![
            photo("photo-1586519822803-a53728428437"),
            photo("photo-1769690093863-4f2a3fd17dd2"),
            photo("photo-1654466505733-47eb25188942"),
        ],
        closing_title: "Ready to start your construction project?",
    }
}

fn fencing() -> ServiceData {
    ServiceData {
        name: "Fencing",
        intro: "Combining security, privacy, and elegance, our premium fencing solutions are built to last — enhancing your property with durability, style, and expert craftsmanship.",
        hero_img: hero("photo-1695550056778-de79c8cd909e"),
        features_title: "Fencing done right, down to the post",
        features: vec![
            feature("ruler", "Custom design", "Styles and heights tailored to your property and bylaws."),
            feature("layers", "Premium materials", "Cedar, vinyl, aluminum, and chain-link built for our climate."),
            feature("hammer", "Solid installation", "Properly set posts and footings that won't shift or lean."),
            feature("shield-check", "Privacy & security", "Enclosures that keep kids and pets in, and prying eyes out."),
        ],
        gallery_title: "Recent fencing projects",
        gallery: vec![
            photo("photo-1772682185743-e9030b833de4"),
            photo("photo-1762608676367-e0016a4f4950"),
            photo("photo-1643712153591-624535612928"),
        ],
        closing_title: "Ready to fence in your yard?",
    }
}

fn landscape_lighting() -> ServiceData {
    ServiceData {
        name: "Landscape Lighting",
        intro: "Transform your property after dark with professional landscape lighting — stunning accent, path, hardscape, and underwater lighting from industry-leading brands.",
        hero_img: hero("photo-1765042993443-671465685f8f"),
        features_title: "Light that flatters every feature",
        features: vec![
            feature("lightbulb", "Accent lighting", "Highlight trees, walls, and architecture with warm, even light."),
            feature("route", "Path & step lighting", "Safe, inviting walkways that glow without glare."),
            feature("waves", "Water features", "Bring pools, ponds, and fountains to life after sunset."),
            feature("settings", "Smart controls", "Dimming, zones, and timers you run from your phone."),
        ],
        gallery_title: "Recent lighting projects",
        gallery: vec![
            photo("photo-1766491794481-cd7d8f9aaf4d"),
            photo("photo-1731452716638-58b1699b2af7"),
            photo("photo-1762275167395-e50462b6595c"),
        ],
        closing_title: "Ready to light up your nights?",
    }
}

fn irrigation_maintenance() -> ServiceData {
    ServiceData {
        name: "Irrigation & Maintenance",
        intro: "Keep your outdoor space pristine year-round with expert maintenance — lawn care, seasonal clean-ups, and customized irrigation programs that enhance beauty and value effortlessly.",
        hero_img: hero("photo-1687827900884-131e513ba9b3"),
        features_title: "A healthy yard, handled for you",
        features: vec![
            feature("droplets", "Smart irrigation", "Efficient systems that water the right zones at the right time."),
            feature("scissors", "Lawn & garden care", "Mowing, pruning, weeding, and feeding on a regular schedule."),
            feature("leaf", "Seasonal clean-ups", "Spring wake-ups and fall prep to protect your investment."),
            feature("calendar-check", "Custom programs", "Tailored plans that fit your yard and your budget."),
        ],
        gallery_title: "Recent maintenance projects",
        gallery: vec![
            photo("photo-1689032402083-38b2b911f2ab"),
            photo("photo-1713301610475-7fff48ac53a2"),
            photo("photo-1761867660739-cdafe1141021"),
        ],
        closing_title: "Ready for a yard that stays pristine?",
    }
}

fn civil_trucking() -> ServiceData {
    ServiceData {
        name: "Civil & Trucking",
        intro: "Ensure your project runs smoothly with expert civil and trucking services — excavation, earthmoving, drainage solutions, and commercial transport to meet all your construction needs.",
        hero_img: hero("photo-1598067390123-9e3deec02c17"),
        features_title: "Heavy lifting, handled in-house",
        features: vec![
            feature("truck", "Excavation & earthmoving", "Site prep, grading, and digging with the right machines."),
            feature("layers", "Drainage solutions", "Grading and drains that move water away from structures."),
            feature("boxes", "Material hauling", "Reliable transport of soil, gravel, and aggregate."),
            feature("hard-hat", "Commercial transport", "Permitted, insured trucking for jobs of any scale."),
        ],
        gallery_title: "Recent civil projects",
        gallery: vec![
            photo("photo-1780054984787-b7e47746f5e0"),
            photo("photo-1752342625676-44566a11be8b"),
            photo("photo-1760045788252-d8d386ea1d12"),
        ],
        closing_title: "Ready to break ground?",
    }
}
