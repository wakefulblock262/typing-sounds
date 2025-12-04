use std::io::{self, BufRead, Read, Write};
use serde_json::Value;
use std::error::Error;
use std::io::Cursor;
use rodio::{Decoder, OutputStream, Source};

// Embed the sound file into the binary so it's portable
const SOUND_BYTES: &[u8] = include_bytes!("../../osu-hit-sound.mp3");

fn main() -> Result<(), Box<dyn Error>> {
    eprintln!("Starting typing-sounds-server...");

    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut stdout = io::stdout();

    // Initialize audio output once
    let (_stream, stream_handle) = OutputStream::try_default()?;

    // We use a Sink to manage playback, though for rapid typing sounds,
    // creating a new source for each event is often snappier.
    // However, keeping the stream handle alive is crucial.

    // Pre-decode the sound
    let cursor = Cursor::new(SOUND_BYTES);
    let source = Decoder::new(cursor)?;
    let sample_rate = source.sample_rate();
    let channels = source.channels();
    let samples: Vec<f32> = source.convert_samples().collect();

    loop {
        let mut content_length = 0;
        loop {
            let mut line = String::new();
            let bytes_read = stdin.read_line(&mut line)?;
            if bytes_read == 0 {
                return Ok(());
            }
            if line.trim().is_empty() {
                break;
            }
            if line.starts_with("Content-Length: ") {
                if let Ok(len) = line.trim()["Content-Length: ".len()..].parse() {
                    content_length = len;
                }
            }
        }

        if content_length == 0 {
            continue;
        }

        let mut buf = vec![0; content_length];
        stdin.read_exact(&mut buf)?;
        let msg = String::from_utf8(buf)?;
        let value: Value = match serde_json::from_str(&msg) {
            Ok(v) => v,
            Err(_) => continue,
        };

        if let Some(method) = value.get("method").and_then(|m| m.as_str()) {
            match method {
                "initialize" => {
                    let response = serde_json::json!({
                        "jsonrpc": "2.0",
                        "id": value["id"],
                        "result": {
                            "capabilities": {
                                "textDocumentSync": 1
                            }
                        }
                    });
                    send_response(&mut stdout, response)?;
                }
                "textDocument/didChange" => {
                    // Play the sound
                    let sound = rodio::buffer::SamplesBuffer::new(channels, sample_rate, samples.clone());
                    if let Err(e) = stream_handle.play_raw(sound) {
                        eprintln!("Error playing sound: {}", e);
                    }
                }
                "shutdown" => {
                     let response = serde_json::json!({
                        "jsonrpc": "2.0",
                        "id": value["id"],
                        "result": null
                    });
                    send_response(&mut stdout, response)?;
                }
                "exit" => {
                    break;
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn send_response(stdout: &mut std::io::Stdout, value: Value) -> Result<(), Box<dyn Error>> {
    let msg = serde_json::to_string(&value)?;
    write!(stdout, "Content-Length: {}\r\n\r\n{}", msg.len(), msg)?;
    stdout.flush()?;
    Ok(())
}
