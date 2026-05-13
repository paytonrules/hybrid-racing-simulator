use crate::athlete::{Athlete, FITNESS_FUNCTION_CONSTANT, STARTING_HOURS};

#[derive(Default)]
pub struct Game {
    pub athlete: Athlete,
}

impl Game {
    pub fn new(athlete: Athlete) -> Self {
        Game { athlete }
    }

    pub fn race_week(&mut self, training: u8) {
        self.athlete = self.athlete.train(training).race();
    }

    pub fn fatigue(&self) -> u8 {
        self.athlete.fatigue
    }

    pub fn fitness(&self) -> u8 {
        self.athlete.fitness
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::race::RACE_FATIGUE;

    // This does require knowing the internals of the athlete algorithm, and will make them
    // sensitive to it.
    // You can eventually solve this with a trait for a 'type' of athlete.
    #[test]
    fn race_runs_training_then_races() {
        let athlete = Athlete {
            fatigue: 50,
            fitness: 50,
            ..Default::default()
        };
        let mut game = Game::new(athlete);

        // 0 hours of training change
        game.race_week(STARTING_HOURS);

        assert_eq!(game.fatigue(), 50 + RACE_FATIGUE);
        assert_eq!(game.fitness(), 50 + FITNESS_FUNCTION_CONSTANT)
    }
}
