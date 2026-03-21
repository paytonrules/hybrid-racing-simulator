use crate::athlete::{Athlete, MAX_FITNESS};

// WR as of 3/7
// Alex: 53:15 - 3195 seconds
// Joanna: 56:03 - 3363 seconds
const MALE_WORLD_RECORD: u16 = 3180;
const MALE_SLOWEST_POSSIBLE_TIME: u16 = MALE_WORLD_RECORD * 3;
const MALE_RANGE: u16 = MALE_SLOWEST_POSSIBLE_TIME - MALE_WORLD_RECORD;

pub fn race(athlete: &Athlete) -> u16 {
    let fitness = u32::from(athlete.fitness);
    let slow_time = u32::from(MALE_SLOWEST_POSSIBLE_TIME);
    let range = u32::from(MALE_RANGE);
    let max_fitness = u32::from(MAX_FITNESS);

    (slow_time - ((fitness * range) / max_fitness)) as u16
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
