# Thumbnails
[![crates.io](https://img.shields.io/crates/v/thumbnails.svg)](https://crates.io/crates/thumbnails)
[![Documentation](https://docs.rs/thumbnails/badge.svg)](https://docs.rs/thumbnails)

## A thumbnailing library.

This crate converts various file formats into an `image::DynamicImage` of a user-specified size.

Supports images, videos, PDFs, and ZIP/CBZ archives.

## API
Create a thumbnail:
```rust
use thumbnails::Thumbnailer;

let thumbnailer = Thumbnailer::new(250, 250);
let thumb = thumbnailer.get("video.mp4")?;
thumb.save("thumb.png")?;
```

## Building
Some file types require additional setup and can be disabled via `features` if unneeded.

Video thumbnails depend on `ffmpeg`. See [rust-ffmpeg](https://github.com/zmwangx/rust-ffmpeg/wiki/Notes-on-building)

PDF thumbnails depend on `pdfium`. See [pdfium-render](https://github.com/ajrcarey/pdfium-render?#dynamic-linking)
