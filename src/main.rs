mod text_on_image;
use image::ImageError;
//use image::{DynamicImage, ImageError};
use rusttype::{Font, Scale};
use text_on_image::*;

#[derive(Debug)]
enum PossibleErrors {
    ImageOpeningError(ImageError),
    FontFailure,
    ImageSaveFailure(ImageError),
    TextOnImageError(ImageError),
}
fn main() -> Result<(), PossibleErrors> {
    //Load image
    let mut background = image::open("assets/background.png")
        .map_err(|err| PossibleErrors::ImageOpeningError(err))?;
    //Set up font
    let font = Vec::from(FONT);
    let font = Font::try_from_vec(font).unwrap();
    let mut font_bundle = FontBundle::new(&font, Scale { x: 40., y: 40. });
    //draw on image
    text_on_image(
        &mut background,
        "This is Line 1
        This is Line 2
        This is a Line with more content.",
        &font_bundle,
        400,
        0,
        HorizontalJustify::Center,
        WrapBehavior::NoWrap,
    );
    //save image
    background
        .save("./output/out.png")
        .map_err(|err| PossibleErrors::ImageSaveFailure(err))?;
    Ok(())
}

const FONT: &[u8] = include_bytes!("../assets/BitstreamVeraSansMonoBold-pq1a.ttf");