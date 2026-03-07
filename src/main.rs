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
            div { class: "left nes-container with-title",
                p { class: "title", "Your Athlete" }
                div { class: "nes-field",
                    label { for: "name", "Name:" }
                    input { id: "name", type: "text", class: "nes-input", placeholder: "Lauren Weeks" }
                }
                div { class: "nes-field",
                    label { for: "fitness", "Fitness:" }
                    input { id: "fitness", type: "numeric", class: "nes-input", placeholder: "From 0-80" }
                }
                p { class: "nes-text is-primary", "Fatigue: 0" }
                p { class: "nes-text is-secondary", "PR: none" }
                button { class: "nes-btn", "Save" }

            }
            div { class: "right nes-container with-title",
                p { class: "title", "Your  training this week" }
                div { class: "nes-field",
                    label { for: "hours", "Hours:" }
                    input { id: "hours", type: "numeric", class: "nes-input", placeholder: "8" }
                }

                button { class: "nes-btn", "Train" }
                button { class: "nes-btn", "Race!" }
            }
        }
    }
}
