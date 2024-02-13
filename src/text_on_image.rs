use std::fmt::Display;

use image::{DynamicImage, ImageError, Rgba};
use imageproc::drawing::draw_text_mut;
use rusttype::{point, Font, Scale};

#[derive(Debug)]
pub enum TextOnImageError {
    MissingFontError,
    ImageError(ImageError),
    UnhandledError,
}

pub enum HorizontalJustify {
    Left,
    Center,
    Right,
}
impl Default for HorizontalJustify {
    fn default() -> Self {
        HorizontalJustify::Left
    }
}
pub enum VerticalJustify {
    Top,
    Center,
    Bottom,
}
impl Default for VerticalJustify {
    fn default() -> Self {
        VerticalJustify::Top
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
    pub fn new(max_width: u32) -> Self {
        WrapBehavior::Wrap(max_width)
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
    text_justify: HorizontalJustify,
    wrap_behavior: WrapBehavior,
) {
    let lines: Vec<&str> = text.as_ref().lines().map(|line| line.trim()).collect();
    let words_in_lines: Vec<Vec<&str>> = lines
        .iter()
        .map(|line| line.split_whitespace().collect())
        .collect();
    match wrap_behavior {
        WrapBehavior::NoWrap => {
            let mut current_line = 0;
            for (&line) in &lines {
                let vertical_offset = get_text_height(font_bundle) * current_line;
                let horizontal_offset = match text_justify {
                    HorizontalJustify::Left => 0,
                    HorizontalJustify::Center => get_text_width(font_bundle, line) / 2,
                    HorizontalJustify::Right => get_text_width(font_bundle, line),
                };
                draw_text_mut(
                    image,
                    Rgba([0, 0, 0, 255]),
                    pixels_from_left - horizontal_offset,
                    pixels_from_top + vertical_offset,
                    font_bundle.scale,
                    font_bundle.font,
                    line,
                );
                println!("pixels_from_left for line {}: {}", line, pixels_from_left);
                current_line += 1;
            }
        }
        WrapBehavior::Wrap(max_width) => {
            let line_count = 1;
            let mut current_width = 0;
            todo!()
        }
    }
}

fn get_text_width(font_bundle: &FontBundle, text: &str) -> i32 {
    font_bundle
        .font
        .layout(text, font_bundle.scale, point(0., 0.))
        .map(|g| g.position().x + g.unpositioned().h_metrics().advance_width)
        .last()
        .unwrap_or(0.) as i32
    //todo!()
}

fn get_text_height(font_bundle: &FontBundle) -> i32 {
    //probably don't need text parameter
    let v_metrics = font_bundle.font.v_metrics(font_bundle.scale);
    (v_metrics.ascent - v_metrics.descent + v_metrics.line_gap) as i32
}
