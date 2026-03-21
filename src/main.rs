mod athlete;
mod athlete_draft;

use athlete::{Athlete, MAX_FITNESS};
use dioxus::prelude::*;

// WR as of 3/7
// Alex: 53:15 - 3195 seconds
// Joanna: 56:03 - 3363 seconds
const MALE_WORLD_RECORD: u16 = 3180;
const MALE_SLOWEST_POSSIBLE_TIME: u16 = MALE_WORLD_RECORD * 3;
const MALE_RANGE: u16 = MALE_SLOWEST_POSSIBLE_TIME - MALE_WORLD_RECORD;

fn race(athlete: &Athlete) -> u16 {
    let fitness = u32::from(athlete.fitness);
    let slow_time = u32::from(MALE_SLOWEST_POSSIBLE_TIME);
    let range = u32::from(MALE_RANGE);
    let max_fitness = u32::from(MAX_FITNESS);

    (slow_time - ((fitness * range) / max_fitness)) as u16
}

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
fn AthleteComponent(mut athlete: Signal<Option<Athlete>>) -> Element {
    rsx! {
        div { class: "left nes-container with-title",
            p { class: "title", "Your Athlete" }
            if let Some(created_athlete) = athlete.read().as_ref() {
                AthleteStats { athlete: created_athlete.clone() }
            } else {
                AthleteCreator { athlete: athlete }
            }
        }
    }
}

#[component]
fn AthleteCreator(mut athlete: Signal<Option<Athlete>>) -> Element {
    let mut draft = use_signal(|| athlete_draft::AthleteDraft::default());
    let nes_btn_class = if Athlete::try_from(draft.read().clone()).is_ok() {
        "nes-btn"
    } else {
        "nes-btn is-disabled"
    };

    rsx! {
        form {
            onsubmit: move |event| {
                event.prevent_default();
                athlete.set(Athlete::try_from(draft.read().clone()).ok());
            },

            div { class: "nes-field",
                label { for: "name", "Name:" }
                input {
                    id: "name",
                    r#type: "text",
                    class: "nes-input",
                    placeholder: "Lauren Weeks",
                    oninput: move |e| draft.write().name = e.value()
                }
            }
            div { class: "nes-field",
                label { for: "fitness", "Fitness (0-80):" }
                input {
                    id: "fitness",
                    r#type: "number",
                    class: "nes-input",
                    placeholder: "60",
                    min: 0,
                    max: 80,
                    oninput: move |e| draft.write().fitness = e.value()
                }
            }
            input {
                class: nes_btn_class,
                r#type: "submit",
                value: "Save"
            }
        }
    }
}

#[component]
fn AthleteStats(athlete: Athlete) -> Element {
    rsx! {
        p { id: "name", "Name: {athlete.name}"}
        p { id: "fitness", "Fitness: {athlete.fitness}"}
        p { class: "nes-text is-secondary", "Fatigue: {athlete.fatigue}" }
        p { class: "nes-text is-primary", "PR: {athlete.pr}" }
    }
}

#[component]
fn Training(mut athlete: Signal<Option<Athlete>>) -> Element {
    let modifier = if athlete.read().is_none() {
        "is-disabled"
    } else {
        ""
    };

    rsx! {
        div { class: "right nes-container with-title",
            p { class: "title", "This Week" }
            div { class: "nes-field",
                label { for: "hours", "Hours:" }
                input { id: "hours", r#type: "number", class: "nes-input", min: "0", max: "24", placeholder: "8" }
            }
            button {
                class: format_args!("nes-btn {modifier}"),
                "Train"
            }
            button {
                class: format_args!("nes-btn {modifier}"),
                onclick: move |_evt| {
                    athlete.write().as_mut().map(|athlete| athlete.pr = race(athlete));
                },
                "Race!"
            }
        }
    }
}

#[component]
fn Home() -> Element {
    let athlete: Signal<Option<Athlete>> = use_signal(|| None);

    rsx! {
        document::Stylesheet { href: CSS }
        div { class: "container",
            header { "Hybrid Race Simulator" }
            AthleteComponent { athlete: athlete }
            Training { athlete: athlete }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_race_wr_time_if_fitness_is_maxed_and_fatigue_is_zero() {
        let athlete = Athlete {
            fitness: MAX_FITNESS,
            ..Default::default()
        };

        let result = race(&athlete);

        assert_eq!(result, MALE_WORLD_RECORD);
    }

    #[test]
    fn it_should_take_slowest_time_if_fitness_is_min_and_fatigue_is_zero() {
        let athlete = Athlete::default();

        let result = race(&athlete);

        assert_eq!(result, MALE_SLOWEST_POSSIBLE_TIME);
    }

    #[test]
    fn it_should_take_the_halfway_point_if_fitness_is_forty_and_fatigue_is_zero() {
        let athlete = Athlete {
            fitness: MAX_FITNESS / 2,
            ..Default::default()
        };

        let result = race(&athlete);

        assert_eq!(result, (MALE_SLOWEST_POSSIBLE_TIME + MALE_WORLD_RECORD) / 2);
    }
}
