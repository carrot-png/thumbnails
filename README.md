# Thumbnails

## A thumbnailing library.

This crate converts various file formats into an `image::DynamicImage` of a user-specified size.

Supports images, videos, ZIP/CBZ archives, and PDF files.

## API
Create a thumbnail:
```rust
use thumbnails::Thumbnailer;

let thumbnailer = Thumbnailer::new(250, 250);
let thumb = thumbnailer.get("video.mp4")?;
thumb.save("thumb.png")?;
```
