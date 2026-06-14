use dioxus::prelude::*;

/// Эмблема Skillful Hands — пила + 3 дома на диске. Геометрия 1:1 из Pencil (vyYgx).
#[component]
pub fn LogoMark(#[props(default = 46)] size: i32) -> Element {
    rsx! {
        svg {
            class: "logo__mark",
            width: "{size}",
            height: "{size}",
            view_box: "0 0 48 48",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            // Пильный диск (зубья)
            path {
                d: "M24 0.5l2.7 4.69 3.92-3.74 1.27 5.27 4.82-2.49-0.27 5.41 5.32-1.03-1.78 5.12 5.4 0.51-3.15 4.41 5.03 2.01-4.26 3.34 4.26 3.34-5.03 2.01 3.15 4.41-5.4 0.51 1.78 5.12-5.32-1.03 0.27 5.41-4.82-2.49-1.27 5.27-3.92-3.74-2.7 4.69-2.7-4.69-3.92 3.74-1.27-5.27-4.82 2.49 0.27-5.41-5.32 1.03 1.78-5.12-5.4-0.51 3.15-4.41-5.03-2.01 4.26-3.34-4.26-3.34 5.03-2.01-3.15-4.41 5.4-0.51-1.78-5.12 5.32 1.03-0.27-5.41 4.82 2.49 1.27-5.27 3.92 3.74z",
                fill: "var(--leaf)",
            }
            // Тёмный диск
            ellipse { cx: "24", cy: "24", rx: "17.5", ry: "17.5", fill: "var(--forest-deep)", stroke: "var(--leaf)", stroke_width: "1.5" }
            // Три крыши
            path { transform: "translate(15 15)", d: "M0 7.14l9-7.14 9 7.14 0 9.86-18 0z", fill: "var(--leaf)" }
            path { transform: "translate(5 20.5)", d: "M0 5.25l7.25-5.25 7.25 5.25 0 7.25-14.5 0z", fill: "var(--leaf)" }
            path { transform: "translate(28.5 20.5)", d: "M0 5.25l7.25-5.25 7.25 5.25 0 7.25-14.5 0z", fill: "var(--leaf)" }
            // Окна
            rect { x: "21.6", y: "22.2", width: "4.8", height: "5.2", rx: "0.4", fill: "var(--forest-deep)" }
            rect { x: "10.3", y: "26", width: "3.2", height: "3.8", rx: "0.4", fill: "var(--forest-deep)" }
            rect { x: "33.8", y: "26", width: "3.2", height: "3.8", rx: "0.4", fill: "var(--forest-deep)" }
        }
    }
}

/// Лого + вордмарк (эмблема + «Skillful Hands / SOLUTIONS LTD.»). Используется в Header и Footer.
#[component]
pub fn Logo() -> Element {
    rsx! {
        span { class: "logo",
            LogoMark {}
            span { class: "logo__word",
                span { class: "logo__name", "Skillful Hands" }
                span { class: "logo__sub", "SOLUTIONS LTD." }
            }
        }
    }
}
