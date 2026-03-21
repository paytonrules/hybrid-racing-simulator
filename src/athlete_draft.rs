use crate::athlete::{Athlete, MAX_FITNESS};

#[derive(Clone, Default)]
pub struct AthleteDraft {
    pub name: String,
    pub fitness: String,
}

impl TryFrom<AthleteDraft> for Athlete {
    type Error = String;

    fn try_from(value: AthleteDraft) -> Result<Self, Self::Error> {
        if value.name.trim().is_empty() {
            return Err("Athlete name cannot be empty".into());
        }

        let fitness: u8 = value
            .fitness
            .parse()
            .map_err(|_| "Fitness must be a number")?;

        if fitness > MAX_FITNESS {
            return Err(format!("Fitness cannot be greater than {MAX_FITNESS}"));
        }

        Ok(Athlete {
            name: value.name,
            fitness: 50,
            ..Default::default()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // For now fitness will always be 50, this will convert to archetypes
    // But only once I get a fun game loop going.

    #[test]
    fn it_converts_from_athlete_draft_to_athlete() -> Result<(), String> {
        let draft = AthleteDraft {
            name: "Eric Smith".into(),
            fitness: "80".into(),
        };

        let athlete: Athlete = draft.try_into()?;

        assert_eq!(athlete.name, String::from("Eric Smith"));
        assert_eq!(athlete.fitness, 50);
        assert_eq!(athlete.fatigue, 0);

        Ok(())
    }

    #[test]
    fn it_errors_when_the_fitness_is_not_parsable() {
        let draft = AthleteDraft {
            name: "Eric Smith".into(),
            fitness: "abc".into(),
        };

        let athlete: Result<Athlete, String> = draft.try_into();

        assert!(athlete.is_err());
    }

    #[test]
    fn it_errors_when_fitness_is_negative() {
        let draft = AthleteDraft {
            name: "Eric Smith".into(),
            fitness: "-1".into(),
        };

        let athlete: Result<Athlete, String> = draft.try_into();

        assert!(athlete.is_err());
    }

    #[test]
    fn it_errors_when_fitness_is_above_max() {
        let draft = AthleteDraft {
            name: "Eric Smith".into(),
            fitness: format!("{}", MAX_FITNESS + 1).into(),
        };

        let athlete: Result<Athlete, String> = draft.try_into();

        assert!(athlete.is_err());
    }

    #[test]
    fn it_errors_when_athlete_name_is_blank() {
        let draft = AthleteDraft {
            name: "    ".into(),
            fitness: "10".into(),
        };

        let athlete: Result<Athlete, String> = draft.try_into();

        assert!(athlete.is_err());
    }
}
