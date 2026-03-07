use dioxus::prelude::*;

#[derive(Default)]
struct Athlete {
    fatigue: i8,
    fitness: u8,
}

const MAX_FITNESS: u8 = 80;
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
fn Home() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        div { class: "container",
            header { "Hybrid Race Simulator" }
            div { class: "left nes-container with-title",
                p { class: "title", "Your Athlete" }
                div { class: "nes-field",
                    label { for: "name", "Name:" }
                    input { id: "name", r#type: "text", class: "nes-input", placeholder: "Lauren Weeks" }
                }
                div { class: "nes-field",
                    label { for: "fitness", "Fitness (0-80):" }
                    input { id: "fitness", r#type: "number", class: "nes-input", placeholder: "60", min: 0, max: 80 }
                }
                p { class: "nes-text is-secondary", "Fatigue: 0" }
                p { class: "nes-text is-primary", "PR: none" }
                button { class: "nes-btn", "Save" }

            }
            div { class: "right nes-container with-title",
                p { class: "title", "Your  training this week" }
                div { class: "nes-field",
                    label { for: "hours", "Hours:" }
                    input { id: "hours", r#type: "number", class: "nes-input", min: "0", max: "24", placeholder: "8" }
                }

                button { class: "nes-btn", "Train" }
                button { class: "nes-btn", "Race!" }
            }
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
