# Placing text on images just got easier.

While making a bingo card generator using [image](https://crates.io/crates/image) and [imageproc](https://crates.io/crates/imageproc), I recognized a need for a better way to place text on images. This crate depends on those two crates, but provides users with extra functionality.

## Why use this?

If you are looking to place multiple lines of text on an image that are spatially related, then this should be a better option than using imageproc's draw_text_mut function.

## Features

- Text justification: Left, Center, Right
- Vertical anchor: Top, Center, Bottom
- Text wrapping: Choose a max length in pixels, and your text will wrap to a new line, respecting your choices for text justification and vertical anchoring.
