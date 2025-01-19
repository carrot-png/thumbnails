//! # Overview
//!
//! This crate provides a simple API to generate thumbnails from a variety of file types.
//!
//! Create and save a thumbmail using [`Thumbnailer`]:
//! ```rust
//! use thumbnails::Thumbnailer;
//! 
//! let thumbnailer = Thumbnailer::new(250, 250);
//! let thumb = thumbnailer.get("video.mp4")?;
//! thumb.save("thumb.png")?;
//! ```

#![warn(missing_docs)]
pub use crate::thumbnail::Thumbnailer;

mod thumbnail;
mod thumbs;
