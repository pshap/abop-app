//! Handler for cover art related operations

use iced::Command;
use log::{error, info};
use std::sync::Arc;

use crate::{
    cover_art,
    messages::Message,
    state::AppState,
    utils::image_cache::ImageCache,
};

/// Handle cover art related messages
pub fn handle_cover_art_message(
    state: &mut AppState,
    message: Message,
    image_cache: &Arc<ImageCache>,
) -> Option<Command<Message>> {
    match message {
        Message::EditCoverArt(audiobook_id) => {
            info!("Edit cover art requested for audiobook: {}", audiobook_id);
            // TODO: Open file dialog to select new cover art
            None
        }
        
        Message::RemoveCoverArt(audiobook_id) => {
            info!("Remove cover art requested for audiobook: {}", audiobook_id);
            let cache = Arc::clone(image_cache);
            Some(Command::perform(
                async move {
                    match cover_art::remove_cover_art(audiobook_id.clone(), &cache).await {
                        Ok(_) => Message::CoverArtUpdated(audiobook_id),
                        Err(e) => Message::CoverArtError { 
                            audiobook_id, 
                            error: e.to_string() 
                        },
                    }
                },
                |msg| msg,
            ))
        }
        
        Message::CoverArtUpdated(audiobook_id) => {
            info!("Cover art updated for audiobook: {}", audiobook_id);
            // TODO: Update the UI to reflect the change
            None
        }
        
        Message::CoverArtError { audiobook_id, error } => {
            error!("Error processing cover art for {}: {}", audiobook_id, error);
            // TODO: Show error message to the user
            None
        }
        
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tempfile::tempdir;
    use crate::state::AppState;
    use crate::utils::image_cache::ImageCache;

    #[tokio::test]
    async fn test_remove_cover_art() {
        // Setup test environment
        let temp_dir = tempdir().unwrap();
        let cache = Arc::new(ImageCache::new());
        let mut state = AppState::default();
        
        // Test removing cover art
        let audiobook_id = "test-book".to_string();
        let message = Message::RemoveCoverArt(audiobook_id.clone());
        
        // The actual test would need a more complete setup with a real database
        // For now, we just verify the function doesn't panic
        let _ = handle_cover_art_message(&mut state, message, &cache);
    }
}
