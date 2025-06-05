Symphonia 0.5 Guide
Introduction
Symphonia is a pure Rust library designed for audio decoding and media demuxing. It supports a wide array of formats such as AAC, ADPCM, AIFF, ALAC, CAF, FLAC, MKV, MP1, MP2, MP3, MP4, OGG, Vorbis, WAV, and WebM. Version 0.5 brings enhancements like additional codec support, improved performance, and a more ergonomic API.
Key Features

Decoding: Supports numerous audio codecs with gapless playback for seamless transitions.
Performance: Competes with or surpasses FFmpeg in speed on modern hardware.
Modularity: Separate crates for codecs and formats allow selective inclusion.
Safety: Built entirely in safe Rust, ensuring memory and type safety.

Symphonia 0.5 is ideal for projects ranging from simple audio players to complex multimedia applications.
Installation and Setup
Add Symphonia to your Rust project by including it in your Cargo.toml:
[dependencies]
symphonia = "0.5.4"

By default, it supports royalty-free codecs. To enable additional codecs or features, use feature flags:

MP3 support: features = ["mp3"]
All codecs: features = ["all"]
SIMD optimizations: features = ["opt-simd"]

Basic Usage
Here’s an example of decoding an audio file with Symphonia:
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::default::get_probe;
use std::fs::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("audio.mp3")?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());
    let hint = Hint::new();
    let probe = get_probe();
    let mut format = probe.format(&hint, mss, &FormatOptions::default(), &MetadataOptions::default())?;
    
    let track = format.default_track().unwrap();
    let mut decoder = symphonia::default::get_codecs().make(&track.codec_params, &Default::default())?;
    
    while let Ok(packet) = format.next_packet() {
        match decoder.decode(&packet) {
            Ok(decoded) => {
                if let Some(buffer) = decoded.as_audio_buffer_ref() {
                    for frame in 0..buffer.frames() {
                        for channel in 0..buffer.spec().channels.count() {
                            let sample = buffer.chan(channel)[frame];
                            // Process sample here
                        }
                    }
                }
            }
            Err(e) => eprintln!("Decode error: {}", e),
        }
    }
    Ok(())
}

This code opens an MP3 file, probes its format, selects the default track, and decodes audio packets into samples.
Advanced Features

Gapless Playback: Enable with enable_gapless: true in FormatOptions for uninterrupted playback.
Metadata: Access via format.metadata().current() to retrieve tags like artist or title.
Seeking: Use format.seek to navigate to specific points in the audio.

Example: Gapless Playback
let format_opts = FormatOptions {
    enable_gapless: true,
    ..Default::default()
};
let mut format = probe.format(&hint, mss, &format_opts, &MetadataOptions::default())?;

Best Practices

Memory: Utilize Symphonia’s zero-copy approach to reduce allocations.
Errors: Handle errors with Result and ? for robustness. Use a custom Error type for user-facing errors.
Performance: Enable opt-simd for faster decoding on supported hardware.

AudioBufferRef Handling: Always match all variants, including F64, to avoid non-exhaustive match errors.

Integration with Other Crates
Symphonia pairs well with libraries like Rodio for playback:
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use symphonia::core::io::MediaSourceStream;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("audio.mp3")?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());
    let decoder = Decoder::new(BufReader::new(mss))?;
    
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;
    sink.append(decoder);
    sink.sleep_until_end();
    Ok(())
}

## Troubleshooting
- If you encounter non-exhaustive match errors, update your match arms to include all AudioBufferRef variants.
- For decoding errors, ensure all required features are enabled in Cargo.toml.
- See [FIXES.md](../FIXES.md) for current outstanding issues.

Testing and Debugging

Testing: Use small audio files to verify decoding.
Tools: Leverage symphonia-play and symphonia-check for playback and validation.

Resources

Symphonia GitHub
Documentation
Examples

This guide provides a solid foundation for working with Symphonia 0.5 in Rust projects.
