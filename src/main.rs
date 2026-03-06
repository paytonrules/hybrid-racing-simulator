use dioxus::prelude::*;

static CSS: Asset = asset!("/assets/main.css");

#[derive(Routable, Clone)]
enum Route {
    #[route("/")]
    Home {},
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        div { class: "container",
            header { "Hybrid Race Simulator" }
            div { class: "left nes-container",
                p { "left side" }
            }
            div { class: "right nes-container",
                div { "The money" }
            }
        }
    }
}
