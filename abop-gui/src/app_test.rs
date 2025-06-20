//! Tests for the main application module

use super::*;
use iced::{executor, Application, Command, Settings};
use std::{sync::Arc, time::Duration};
use tokio::sync::mpsc::unbounded_channel;

/// Test that the application can be created with default settings
#[test]
fn test_app_new() {
    let app = App::new();
    assert!(app.services().is_ok());
    assert_eq!(app.state.current_route(), &Route::Home);
}

/// Test application initialization
#[tokio::test]
async fn test_app_initial() {
    let (app, task) = App::initial();
    assert_eq!(app.state.current_route(), &Route::Home);
    assert!(task.is_ready());
}

/// Test application title
#[test]
fn test_app_title() {
    let app = App::new();
    assert_eq!(app.title(), "ABOP");
}

/// Test theme selection
#[test]
fn test_app_theme() {
    let app = App::new();
    let theme = app.theme();
    assert_eq!(theme, IcedTheme::Light);
}

/// Test task management
#[tokio::test]
async fn test_task_management() {
    let (tx, mut rx) = unbounded_channel();
    
    // Test task spawning
    let task_id = app.spawn_task("test_task", async { 
        tokio::time::sleep(Duration::from_millis(100)).await;
    });
    
    assert!(task_id.is_some());
    
    // Test task cancellation
    let cancelled = app.cancel_task(task_id.unwrap());
    assert!(cancelled);
    
    // Verify task cancellation message
    if let Some(msg) = rx.recv().await {
        match msg {
            TaskMessage::Cancel(id) => assert_eq!(id, task_id.unwrap()),
            _ => panic!("Unexpected message type"),
        }
    } else {
        panic!("No message received");
    }
}

/// Test application drop
#[test]
fn test_app_drop() {
    let app = App::new();
    drop(app);
    // Should not panic
}

/// Test message handling
#[tokio::test]
async fn test_message_handling() {
    let mut app = App::new();
    
    // Test navigation message
    let task = app.update(Message::Navigate(Route::Settings));
    assert!(task.is_ready());
    assert_eq!(app.state.current_route(), &Route::Settings);
    
    // Test theme change
    let task = app.update(Message::ChangeTheme(Theme::Dark));
    assert!(task.is_ready());
    assert_eq!(app.theme(), IcedTheme::Dark);
}

/// Test view rendering
#[test]
fn test_view_rendering() {
    let app = App::new();
    let view = app.view();
    assert!(!view.is_empty());
}

/// Test subscription handling
#[test]
fn test_subscriptions() {
    let app = App::new();
    let subscription = app.subscription();
    assert!(!subscription.is_none());
}
