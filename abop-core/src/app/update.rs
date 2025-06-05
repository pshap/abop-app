//! State update logic for the application

use super::{Message, state::AppState};

/// Updates the application state based on the given message
pub const fn update(state: &mut AppState, message: Message) {
    match message {
        Message::Navigate(view) => {
            state.current_view = view;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::state::View;

    #[test]
    fn test_navigation() {
        let mut state = AppState::default();

        // Test navigation to Player view
        update(&mut state, Message::Navigate(View::Player));
        assert_eq!(state.current_view, View::Player);

        // Test navigation to Settings view
        update(&mut state, Message::Navigate(View::Settings));
        assert_eq!(state.current_view, View::Settings);

        // Test navigation to Library view
        update(&mut state, Message::Navigate(View::Library));
        assert_eq!(state.current_view, View::Library);
    }
}
