use dioxus::prelude::*;

use crate::Route;

/// Плавный переход к секции-якорю без перезагрузки страницы.
/// Секция есть на текущей странице → smooth scroll; нет → SPA-переход на Home и скролл.
#[component]
pub fn ScrollLink(
    target: String,
    #[props(default)] class: String,
    #[props(default)] onnav: EventHandler<MouseEvent>,
    children: Element,
) -> Element {
    let route = use_route::<Route>();
    let nav = use_navigator();
    // #contact есть на Home/Services/ServiceDetail; projects/about/testimonials — только на Home.
    let here = match target.as_str() {
        "contact" => matches!(
            route,
            Route::Home {} | Route::Services {} | Route::ServiceDetail { .. }
        ),
        _ => matches!(route, Route::Home {}),
    };
    let href = format!("/#{target}");
    let id = target;

    rsx! {
        a {
            class: "{class}",
            href: "{href}",
            onclick: move |e| {
                e.prevent_default();
                onnav.call(e);
                let id = id.clone();
                if here {
                    spawn(async move {
                        let _ = document::eval(&format!(
                            "document.getElementById('{id}')?.scrollIntoView({{behavior:'smooth',block:'start'}});"
                        )).await;
                    });
                } else {
                    nav.push(Route::Home {});
                    spawn(async move {
                        let _ = document::eval(&format!(
                            "setTimeout(function(){{var el=document.getElementById('{id}');if(el)el.scrollIntoView({{behavior:'smooth',block:'start'}});}},80);"
                        )).await;
                    });
                }
            },
            {children}
        }
    }
}
