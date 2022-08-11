use image;
use image::ImageBuffer;
use image::Rgba;
use imageproc::drawing;
use imageproc::rect;
use rusttype::{Font, Scale};
use std::cmp;
use std::path::Path;
use std::path::PathBuf;

use crate::national_day::NationalDay;

///Splits a string into multiple strings where each string is shorter than `cutoff`
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

struct TextFont {
    font: Font<'static>,
    scale: Scale,
}

impl TextFont {
    pub fn new(font_vec: Vec<u8>, text_size: f32) -> Self {
        let font = Font::try_from_vec(font_vec).unwrap();
        let scale = Scale {
            x: text_size,
            y: text_size,
        };
        TextFont { font, scale }
    }
}

struct BoxDimensions {
    width: i32,
    height: i32,
}

/// Based on the text that is to be added, determines the needed size of the box to be used as background
///
/// Assumes the country name will be written with the `big_font` while the country date and the `detail_texts`
/// will use the small font
///
/// Assumes that the line spacing is halved between the `detail_texts`
fn determine_box_size(
    country: &NationalDay,
    detail_texts: &Vec<String>,
    big_font: &TextFont,
    small_font: &TextFont,
    line_spacing: i32,
) -> BoxDimensions {
    let mut max_width: i32 = 0;
    let mut total_height: i32 = 0;
    let (width, height) = drawing::text_size(big_font.scale, &big_font.font, &country.country);
    if width > max_width {
        max_width = width;
    }
    total_height += height + line_spacing;
    let (width, height) = drawing::text_size(
        small_font.scale,
        &small_font.font,
        &country.date.to_string(),
    );
    if width > max_width {
        max_width = width;
    }
    total_height += height + line_spacing;
    for line in detail_texts {
        let (width, height) = drawing::text_size(small_font.scale, &small_font.font, line);
        if width > max_width {
            max_width = width;
        }
        total_height += height + line_spacing / 2;
    }
    BoxDimensions {
        width: max_width,
        height: total_height,
    }
}

/// Adds text to the given image
///
/// Text that is added is the country name using the big font and the date and the `detail_texts`
/// using the small font.
///
/// Adds line spacing as specified between the parts, but only half line spacing between lines of the
/// `detail_texts`
fn add_text_to_image(
    image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    country: &NationalDay,
    detail_texts: &Vec<String>,
    big_font: &TextFont,
    small_font: &TextFont,
    line_spacing: i32,
    text_color: Rgba<u8>,
    y: i32,
    x: i32,
) {
    let mut current_y = y;
    drawing::draw_text_mut(
        image,
        text_color,
        x,
        current_y,
        big_font.scale,
        &big_font.font,
        &country.country,
    );
    current_y += big_font.scale.y as i32 + line_spacing;
    drawing::draw_text_mut(
        image,
        text_color,
        x,
        current_y,
        small_font.scale,
        &small_font.font,
        &country.date.to_string(),
    );
    current_y += small_font.scale.y as i32 + 5;
    for line in detail_texts {
        drawing::draw_text_mut(
            image,
            text_color,
            x,
            current_y,
            small_font.scale,
            &small_font.font,
            line,
        );
        current_y += small_font.scale.y as i32 + line_spacing / 2;
    }
}

/// Adds text to the picture at the file provided.
///
/// Adds text about the country name, country date as well as the extra information.
pub fn add_text_to_picture(country: &NationalDay, file: &Path) -> PathBuf {
    let mut image = image::open(file)
        .expect("No image found at provided path")
        .to_rgba8();
    let big_to_small_scale = 2;
    let minimum_cutoff = 30;
    let small_text_size = 25;
    let line_spacing = 5;
    let big_font = TextFont::new(
        Vec::from(include_bytes!("Inter-ExtraBold.ttf") as &[u8]),
        (small_text_size * big_to_small_scale) as f32,
    );
    let small_font = TextFont::new(
        Vec::from(include_bytes!("Inter-Regular.ttf") as &[u8]),
        small_text_size as f32,
    );
    let text_color = image::Rgba([255u8, 255u8, 255u8, 255u8]);
    let background_color = image::Rgba([155u8, 155u8, 155u8, 155u8]);
    let cutoff = cmp::max(country.country.len() * big_to_small_scale, minimum_cutoff);
    let detail_texts = split_long_text_into_multiple_lines(country.extra_info.clone(), cutoff);
    let box_dimensions =
        determine_box_size(country, &detail_texts, &big_font, &small_font, line_spacing);
    let margin = 25;
    let background_rectangle = rect::Rect::at(margin, margin).of_size(
        u32::try_from(box_dimensions.width + 2 * margin).unwrap(),
        u32::try_from(box_dimensions.height + 2 * margin).unwrap(),
    );
    drawing::draw_filled_rect_mut(&mut image, background_rectangle, background_color);
    add_text_to_image(
        &mut image,
        country,
        &detail_texts,
        &big_font,
        &small_font,
        line_spacing,
        text_color,
        margin * 2,
        margin * 2,
    );
    let file_format = file.to_str().unwrap().split(".").last().unwrap();
    let edited_file_path = "todays_picture.".to_owned() + file_format;
    image.save(&edited_file_path).unwrap();
    Path::new(&edited_file_path).to_owned()
}
