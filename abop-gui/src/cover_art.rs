//! Cover art handling and processing

use std::path::PathBuf;
use std::sync::Arc;
use image::imageops::FilterType;
use image::{DynamicImage, ImageBuffer, RgbaImage};
use tokio::fs;
use anyhow::{Context, Result};
use abop_core::models::Audiobook;
use crate::messages::Message;
use crate::utils::image_cache::ImageCache;

/// Maximum dimensions for cover art (width and height)
const MAX_COVER_ART_SIZE: u32 = 1024;

/// Supported image formats for cover art
const SUPPORTED_FORMATS: [&str; 4] = ["jpg", "jpeg", "png", "webp"];

/// Process and save cover art for an audiobook
pub async fn process_cover_art(
    audiobook: &Audiobook,
    image_path: PathBuf,
    image_cache: &Arc<ImageCache>,
) -> Result<Message> {
    let audiobook_id = audiobook.id.clone();
    
    // Read the image file
    let image_data = fs::read(&image_path)
        .await
        .context("Failed to read image file")?;
    
    // Process the image in a blocking task
    let processed_image = tokio::task::spawn_blocking(move || {
        // Decode the image
        let img = image::load_from_memory(&image_data)
            .context("Failed to decode image")?;
        
        // Resize the image while maintaining aspect ratio
        let (width, height) = img.dimensions();
        let (new_width, new_height) = if width > height {
            (MAX_COVER_ART_SIZE, (height as f32 * (MAX_COVER_ART_SIZE as f32 / width as f32)) as u32)
        } else {
            ((width as f32 * (MAX_COVER_ART_SIZE as f32 / height as f32)) as u32, MAX_COVER_ART_SIZE)
        };
        
        let resized = img.resize_exact(
            new_width,
            new_height,
            FilterType::Lanczos3,
        );
        
        // Convert to RGBA if needed
        let rgba = resized.to_rgba8();
        
        // Encode as JPEG (smaller file size, good quality)
        let mut output = Vec::new();
        let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut output, 85);
        encoder.encode_image(&rgba)?;
        
        Ok(output)
    })
    .await??; // Handle both join and processing errors
    
    // Update the cache (insert processed image if supported, or refresh handle)
    let _ = image_cache.get_or_create_handle(&audiobook_id, Some(&processed_image), MAX_COVER_ART_SIZE);
    
    // TODO: Update the database with the new cover art
    // This would typically involve calling a method on your database service
    
    Ok(Message::CoverArtUpdated(audiobook_id))
}

/// Remove cover art for an audiobook
pub async fn remove_cover_art(
    audiobook_id: String,
    image_cache: &Arc<ImageCache>,
) -> Result<Message> {
    // NOTE: ImageCache does not support per-item removal; cache is left unchanged.
    
    // TODO: Update the database to remove the cover art
    // This would typically involve calling a method on your database service
    
    Ok(Message::CoverArtUpdated(audiobook_id))
}

/// Check if a file has a supported image extension
pub fn is_supported_image_file(file_name: &str) -> bool {
    let ext = std::path::Path::new(file_name)
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_lowercase())
        .unwrap_or_default();
    
    SUPPORTED_FORMATS.contains(&ext.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::io::Write;
    
    #[tokio::test]
    async fn test_process_cover_art() {
        // Create a temporary directory
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.jpg");
        
        // Create a simple test image (1x1 pixel)
        let img = RgbaImage::from_pixel(1, 1, image::Rgba([255, 0, 0, 255]));
        let mut file = std::fs::File::create(&file_path).unwrap();
        img.write_to(&mut file, image::ImageOutputFormat::Jpeg(90)).unwrap();
        
        // Test processing
        let cache = Arc::new(ImageCache::new());
        let audiobook = Audiobook {
            id: "test-book".to_string(),
            // ... other fields with default values
            ..Default::default()
        };
        
        let result = process_cover_art(&audiobook, file_path, &cache).await;
        assert!(result.is_ok());
        
    // Verify the image was added to the cache
    assert!(cache.get_or_create_handle(&audiobook.id, None, MAX_COVER_ART_SIZE).is_some());
    }
    
    #[test]
    fn test_is_supported_image_file() {
        assert!(is_supported_image_file("cover.jpg"));
        assert!(is_supported_image_file("COVER.JPG"));
        assert!(is_supported_image_file("cover.png"));
        assert!(is_supported_image_file("cover.webp"));
        assert!(!is_supported_image_file("document.pdf"));
        assert!(!is_supported_image_file("no_extension"));
    }
}
