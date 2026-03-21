pub const MAX_FITNESS: u8 = 80;

#[derive(Default, Clone, PartialEq)]
pub struct Athlete {
    pub name: String,
    pub fatigue: i8,
    pub fitness: u8,
    pub pr: u16,
}

impl Athlete {
    pub fn pr(&self) -> String {
        let hours = self.pr / 3600;
        let minutes = (self.pr % 3600) / 60;
        let seconds = self.pr % 60;

        format!("{hours}:{minutes:02}:{seconds:02}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_format_pr_into_min_seconds() {
        let athlete = Athlete {
            pr: 5565,
            ..Default::default()
        };

        assert_eq!(athlete.pr(), "1:32:45");
    }
}
