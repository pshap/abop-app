//! Audio processing: converting audiobooks to mono WAV copies
//!
//! Conversion streams through each file (see
//! [`abop_core::audio::processing::streaming`]), so memory use stays small even
//! for multi-hour audiobooks. Originals are never modified.

use std::path::{Path, PathBuf};

use abop_core::audio::processing::streaming::{convert_to_mono_wav, estimate_mono_wav_bytes};
use abop_core::models::Audiobook;
use abop_core::utils::casting::safe_progress;
use futures::{SinkExt, Stream};

/// Progress and completion events emitted while converting
#[derive(Debug, Clone, PartialEq)]
pub enum ProcessingEvent {
    /// Overall progress (0.0–1.0) and a description of the current step
    Progress(f32, String),
    /// Conversion finished: a summary, or an error if nothing was converted
    Finished(Result<String, String>),
}

/// Sample rate assumed when estimating output size (most audiobooks use 44.1 kHz)
const ESTIMATE_SAMPLE_RATE: u32 = 44_100;

/// Where the mono copy of `input` is written: `"<name> (mono).wav"` alongside it
#[must_use]
pub fn mono_output_path(input: &Path) -> PathBuf {
    let stem = input
        .file_stem()
        .map_or_else(|| "audiobook".into(), |s| s.to_string_lossy());
    input.with_file_name(format!("{stem} (mono).wav"))
}

/// Rough total size of the mono WAV copies for the given audiobooks
#[must_use]
pub fn estimate_output_bytes<'a>(books: impl IntoIterator<Item = &'a Audiobook>) -> u64 {
    books
        .into_iter()
        .map(|book| {
            estimate_mono_wav_bytes(book.duration_seconds.unwrap_or(0), ESTIMATE_SAMPLE_RATE)
        })
        .sum()
}

/// Convert each audiobook to a mono WAV copy, reporting progress as a stream
pub fn convert_to_mono(books: Vec<Audiobook>) -> impl Stream<Item = ProcessingEvent> {
    iced::stream::channel(16, move |mut output| async move {
        let total = books.len().max(1);
        let mut converted = 0;
        let mut failures = Vec::new();

        for (index, book) in books.into_iter().enumerate() {
            let title = book.title.clone().unwrap_or_else(|| book.id.clone());
            let label = format!("Converting “{title}” ({}/{total})", index + 1);
            let base = safe_progress(index, total).unwrap_or(0.0);
            let step = safe_progress(1, total).unwrap_or(1.0);
            let _ = output
                .send(ProcessingEvent::Progress(base, label.clone()))
                .await;

            // Forward per-file progress from the blocking worker
            let (progress_tx, mut progress_rx) = tokio::sync::mpsc::unbounded_channel::<f32>();
            let input = book.path.clone();
            let worker = tokio::task::spawn_blocking(move || {
                let output_path = mono_output_path(&input);
                convert_to_mono_wav(&input, &output_path, |p| {
                    let _ = progress_tx.send(p);
                })
            });
            while let Some(fraction) = progress_rx.recv().await {
                let overall = base + fraction * step;
                let _ = output
                    .send(ProcessingEvent::Progress(overall, label.clone()))
                    .await;
            }

            match worker.await {
                Ok(Ok(summary)) => {
                    log::info!("Wrote {}", summary.output.display());
                    converted += 1;
                }
                Ok(Err(error)) => failures.push(format!("{title}: {error}")),
                Err(join_error) => failures.push(format!("{title}: {join_error}")),
            }
        }

        for failure in &failures {
            log::error!("Mono conversion failed for {failure}");
        }
        let result = match (converted, failures.len()) {
            (0, 0) => Err("No audiobooks selected".to_string()),
            (0, _) => Err(format!("Conversion failed: {}", failures.join("; "))),
            (n, 0) => Ok(format!(
                "Created {n} mono cop{} — rescan to add to the library",
                if n == 1 { "y" } else { "ies" }
            )),
            (n, f) => Ok(format!(
                "Created {n} mono cop{}, {f} failed (see log)",
                if n == 1 { "y" } else { "ies" }
            )),
        };
        let _ = output.send(ProcessingEvent::Finished(result)).await;
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt;

    #[test]
    fn output_path_sits_next_to_input() {
        assert_eq!(
            mono_output_path(Path::new("/books/The Martian.m4b")),
            PathBuf::from("/books/The Martian (mono).wav")
        );
    }

    #[test]
    fn estimates_size_from_duration() {
        let mut book = Audiobook::new("lib", "/books/a.mp3");
        book.duration_seconds = Some(3600);
        // One hour of 16-bit mono at 44.1 kHz is about 318 MB
        let bytes = estimate_output_bytes([&book]);
        assert!((317_000_000..=318_000_000).contains(&bytes), "{bytes}");
    }

    #[tokio::test]
    async fn converts_fixture_and_reports_progress() {
        let dir = tempfile::tempdir().unwrap();
        let input = dir.path().join("Book.mp3");
        std::fs::copy(
            concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../abop-core/tests/fixtures/tagged.mp3"
            ),
            &input,
        )
        .unwrap();
        let mut book = Audiobook::new("lib", &input);
        book.title = Some("Book".into());

        let events: Vec<_> = convert_to_mono(vec![book]).collect().await;

        assert!(matches!(events.first(), Some(ProcessingEvent::Progress(p, _)) if *p == 0.0));
        match events.last() {
            Some(ProcessingEvent::Finished(Ok(message))) => assert!(message.contains("Created 1")),
            other => panic!("unexpected final event: {other:?}"),
        }
        assert!(dir.path().join("Book (mono).wav").exists());
    }

    #[tokio::test]
    async fn reports_failures() {
        let mut book = Audiobook::new("lib", "/nonexistent/Missing.mp3");
        book.title = Some("Missing".into());
        let events: Vec<_> = convert_to_mono(vec![book]).collect().await;
        match events.last() {
            Some(ProcessingEvent::Finished(Err(message))) => assert!(message.contains("Missing")),
            other => panic!("unexpected final event: {other:?}"),
        }
    }
}
