use dioxus::prelude::*;

use crate::components::{ContactForm, Icon, ReviewCard, ScrollLink, ServiceCard};
use crate::Route;

/// Лендинг (Pencil C5tVy «Sweet Yards Landing» → ребренд Skillful Hands).
#[component]
pub fn Home() -> Element {
    rsx! {
        main {
            Hero {}
            Projects {}
            ServicesSection {}
            WhyUs {}
            Owners {}
            Testimonials {}
            CtaBand {}
            ContactForm {}
        }
    }
}

// ---------- Hero (hv0zC) ----------
#[component]
fn Hero() -> Element {
    rsx! {
        section { class: "hero",
            div { class: "hero__inner",
                div { class: "hero__eyebrow",
                    Icon { name: "map-pin".to_string(), size: 14 }
                    span { "Kelowna  ·  Vancouver Island  ·  Since 2011" }
                }
                h1 { class: "hero__headline", "Crafting outdoor spaces you'll never want to leave." }
                p { class: "hero__sub",
                    "Expert landscaping, irrigation, and fencing that turns ordinary yards into a picturesque outdoor oasis."
                }
                div { class: "hero__ctas",
                    Link { to: Route::Services {}, class: "btn", "Explore our services" }
                    ScrollLink { target: "contact", class: "btn btn--light", "Get a free quote" }
                }
            }
        }
    }
}

// ---------- Projects (GpsRO) ----------
#[component]
fn ProjectTile(label: String, img: String, #[props(default = false)] big: bool) -> Element {
    rsx! {
        div {
            class: if big { "tile tile--big" } else { "tile" },
            style: "background-image:url('{img}')",
            span { class: "tile__chip", "{label}" }
        }
    }
}

#[component]
fn Projects() -> Element {
    rsx! {
        section { id: "projects", class: "home-section home-section--surface",
            div { class: "wrap",
                div { class: "sec-head",
                    div { class: "sec-head__left",
                        span { class: "sec-eyebrow", "OUR WORK" }
                        h2 { class: "sec-title", "See some of our past projects" }
                    }
                    Link { to: Route::Services {}, class: "pill-link",
                        "View all projects"
                        Icon { name: "arrow-up-right".to_string(), size: 16 }
                    }
                }
                div { class: "gallery-top",
                    ProjectTile {
                        big: true,
                        label: "Custom Home",
                        img: "https://images.unsplash.com/photo-1515095182805-4367ad159fde?crop=entropy&cs=tinysrgb&fit=max&fm=jpg&q=80&w=1280",
                    }
                    div { class: "gallery-top__right",
                        ProjectTile {
                            label: "Lakeside Retreat",
                            img: "https://images.unsplash.com/photo-1667924599516-212cacfe3a2b?crop=entropy&cs=tinysrgb&fit=max&fm=jpg&q=80&w=1080",
                        }
                        ProjectTile {
                            label: "Resort Villa",
                            img: "https://images.unsplash.com/photo-1610195771759-7926c38df364?crop=entropy&cs=tinysrgb&fit=max&fm=jpg&q=80&w=1080",
                        }
                    }
                }
                div { class: "gallery-bottom",
                    ProjectTile {
                        label: "Private Pool",
                        img: "https://images.unsplash.com/photo-1762811054950-b74e0a055c80?crop=entropy&cs=tinysrgb&fit=max&fm=jpg&q=80&w=1080",
                    }
                    ProjectTile {
                        label: "Modern Estate",
                        img: "https://images.unsplash.com/photo-1505843795480-5cfb3c03f6ff?crop=entropy&cs=tinysrgb&fit=max&fm=jpg&q=80&w=1080",
                    }
                    ProjectTile {
                        label: "Waterfront Home",
                        img: "https://images.unsplash.com/photo-1566704437860-497902cb0c37?crop=entropy&cs=tinysrgb&fit=max&fm=jpg&q=80&w=1080",
                    }
                }
            }
        }
    }
}

// ---------- Services (MMmfm) ----------
#[component]
fn ServicesSection() -> Element {
    let services = [
        ("hammer", "Construction", "Expertly crafting durable and stunning outdoor spaces built to last.", "construction"),
        ("fence", "Fencing", "Premium fencing solutions combining security, privacy, and elegance.", "fencing"),
        ("lightbulb", "Landscape Lighting", "Transform your property after dark with warm, layered lighting.", "landscape-lighting"),
        ("droplets", "Irrigation & Maintenance", "Keep your outdoor space pristine and healthy all year round.", "irrigation-maintenance"),
        ("truck", "Civil & Trucking", "Excavation, earthmoving, and site prep handled in-house.", "civil-trucking"),
    ];
    rsx! {
        section { id: "services", class: "home-section home-section--bg",
            div { class: "wrap",
                div { class: "sec-head",
                    div { class: "sec-head__left",
                        span { class: "sec-eyebrow", "WHAT WE DO" }
                        h2 { class: "sec-title", "Full-service care for every corner of your property" }
                    }
                    p { class: "sec-note", "From the ground up to the finishing touch — one trusted team for it all." }
                }
                div { class: "service-grid",
                    for (icon , title , desc , slug) in services {
                        ServiceCard {
                            key: "{slug}",
                            icon: icon.to_string(),
                            title: title.to_string(),
                            desc: desc.to_string(),
                            slug: slug.to_string(),
                        }
                    }
                    // CTA-карточка вместо 6-й услуги
                    div { class: "service-cta",
                        div {
                            h3 { class: "service-cta__title", "Not sure where to start?" }
                            p { class: "service-cta__desc",
                                "Tell us about your yard and we'll map out a plan that fits your space and budget."
                            }
                        }
                        ScrollLink { target: "contact", class: "btn service-cta__btn",
                            "Book a consult"
                            Icon { name: "arrow-right".to_string(), size: 16 }
                        }
                    }
                }
            }
        }
    }
}

// ---------- Why Us (C5Ccj) ----------
#[component]
fn WhyUs() -> Element {
    let values = [
        ("leaf", "Sustainability", "Eco-conscious materials and water-wise design."),
        ("heart-handshake", "Customer Care", "Clear communication from first call to final walkthrough."),
        ("ruler", "Precision", "Meticulous craft down to the smallest detail."),
        ("shield-check", "Fully Insured", "Licensed, bonded, and insured for your peace of mind."),
    ];
    let stats = [
        ("10+", "Years of experience"),
        ("2,000+", "Projects completed"),
        ("100%", "Client satisfaction"),
        ("10", "Dedicated team members"),
    ];
    rsx! {
        section { id: "why", class: "home-section home-section--forest",
            div { class: "wrap",
                div { class: "sec-head",
                    div { class: "sec-head__left",
                        span { class: "sec-eyebrow sec-eyebrow--soft", "WHY SKILLFUL HANDS" }
                        h2 { class: "sec-title sec-title--light", "Built on craft, care, and a decade of trust" }
                    }
                    p { class: "sec-note sec-note--light", "We treat every yard like our own — and stand behind the result." }
                }
                div { class: "values",
                    for (icon , title , desc) in values {
                        div { key: "{title}", class: "value-card",
                            span { class: "value-card__icon", Icon { name: icon.to_string(), size: 24 } }
                            h3 { class: "value-card__title", "{title}" }
                            p { class: "value-card__desc", "{desc}" }
                        }
                    }
                }
                div { class: "stats",
                    for (num , label) in stats {
                        div { key: "{label}", class: "stat",
                            div { class: "stat__num", "{num}" }
                            div { class: "stat__label", "{label}" }
                        }
                    }
                }
            }
        }
    }
}

// ---------- Owners (ukTs8) ----------
#[component]
fn OwnerCard(name: String, role: String, bio: String, img: String) -> Element {
    rsx! {
        div { class: "owner-card",
            div { class: "owner-card__photo", style: "background-image:url('{img}')" }
            div { class: "owner-card__body",
                h3 { class: "owner-card__name", "{name}" }
                div { class: "owner-card__role", "{role}" }
                p { class: "owner-card__bio", "{bio}" }
            }
        }
    }
}

#[component]
fn Owners() -> Element {
    rsx! {
        section { id: "about", class: "home-section home-section--surface",
            div { class: "wrap",
                div { class: "sec-head",
                    div { class: "sec-head__left",
                        span { class: "sec-eyebrow", "MEET THE TEAM" }
                        h2 { class: "sec-title", "Meet the owners of Skillful Hands" }
                    }
                    p { class: "sec-note", "Two friends, one shovel each, and a shared obsession with doing the job right." }
                }
                div { class: "owners",
                    OwnerCard {
                        name: "Jake",
                        role: "Co-Founder · Operations",
                        bio: "Jake leads every build on the ground — from the first stake in the soil to the final cleanup. He's the reason projects finish on time and to spec.",
                        img: "https://images.unsplash.com/photo-1602752709993-9ab17ac8cf0d?crop=entropy&cs=tinysrgb&fit=max&fm=jpg&q=80&w=1080",
                    }
                    OwnerCard {
                        name: "Rick",
                        role: "Co-Founder · Design",
                        bio: "Rick translates your vision into a plan that works for your space, your lifestyle, and your budget — then sweats every detail until it's perfect.",
                        img: "https://images.unsplash.com/photo-1731341711390-a721b4e31b6a?crop=entropy&cs=tinysrgb&fit=max&fm=jpg&q=80&w=1080",
                    }
                }
            }
        }
    }
}

// ---------- Testimonials (yGEU9) ----------
#[component]
fn Testimonials() -> Element {
    let reviews = [
        ("James R.", "Kelowna, BC", "The team transformed our backyard beyond what we imagined. Professional, punctual, and the quality is outstanding."),
        ("Megan T.", "West Kelowna, BC", "From irrigation to fencing, everything was done right the first time. We finally use our yard every single day."),
        ("David & Lin", "Vancouver Island, BC", "Honest pricing and incredible attention to detail. Skillful Hands is the only crew we'll call from now on."),
    ];
    rsx! {
        section { id: "testimonials", class: "home-section home-section--bg",
            div { class: "wrap",
                div { class: "sec-head",
                    div { class: "sec-head__left",
                        span { class: "sec-eyebrow", "WHAT CLIENTS SAY" }
                        h2 { class: "sec-title", "Loved by homeowners across the valley" }
                    }
                    div { class: "google-badge",
                        Icon { name: "star".to_string(), size: 18 }
                        span { "5.0 on Google Reviews" }
                    }
                }
                div { class: "reviews",
                    for (name , role , quote) in reviews {
                        ReviewCard {
                            key: "{name}",
                            name: name.to_string(),
                            role: role.to_string(),
                            quote: quote.to_string(),
                        }
                    }
                }
            }
        }
    }
}

// ---------- CTA Band (GeAR4) ----------
#[component]
fn CtaBand() -> Element {
    rsx! {
        section { class: "cta-band",
            div { class: "cta-band__inner",
                span { class: "sec-eyebrow sec-eyebrow--soft", "LET'S BUILD SOMETHING GREAT" }
                h2 { class: "cta-band__headline", "Ready to fall in love with your yard?" }
                p { class: "cta-band__sub",
                    "Book a free on-site consultation and get a clear, no-pressure plan for your outdoor space."
                }
                div { class: "cta-band__btns",
                    ScrollLink { target: "contact", class: "btn btn--light", "Get a free quote" }
                    a { href: "tel:+17782396704", class: "btn btn--outline",
                        "Call (778) 239 6704"
                    }
                }
            }
        }
    }
}
