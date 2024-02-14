use std::{fmt::Display, io::BufRead};

use image::{DynamicImage, ImageError, Rgba};
use imageproc::drawing::draw_text_mut;
use rusttype::{point, Font, Scale};

#[derive(Debug)]
pub enum TextOnImageError {
    MissingFontError,
    ImageError(ImageError),
    UnhandledError,
}

pub enum TextJustify {
    Left,
    Center,
    Right,
}
impl Default for TextJustify {
    fn default() -> Self {
        TextJustify::Left
    }
}
pub enum VerticalAnchor {
    Top,
    Center,
    Bottom,
}
impl Default for VerticalAnchor {
    fn default() -> Self {
        VerticalAnchor::Top
    }
}

pub enum WrapBehavior {
    NoWrap,
    Wrap(u32),
}
impl Default for WrapBehavior {
    fn default() -> Self {
        WrapBehavior::NoWrap
    }
}
impl WrapBehavior {
    pub fn new(width_before_newline: u32) -> Self {
        WrapBehavior::Wrap(width_before_newline)
    }
}

//#[derive(Display)]
pub struct FontBundle<'a> {
    font: &'a Font<'a>,
    scale: Scale,
}

impl<'a> Display for FontBundle<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FontBundle{{font: {:?}, scale: {:?}}}",
            self.font, self.scale
        )
    }
}

impl<'a> FontBundle<'a> {
    pub fn new(font_: &'a Font<'a>, scale_: Scale) -> Self {
        FontBundle {
            font: font_,
            scale: scale_,
        }
    }

    pub fn set_scale(&mut self, scale_: Scale) {
        self.scale = scale_;
    }
}

pub fn text_on_image<T: AsRef<str>>(
    image: &mut DynamicImage,
    text: T,
    font_bundle: &FontBundle<'_>,
    pixels_from_left: i32,
    pixels_from_top: i32,
    horizontal_justify: TextJustify,
    vertical_anchor: VerticalAnchor,
    wrap_behavior: WrapBehavior,
) {
    let lines: Vec<&str> = text.as_ref().lines().map(|line| line.trim()).collect();
    match wrap_behavior {
        WrapBehavior::NoWrap => position_and_draw(
            image,
            lines,
            font_bundle,
            pixels_from_left,
            pixels_from_top,
            horizontal_justify,
            vertical_anchor,
        ),
        WrapBehavior::Wrap(width_before_newline) => {
            let mut lines_altered: Vec<String> = vec![];
            for &line in &lines {
                let mut current_width: u32 = 0;
                let mut buffer: String = String::new();
                for word in line.split_whitespace() {
                    if buffer.is_empty() {
                        buffer = buffer + word;
                    } else {
                        buffer = buffer + " " + word;
                    }
                    current_width += get_text_width(font_bundle, word);
                    if current_width > width_before_newline {
                        lines_altered.push(buffer);
                        buffer = String::new();
                        current_width = 0;
                    }
                }
            }
            let lines_altered: Vec<&str> = lines_altered.iter().map(|line| line.as_str()).collect();
            println!("Lines altered:\n{:?}", lines_altered);
            position_and_draw(
                image,
                lines_altered,
                font_bundle,
                pixels_from_left,
                pixels_from_top,
                horizontal_justify,
                vertical_anchor,
            )
        }
    }
}

fn get_text_width(font_bundle: &FontBundle, text: &str) -> u32 {
    font_bundle
        .font
        .layout(text, font_bundle.scale, point(0., 0.))
        .map(|g| g.position().x + g.unpositioned().h_metrics().advance_width)
        .last()
        .unwrap_or(0.) as u32
    //todo!()
}

fn get_text_height(font_bundle: &FontBundle) -> i32 {
    //probably don't need text parameter
    let v_metrics = font_bundle.font.v_metrics(font_bundle.scale);
    (v_metrics.ascent - v_metrics.descent + v_metrics.line_gap) as i32
}

pub fn text_on_image_draw_debug<T: AsRef<str>>(
    image: &mut DynamicImage,
    text: T,
    font_bundle: &FontBundle<'_>,
    pixels_from_left: i32,
    pixels_from_top: i32,
    horizontal_justify: TextJustify,
    vertical_justify: VerticalAnchor,
    wrap_behavior: WrapBehavior,
) {
    imageproc::drawing::draw_cross_mut(
        image,
        Rgba([255, 0, 0, 255]),
        pixels_from_left,
        pixels_from_top,
    );
    text_on_image(
        image,
        text,
        font_bundle,
        pixels_from_left,
        pixels_from_top,
        horizontal_justify,
        vertical_justify,
        wrap_behavior,
    );
}

fn position_and_draw(
    image: &mut DynamicImage,
    lines: Vec<&str>,
    font_bundle: &FontBundle<'_>,
    pixels_from_left: i32,
    pixels_from_top: i32,
    horizontal_justify: TextJustify,
    vertical_anchor: VerticalAnchor,
) {
    let lines_len = lines.len() as i32;
    let mut current_line = 0;
    for (&line) in &lines {
        if cfg!(debug_assertions) {
            println!("{} width: {}", line, get_text_width(font_bundle, line));
        }
        let vertical_offset = match vertical_anchor {
            VerticalAnchor::Top => get_text_height(font_bundle) * current_line,
            VerticalAnchor::Center => {
                (get_text_height(font_bundle) * current_line
                    - get_text_height(font_bundle) * (lines_len - current_line))
                    / 2
            }
            VerticalAnchor::Bottom => {
                -1 * (get_text_height(font_bundle) * (lines_len - current_line))
            }
        };
        let horizontal_offset = match horizontal_justify {
            TextJustify::Left => 0,
            TextJustify::Center => get_text_width(font_bundle, line) / 2,
            TextJustify::Right => get_text_width(font_bundle, line),
        };
        draw_text_mut(
            image,
            Rgba([0, 0, 0, 255]),
            pixels_from_left - horizontal_offset as i32,
            pixels_from_top + vertical_offset,
            font_bundle.scale,
            font_bundle.font,
            line,
        );
        println!("pixels_from_left for line {}: {}", line, pixels_from_left);
        current_line += 1;
    }
}
