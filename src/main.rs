mod app;
mod components;
mod pages;

pub use app::Route;

fn main() {
    dioxus::launch(app::App);
}
