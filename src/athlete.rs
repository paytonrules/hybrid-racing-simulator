pub const MAX_FITNESS: u8 = 80;
const STARTING_HOURS: u8 = 5; // The 'crossfit' athlete goes to class 4-5 times a week. 
const FITNESS_FUNCTION_CONSTANT: u8 = 1;

#[derive(Default, Clone, PartialEq)]
pub struct Athlete {
    pub name: String,
    pub fatigue: u8,
    pub fitness: u8,
    pub races: Vec<u16>,
}

impl Athlete {
    pub fn train(&self, time: u8) -> Athlete {
        let mut trained_athlete = self.clone();

        // Running into weirdness on u8/i16 and I'm being dumb about it.
        // Training is clamped from 0-80 (so u8 is fine)
        // Time - old time
        // Can be negative, so i16 (i8 is too small)
        let diff: i16 = i16::from(time) - i16::from(STARTING_HOURS);

        // Range is really 255 -255
        let diff_squared: u16 = diff.unsigned_abs() * diff.unsigned_abs();
        let adjustment: i32 = (diff_squared + FITNESS_FUNCTION_CONSTANT as u16) as i32;
        println!("adjustment: {adjustment}");
        if diff < 0 {
            trained_athlete.fitness -= adjustment as u8;
        } else {
            trained_athlete.fitness += adjustment as u8;
        }

        trained_athlete
    }

    pub fn pr(&self) -> String {
        let pr = self.races.iter().min().unwrap_or_else(|| &0);
        let hours = pr / 3600;
        let minutes = (pr % 3600) / 60;
        let seconds = pr % 60;

        format!("{hours}:{minutes:02}:{seconds:02}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_format_pr_into_min_seconds() {
        let athlete = Athlete {
            races: vec![5565],
            ..Default::default()
        };

        assert_eq!(athlete.pr(), "1:32:45");
    }

    #[test]
    fn it_stores_all_race_results() {
        let mut athlete = Athlete::default();

        assert_eq!(athlete.pr(), "0:00:00");

        athlete.log_race(5565);
        assert_eq!(athlete.pr(), "1:32:45");

        athlete.log_race(5565 + 1);
        assert_eq!(athlete.pr(), "1:32:45");
    }

    #[test]
    fn training_session_increases_fitness_if_the_same_as_previous_week() {
        let athlete = Athlete {
            fitness: 50,
            ..Default::default()
        };

        let new_athlete = athlete.train(STARTING_HOURS);

        // Fitness function constant is what you increase with no change in training
        assert_eq!(new_athlete.fitness, 50 + FITNESS_FUNCTION_CONSTANT);
    }

    #[test]
    fn training_session_increases_more_with_an_increase_in_hours() {
        let athlete = Athlete {
            fitness: 50,
            ..Default::default()
        };

        let new_athlete = athlete.train(STARTING_HOURS + 1);

        // Fitness function const + load increase (the diff)
        assert_eq!(new_athlete.fitness, 50 + FITNESS_FUNCTION_CONSTANT + 1);
    }

    #[test]
    fn increase_in_training_load_has_an_exponential_increase_in_fitness() {
        let athlete = Athlete {
            fitness: 50,
            ..Default::default()
        };

        let new_athlete = athlete.train(STARTING_HOURS + 3);

        // Load increase is exponential (3 squared)
        assert_eq!(new_athlete.fitness, 50 + FITNESS_FUNCTION_CONSTANT + 9);
    }

    #[test]
    fn decrease_in_training_load_has_an_exponsential_decrease_in_fitness() {
        let athlete = Athlete {
            fitness: 50,
            ..Default::default()
        };

        assert!(STARTING_HOURS > 3); // Make sure this test won't go kerblooie work
        let new_athlete = athlete.train(STARTING_HOURS - 3);

        // Load increase is exponential (3 squared)
        assert_eq!(new_athlete.fitness, 50 - FITNESS_FUNCTION_CONSTANT - 9);
    }
}
