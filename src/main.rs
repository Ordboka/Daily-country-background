mod date;
mod edit_image;
mod national_day;
mod parse_error;
mod parse_file;

use national_day::NationalDay;
use std::collections::HashMap;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;

use rand::prelude::SliceRandom;
use rand::Rng;

use chrono::{Datelike, Utc};

use date::Date;

use wallpaper;

fn get_countries_for_current_day<'a>(
    country_date_hash_map: &'a HashMap<Date, Vec<NationalDay>>,
) -> Option<&'a Vec<NationalDay>> {
    let current_date = Utc::now();
    let today = Date {
        day: current_date.day(),
        month: current_date.month(),
    };
    country_date_hash_map.get(&today)
}

fn get_random_country(
    country_date_hash_map: &HashMap<Date, Vec<NationalDay>>,
) -> Option<&NationalDay> {
    let mut total_values = 0;
    for (_, countries) in country_date_hash_map {
        total_values += countries.len();
    }
    let random_choice = rand::thread_rng().gen_range(0..total_values);
    let mut seen_so_far = 0;
    for (_, countries) in country_date_hash_map {
        for country in countries {
            if seen_so_far == random_choice {
                return Some(country);
            }
            seen_so_far += 1;
        }
    }
    None
}

fn find_file_for_country(country: &NationalDay) -> Option<DirEntry> {
    let picture_dir = Path::new("pictures/");
    for entry in fs::read_dir(picture_dir).expect("Couldn't read from dir") {
        match entry {
            Ok(file) => {
                let filename = String::from(file.file_name().to_string_lossy());
                if filename
                    .split(".")
                    .next()
                    .expect("Filename did not have a .")
                    == country.country
                {
                    return Some(file);
                }
            }
            Err(error) => println!("{}", error),
        }
    }
    None
}

fn set_wallpaper_for_country(country: &NationalDay) -> Result<(), &str> {
    let wallpaper_file = find_file_for_country(country);
    match wallpaper_file {
        Some(file) => {
            let file_path = file.path();
            let file_path = fs::canonicalize(&file_path).expect("Could not canonicalize the file");
            let edited_file_path = edit_image::add_text_to_picture(country, &file_path);
            let edited_file_path =
                fs::canonicalize(&edited_file_path).expect("Could not canonicalize the file");
            let edited_file_path = edited_file_path
                .to_str()
                .expect("Couldn't convert filename to string");
            wallpaper::set_from_path(edited_file_path).expect("Couldn't set the wallpaper");
            return Ok(());
        }
        None => return Err("Couldn't find a file"),
    }
}

fn main() {
    let national_days_path = Path::new("national_days.csv");
    let country_date_hash_map = parse_file::read_country_list_to_date_hash_map(national_days_path);
    match get_countries_for_current_day(&country_date_hash_map) {
        Some(nations) => match nations.choose(&mut rand::thread_rng()) {
            Some(nation) => {
                println!("{:?}", nation);
                set_wallpaper_for_country(nation).expect("Couldn't set file for this country");
            }
            None => panic!("Date chosen has no national days"),
        },
        None => {
            let random_national_day = get_random_country(&country_date_hash_map)
                .expect("Could not find random national day");
            println!("{:?}", random_national_day);
            set_wallpaper_for_country(random_national_day)
                .expect("Couldn't set file for this country");
        }
    }
}
