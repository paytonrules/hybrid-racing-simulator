use dioxus::prelude::*;

#[derive(Routable, Clone)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/teams")]
    Teams {},
    #[route("/schedule")]
    Schedule {},
    #[route("/settings")]
    Settings {},
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
        div { class: "page",
            h1 { "Sports League" }
            p { "Welcome to the game" }
            Navigation {}
        }
    }
}

#[component]
fn Teams() -> Element {
    rsx! {
        div { class: "page",
            h1 { "Teams" }
            ul {
                li { "Team A" }
                li { "Team B" }
                li { "Team C" }
            }
            Navigation {}
        }
    }
}

#[component]
fn Schedule() -> Element {
    rsx! {
        div { class: "page",
            h1 { "Schedule" }
            ul {
                li { "Game 1 - Monday" }
                li { "Game 2 - Wednesday" }
                li { "Game 3 - Friday" }
            }
            Navigation {}
        }
    }
}

#[component]
fn Settings() -> Element {
    rsx! {
        div { class: "page",
            h1 { "Settings" }
            p { "Coming soon..." }
            Navigation {}
        }
    }
}

#[component]
fn Navigation() -> Element {
    rsx! {
        nav { class: "nav",
            Link { to: Route::Home {}, "Home" }
            Link { to: Route::Teams {}, "Teams" }
            Link { to: Route::Schedule {}, "Schedule" }
            Link { to: Route::Settings {}, "Settings" }
        }
    }
}
