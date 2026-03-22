pub const MAX_FITNESS: u8 = 80;

#[derive(Default, Clone, PartialEq)]
pub struct Athlete {
    pub name: String,
    pub fatigue: i8,
    pub fitness: u8,
    pub races: Vec<u16>,
}

impl Athlete {
    pub fn pr(&self) -> String {
        let pr = self.races.iter().min().unwrap_or_else(|| &0);
        let hours = pr / 3600;
        let minutes = (pr % 3600) / 60;
        let seconds = pr % 60;

        format!("{hours}:{minutes:02}:{seconds:02}")
    }

    pub fn log_race(&mut self, time: u16) {
        self.races.push(time);
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
}
