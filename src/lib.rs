//! # Overview
//!
//! This crate provides a simple API to generate thumbnails from a variety of file types.
//!
//! Create and save a thumbmail using [`Thumbnailer`]:
//! ```rust,no_run
//! use thumbnails::Thumbnailer;
//!
//! # fn main() -> anyhow::Result<()> {
//! let thumbnailer = Thumbnailer::new(250, 250);
//! let thumb = thumbnailer.get("video.mp4")?;
//! thumb.save("thumb.png")?;
//! # Ok(())
//! # }
//! ```

#![warn(missing_docs)]
pub use crate::thumbnail::Thumbnailer;

mod thumbnail;
mod thumbs;
