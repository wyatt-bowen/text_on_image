use crate::prelude::*;
use image::{ImageError, Rgba};
use rusttype::{Font, Scale};

#[derive(Debug)]
enum PossibleErrors {
    ImageOpeningError(ImageError),
    ImageSaveFailure(ImageError),
}

const FONT: &[u8] = include_bytes!("../assets/BitstreamVeraSansMonoBold-pq1a.ttf");

#[test]
fn test_example_text() -> Result<(), PossibleErrors> {
    let mut background = image::open("assets/background.png")
        .map_err(|err| PossibleErrors::ImageOpeningError(err))?;
    //Set up font
    let font = Vec::from(FONT);
    let font = Font::try_from_vec(font).unwrap();
    let font_bundle = FontBundle::new(&font, Scale { x: 40., y: 40. }, Rgba([0, 255, 0, 255]));
    //draw on image
    text_on_image_draw_debug(
        &mut background,
        "This is Line 1
        Thisislinewithextralong 2",
        &font_bundle,
        400,
        800,
        TextJustify::Center,
        VerticalAnchor::Center,
        WrapBehavior::Wrap(250),
    );
    //save image
    background
        .save("./output/test_example_text.png")
        .map_err(|err| PossibleErrors::ImageSaveFailure(err))?;
    Ok(())
}

#[test]
#[should_panic]
fn test_negative_scale() {
    let font = Vec::from(FONT);
    let font = Font::try_from_vec(font).unwrap();
    let _font_bundle = FontBundle::new(&font, Scale { x: -40., y: 40. }, Rgba([0, 255, 0, 255]));
}
