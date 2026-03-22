use crate::athlete::{Athlete, MAX_FITNESS};

// WR as of 3/7
// Alex: 53:15 - 3195 seconds
// Joanna: 56:03 - 3363 seconds
const MALE_WORLD_RECORD: u16 = 3180;
const MALE_SLOWEST_POSSIBLE_TIME: u16 = MALE_WORLD_RECORD * 3;
const MALE_RANGE: u16 = MALE_SLOWEST_POSSIBLE_TIME - MALE_WORLD_RECORD;
const RACE_FATIGUE: u8 = 10;

pub fn race(athlete: &Athlete) -> Athlete {
    let mut raced_athlete = athlete.clone();
    let fitness = u32::from(athlete.fitness);
    let slow_time = u32::from(MALE_SLOWEST_POSSIBLE_TIME);
    let range = u32::from(MALE_RANGE);
    let max_fitness = u32::from(MAX_FITNESS);

    let race_time = (slow_time - ((fitness * range) / max_fitness)) as u16;

    raced_athlete.races.push(race_time);
    raced_athlete.fatigue += RACE_FATIGUE;

    raced_athlete
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_race_wr_time_if_fitness_is_maxed_and_fatigue_is_zero() -> Result<(), &'static str>
    {
        let athlete = Athlete {
            fitness: MAX_FITNESS,
            ..Default::default()
        };

        let new_athlete = race(&athlete);

        assert_eq!(
            *new_athlete.races.last().ok_or("No Races Stored")?,
            MALE_WORLD_RECORD
        );

        Ok(())
    }

    #[test]
    fn it_should_take_slowest_time_if_fitness_is_min_and_fatigue_is_zero()
    -> Result<(), &'static str> {
        let athlete = Athlete::default();

        let new_athlete = race(&athlete);

        assert_eq!(
            *new_athlete.races.last().ok_or("No Races Stored")?,
            MALE_SLOWEST_POSSIBLE_TIME
        );

        Ok(())
    }

    #[test]
    fn it_should_take_the_halfway_point_if_fitness_is_forty_and_fatigue_is_zero()
    -> Result<(), &'static str> {
        let athlete = Athlete {
            fitness: MAX_FITNESS / 2,
            ..Default::default()
        };

        let new_athlete = race(&athlete);

        assert_eq!(
            *new_athlete.races.last().ok_or("No Races Stored")?,
            (MALE_SLOWEST_POSSIBLE_TIME + MALE_WORLD_RECORD) / 2
        );

        Ok(())
    }

    #[test]
    fn racing_should_increase_fatigue() {
        let athlete = Athlete {
            fatigue: 3,
            ..Default::default()
        };

        let new_athlete = race(&athlete);

        assert_eq!(new_athlete.fatigue, RACE_FATIGUE + 3);
    }
}
