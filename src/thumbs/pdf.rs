use image::DynamicImage;
use pdfium_render::prelude::{PdfRenderConfig, Pdfium, PdfiumError};

use crate::thumbnail::*;
use std::path::Path;

pub struct Pdf;

impl Thumbnailable for Pdf {
    const MIME_TYPES: &'static [&'static str] = &["application/pdf"];

    fn run(thumbnailer: &Thumbnailer, path: &Path) -> anyhow::Result<DynamicImage> {
        let pdfium = get_pdfium()?;

        let w: i32 = thumbnailer.width.try_into()?;
        let h: i32 = thumbnailer.height.try_into()?;

        let document = pdfium.load_pdf_from_file(&path, None)?;
        let render_config = PdfRenderConfig::new().scale_page_to_display_size(w, h);

        let first_page = document.pages().first()?;
        let img = first_page.render_with_config(&render_config)?.as_image();
        Ok(img)
    }
}

fn get_pdfium() -> Result<Pdfium, PdfiumError> {
    let lib = Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./"))
        .or_else(|_| Pdfium::bind_to_system_library())?;
    Ok(Pdfium::new(lib))
}
