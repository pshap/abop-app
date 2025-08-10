//! Scan progress subscription wiring

use iced::{Subscription, time};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use crate::messages::Message;

// Global progress state and versioning
static PROGRESS_STATE: Lazy<Mutex<Option<abop_core::scanner::ScanProgress>>> =
    Lazy::new(|| Mutex::new(None));
static PROGRESS_VERSION: AtomicU64 = AtomicU64::new(0);
static PROGRESS_DELIVERED: AtomicU64 = AtomicU64::new(0);

/// Publish a scan progress event; stored globally and picked up by the subscription tick
pub fn publish(progress: abop_core::scanner::ScanProgress) {
    if let Ok(mut guard) = PROGRESS_STATE.lock() {
        *guard = Some(progress);
        PROGRESS_VERSION.fetch_add(1, Ordering::SeqCst);
    }
}

/// Subscription that polls for new progress at a fixed interval and emits updates
pub fn subscription() -> Subscription<Message> {
    time::every(Duration::from_millis(200)).map(|_| {
        let v = PROGRESS_VERSION.load(Ordering::Relaxed);
        let delivered = PROGRESS_DELIVERED.load(Ordering::Relaxed);
        if v != delivered {
            if let Ok(guard) = PROGRESS_STATE.lock() {
                if let Some(progress) = guard.clone() {
                    PROGRESS_DELIVERED.store(v, Ordering::Relaxed);
                    return Message::ScanProgress(progress);
                }
            }
        }
        Message::NoOp
    })
}
