use image::{DynamicImage, ImageDecoder, ImageReader};

use crate::thumbnail::*;
use std::path::Path;

pub struct Img;

impl Thumbnailable for Img {
    const MIME_TYPES: &'static [&'static str] = &[
        "image/jpeg",
        "image/png",
        "image/gif",
        "application/x-riff",
        "image/bmp",
    ];

    fn run(thumbnailer: &Thumbnailer, path: &Path) -> anyhow::Result<DynamicImage> {
        let mut decoder = ImageReader::open(path)?
            .with_guessed_format()?
            .into_decoder()?;

        // Read the orientation before consuming the decoder.
        let orientation = decoder.orientation()?;

        let mut image = DynamicImage::from_decoder(decoder)?;

        // Apply EXIF orientation before resizing.
        image.apply_orientation(orientation);

        Ok(image.thumbnail(thumbnailer.width, thumbnailer.height))
    }
}