//! A library to make placing text on images easier. Extends draw_text_mut's functionality from [imageproc](https://docs.rs/imageproc/0.23.0/imageproc/index.html).

use std::fmt::Display;

use image::{DynamicImage, ImageError, Rgba};
use imageproc::drawing::draw_text_mut;
use rusttype::{point, Font, Scale};

#[derive(Debug)]
pub enum TextOnImageError {
    ImageError(ImageError),
}

/// Defines how the text extends from the point you place it.
#[derive(Default)]
pub enum TextJustify {
    Left,
    #[default]
    Center,
    Right,
}

/// Defines where the text sits relative to the vertical coordinate provided.
#[derive(Default)]
pub enum VerticalAnchor {
    Top,
    #[default]
    Center,
    Bottom,
}

/// Choose whether text wraps if it would extend beyond a specified pixel length.
#[derive(Default)]
pub enum WrapBehavior {
    #[default]
    NoWrap,
    Wrap(u32),
}
impl WrapBehavior {
    pub fn new(max_width: u32) -> Self {
        WrapBehavior::Wrap(max_width)
    }
}

/// A bundle of font related values.
pub struct FontBundle<'a> {
    font: &'a Font<'a>,
    scale: Scale,
    color: Rgba<u8>,
}

impl Display for FontBundle<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "FontBundle{{font: {:?}, scale: {:?}, color: {:?}}}",
            self.font, self.scale, self.color
        )
    }
}

impl<'a> FontBundle<'a> {
    pub fn new(font_: &'a Font<'a>, scale_: Scale, color_: Rgba<u8>) -> Self {
        if scale_.x <= 0. || scale_.y <= 0. {
            panic!("text_on_image: FontBundle scale.x or scale.y cannot be <= 0.0!");
        }
        FontBundle {
            font: font_,
            scale: scale_,
            color: color_,
        }
    }

    pub fn set_scale(&mut self, scale_: Scale) {
        if scale_.x <= 0. || scale_.y <= 0. {
            panic!("text_on_image: FontBundle scale.x or scale.y cannot be <= 0.0!");
        }
        self.scale = scale_;
    }

    pub fn set_color(&mut self, color_: Rgba<u8>) {
        self.color = color_;
    }
}

/// Draws text on an image with support for text jusification, vertical anchor, and line wrapping.
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
        WrapBehavior::Wrap(max_width) => {
            if max_width < get_text_width(font_bundle, "mm") {
                panic!("text_on_image: Cannot set max_width for wrapping below 2 ems! Try setting max_width to at least {}", get_text_width(font_bundle, "mm"));
            }
            let mut lines_altered: Vec<String> = vec![];
            for &line in &lines {
                let mut buffer: String = String::new();
                for word in line.split_whitespace() {
                    if cfg!(debug_assertions) {
                        println!(
                            "\"{}\" has width {}. Compare to max_width {}",
                            buffer.clone() + " " + word,
                            get_text_width(font_bundle, buffer.clone() + " " + word),
                            max_width
                        );
                    }
                    let optional_space_width: u32 = if buffer.is_empty() {
                        get_text_width(font_bundle, " ")
                    } else {
                        0
                    };
                    if get_text_width(font_bundle, buffer.clone() + " " + word)
                        <= max_width + optional_space_width
                    {
                        //Add word to line
                        if cfg!(debug_assertions) {
                            println!("Word {} gets added to line", word);
                        }
                        if buffer.is_empty() {
                            buffer += word;
                        } else {
                            buffer = buffer + " " + word;
                        }
                    } else if get_text_width(font_bundle, buffer.clone() + " " + word) > max_width
                        && buffer.is_empty()
                    {
                        //add partial word with a dash at the end
                        let word_chars = word.chars();
                        for word_char in word_chars {
                            if get_text_width(font_bundle, buffer.clone() + "-") <= max_width {
                                buffer = buffer + &word_char.to_string();
                            } else {
                                buffer += "-";
                                lines_altered.push(buffer);
                                buffer = String::new();
                                buffer = buffer + &word_char.to_string();
                            }
                        }
                    } else if get_text_width(font_bundle, buffer.clone() + " " + word) > max_width
                        && !buffer.is_empty()
                    {
                        if cfg!(debug_assertions) {
                            println!("Word {} goes over max width && buffer is not empty.", word);
                        }
                        //write buffer to lines_altered, empty buffer, evaluate as new line
                        lines_altered.push(buffer);
                        buffer = String::new();
                        let word_chars = word.chars();
                        for word_char in word_chars {
                            if get_text_width(font_bundle, buffer.clone() + "-") <= max_width {
                                buffer = buffer + &word_char.to_string();
                            } else {
                                buffer += "-";
                                lines_altered.push(buffer);
                                buffer = String::new();
                            }
                        }
                    }
                }
                lines_altered.push(buffer);
            }
            let lines_altered: Vec<&str> = lines_altered.iter().map(|line| line.as_str()).collect();
            if cfg!(debug_assertions) {
                println!("Lines altered:\n{:?}", lines_altered);
            }
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

/// Helper function to get text width.
fn get_text_width<T: AsRef<str>>(font_bundle: &FontBundle, text: T) -> u32 {
    font_bundle
        .font
        .layout(text.as_ref(), font_bundle.scale, point(0., 0.))
        .map(|glyph| glyph.position().x + glyph.unpositioned().h_metrics().advance_width)
        .last()
        .unwrap_or(0.) as u32
}

/// Helper function to get text height.
fn get_text_height(font_bundle: &FontBundle) -> i32 {
    let v_metrics = font_bundle.font.v_metrics(font_bundle.scale);
    (v_metrics.ascent - v_metrics.descent + v_metrics.line_gap) as i32
}

/// Draws text on an image with a small cross where the coordinates are.
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
    for &line in &lines {
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
            VerticalAnchor::Bottom => -(get_text_height(font_bundle) * (lines_len - current_line)),
        };
        let horizontal_offset = match horizontal_justify {
            TextJustify::Left => 0,
            TextJustify::Center => get_text_width(font_bundle, line) / 2,
            TextJustify::Right => get_text_width(font_bundle, line),
        };
        draw_text_mut(
            image,
            font_bundle.color,
            pixels_from_left - horizontal_offset as i32,
            pixels_from_top + vertical_offset,
            font_bundle.scale,
            font_bundle.font,
            line,
        );
        if cfg!(debug_assertions) {
            println!("pixels_from_left for line {}: {}", line, pixels_from_left);
        }
        current_line += 1;
    }
}

#[cfg(test)]
mod test;
