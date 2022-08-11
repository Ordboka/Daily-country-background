use chrono::{Datelike, TimeZone, Utc};
use std::{fmt, str::FromStr};

use crate::parse_error::ParseError;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct Date {
    pub day: u32,
    pub month: u32,
}

impl fmt::Display for Date {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let date_time = chrono::Local.ymd(2022, self.month, self.day);
        fmt.write_str(&date_time.format("%B %e").to_string())?;
        let today = Utc::now();
        if today.day() == self.day && today.month() == self.month {
            fmt.write_str(" (today)")?;
        }
        Ok(())
    }
}

impl FromStr for Date {
    type Err = ParseError;

    ///Creates a date from a string on the format dd/mm
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let date_tokens: Vec<&str> = s.split("/").collect();
        if date_tokens.len() != 2 {
            return Err(ParseError::IncorrectNumberOfTokensError);
        }
        let day = date_tokens[0].parse::<u32>()?;
        let month = date_tokens[1].parse::<u32>()?;
        let date = Date { day, month };
        Ok(date)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_date_from_string_success() {
        let date_result = Date::from_str("26/10");
        match date_result {
            Ok(date) => {
                assert_eq!(date.day, 26);
                assert_eq!(date.month, 10);
            }
            Err(_) => panic!("Could not parse date from string when it should be possible"),
        }
    }

    #[test]
    fn test_parse_date_from_string_success_zero_padded() {
        let date_result = Date::from_str("01/02");
        match date_result {
            Ok(date) => {
                assert_eq!(date.day, 1);
                assert_eq!(date.month, 2);
            }
            Err(_) => panic!("Could not parse date from string when it should be possible"),
        }
    }
}
