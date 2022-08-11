use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

use crate::date::Date;
use crate::national_day::NationalDay;
use crate::parse_error::ParseError;

/// Read a line and convert to NationalDay type
///
/// # Arguments
///
/// * `line` string on the form country, date, extra info;
fn read_national_day_from_line(line: String) -> Result<NationalDay, ParseError> {
    let tokens: Vec<&str> = line.splitn(3, ",").collect();
    if tokens.len() != 3 {
        return Err(ParseError::IncorrectNumberOfTokensError);
    }
    let country = tokens[0];
    let raw_date = tokens[1];
    let extra_info = tokens[2].replace('"', "");
    let date = Date::from_str(raw_date)?;
    let national_day = NationalDay {
        country: country.to_string(),
        date,
        extra_info: extra_info.to_string(),
    };
    Ok(national_day)
}

pub fn read_country_list_to_date_hash_map(file_path: &Path) -> HashMap<Date, Vec<NationalDay>> {
    let mut countries: HashMap<Date, Vec<NationalDay>> = HashMap::new();
    let file = File::open(file_path).expect("Couldn't open national days file");
    let buf_reader = BufReader::new(file);
    for line in buf_reader.lines() {
        let line = line.expect("Cannot read line from file");
        let national_day_result = read_national_day_from_line(line.clone());
        match national_day_result {
            Ok(national_day) => {
                let countries_for_date = countries.entry(national_day.date).or_insert(Vec::new());
                countries_for_date.push(national_day);
            }
            Err(e) => println!("Could not read line {}, error {}", line, e),
        }
    }
    countries
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::date::Date;

    #[test]
    fn test_read_correct_national_day_from_line() {
        let line = String::from("Norway,17/05,Constitution day");
        let national_day = read_national_day_from_line(line);
        let expected_date = Date { day: 17, month: 5 };
        match national_day {
            Ok(national_day) => {
                assert_eq!(national_day.country, String::from("Norway"));
                assert_eq!(national_day.date, expected_date);
                assert_eq!(national_day.extra_info, String::from("Constitution day"));
            }
            Err(_) => panic!("Function returned error when it should not"),
        }
    }
}
