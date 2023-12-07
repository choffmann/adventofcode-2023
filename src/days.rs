use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug)]
pub struct Day(u8);

impl Day {
    pub fn new(day: u8) -> Option<Self> {
        return if day >= 1 && day <= 24 {
            Some(Self(day))
        } else {
            None
        }
    }
}

impl Display for Day {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}", self.0)
    }
}

impl FromStr for Day {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with("day") {
            Err(anyhow::anyhow!("No valid string"))
        } else {
            let day= s.strip_prefix("day").unwrap().parse::<u8>();
            Ok(Self::new(day.unwrap()).expect("Expecting day between 1 and 24"))
        }
    }
}