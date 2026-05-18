use crate::athlete::{Athlete, FITNESS_FUNCTION_CONSTANT, STARTING_HOURS};
use anyhow::Result;

#[derive(Clone)]
pub enum Game {
    Creating,
    Training(Training),
}

#[derive(Default, Clone)]
pub struct Training {
    pub athlete: Athlete,
}

impl Game {
    pub fn new() -> Self {
        Game::Creating
    }

    pub fn start_career(self, athlete: Athlete) -> Self {
        Game::Training(Training { athlete: athlete })
    }

    pub fn train(self, hours: u8) -> Result<Self> {
        match self {
            Game::Training(training) => {
                let athlete = training.athlete.train(hours);
                Ok(Game::Training(Training { athlete }))
            }
            _ => Err(anyhow::anyhow!("invalid state to train")),
        }
    }

    pub fn race_week(self, hours: u8) -> Result<Self> {
        match self {
            Game::Training(training) => {
                let athlete = training.athlete.train(hours).race();
                Ok(Game::Training(Training { athlete }))
            }
            _ => Err(anyhow::anyhow!("invalid state to race")),
        }
    }

    pub fn fatigue(&self) -> u8 {
        match self {
            Game::Training(training) => training.athlete.fatigue,
            _ => 0,
        }
    }

    pub fn fitness(&self) -> u8 {
        match self {
            Game::Training(training) => training.athlete.fitness,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::race::RACE_FATIGUE;

    #[test]
    fn game_is_created_with_no_fatigue_or_fitness() {
        let game = Game::new();

        assert_eq!(game.fatigue(), 0);
        assert_eq!(game.fitness(), 0);
    }

    #[test]
    fn training_cannot_start_until_after_a_carrer_is_started() {
        let game = Game::new();

        assert!(game.clone().train(STARTING_HOURS).is_err());
        assert!(game.clone().race_week(STARTING_HOURS).is_err());
    }

    #[test]
    fn can_race_after_career_is_started_which_trains_and_adds_special_race_fatigue() -> Result<()> {
        let game = Game::new();
        let athlete = Athlete {
            fatigue: 50,
            fitness: 50,
            ..Default::default()
        };

        let game = game.start_career(athlete.clone());

        // 0 hours of training change
        let game = game.race_week(STARTING_HOURS)?;

        assert_eq!(game.fatigue(), 50 + RACE_FATIGUE);
        assert_eq!(game.fitness(), 50 + FITNESS_FUNCTION_CONSTANT);
        Ok(())
    }

    #[test]
    fn can_race_train_after_career_is_started_which_only_trains() -> Result<()> {
        let game = Game::new();
        let athlete = Athlete {
            fatigue: 50,
            fitness: 50,
            ..Default::default()
        };

        let game = game.start_career(athlete.clone());

        // 0 hours of training change
        let game = game.train(STARTING_HOURS)?;

        assert_eq!(game.fatigue(), 50);
        assert_eq!(game.fitness(), 50 + FITNESS_FUNCTION_CONSTANT);
        Ok(())
    }
}
