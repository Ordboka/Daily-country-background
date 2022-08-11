mod date;
mod national_day;
mod parse_error;
mod parse_file;

use image;
use imageproc::drawing;
use imageproc::rect;
use national_day::NationalDay;
use rusttype::{Font, Scale};
use std::cmp;
use std::collections::HashMap;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;
use std::path::PathBuf;

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

fn split_long_text_into_multiple_lines(long_text: String, cutoff: usize) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let mut current_line = String::new();
    let words = long_text.split_ascii_whitespace();
    let mut line_length = 0;
    for word in words {
        if line_length + word.len() > cutoff {
            lines.push(current_line.trim_start().to_string());
            current_line = String::new();
            line_length = 0;
        }
        current_line += &(String::from(" ") + &word.to_owned());
        line_length += word.len();
    }
    lines.push(current_line.trim_start().to_string());
    lines
}

fn add_text_to_picture(country: &NationalDay, file: &Path) -> PathBuf {
    let mut image = image::open(file)
        .expect("No image found at provided path")
        .to_rgba8();
    let text_color = image::Rgba([255u8, 255u8, 255u8, 255u8]);
    let background_color = image::Rgba([155u8, 155u8, 155u8, 155u8]);
    let big_font = Vec::from(include_bytes!("Inter-ExtraBold.ttf") as &[u8]);
    let big_font = Font::try_from_vec(big_font).unwrap();
    let small_font = Vec::from(include_bytes!("Inter-Regular.ttf") as &[u8]);
    let small_font = Font::try_from_vec(small_font).unwrap();
    let cutoff = cmp::max(country.country.len() * 2, 30);
    let detail_texts = split_long_text_into_multiple_lines(country.extra_info.clone(), cutoff);
    let mut max_width: i32 = 0;
    let mut total_height: i32 = 0;
    let big_scale = Scale { x: 50.0, y: 50.0 };
    let small_scale = Scale { x: 25.0, y: 25.0 };
    let (width, height) = drawing::text_size(big_scale, &big_font, &country.country);
    if width > max_width {
        max_width = width;
    }
    total_height += height + 5;
    let (width, height) = drawing::text_size(small_scale, &small_font, &country.date.to_string());
    if width > max_width {
        max_width = width;
    }
    total_height += height + 5;
    for line in &detail_texts {
        let (width, height) = drawing::text_size(small_scale, &small_font, line);
        if width > max_width {
            max_width = width;
        }
        total_height += height + 5;
    }
    let background_rectangle = rect::Rect::at(25, 25).of_size(
        u32::try_from(max_width + 50).unwrap(),
        u32::try_from(total_height + 45).unwrap(),
    );
    let mut current_height = 50;
    drawing::draw_filled_rect_mut(&mut image, background_rectangle, background_color);
    drawing::draw_text_mut(
        &mut image,
        text_color,
        50,
        current_height,
        big_scale,
        &big_font,
        &country.country,
    );
    current_height += big_scale.y as i32 + 5;
    drawing::draw_text_mut(
        &mut image,
        text_color,
        50,
        current_height,
        small_scale,
        &small_font,
        &country.date.to_string(),
    );
    current_height += small_scale.y as i32 + 5;
    for line in &detail_texts {
        drawing::draw_text_mut(
            &mut image,
            text_color,
            50,
            current_height,
            small_scale,
            &small_font,
            line,
        );
        current_height += small_scale.y as i32 + 3;
    }
    let file_format = file.to_str().unwrap().split(".").last().unwrap();
    let edited_file_path = "todays_picture.".to_owned() + file_format;
    image.save(&edited_file_path).unwrap();
    Path::new(&edited_file_path).to_owned()
}

fn set_wallpaper_for_country(country: &NationalDay) -> Result<(), &str> {
    let wallpaper_file = find_file_for_country(country);
    match wallpaper_file {
        Some(file) => {
            let file_path = file.path();
            let file_path = fs::canonicalize(&file_path).expect("Could not canonicalize the file");
            let edited_file_path = add_text_to_picture(country, &file_path);
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
