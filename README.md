# Typing Sounds for Zed

This is a Zed extension that plays a sound when you type.

## Setup

1. **Build the Server**:
   You need to compile the language server binary.
   ```bash
   cd server
   cargo build --release
   ```

2. **Add to PATH**:
   Add the `server/target/release` directory to your PATH, or copy the `typing-sounds-server` binary to a location in your PATH.
   
   ```bash
   # Example for Linux/macOS
   cp server/target/release/typing-sounds-server /usr/local/bin/ 
   # For Windows, add the folder to your Environment Variables
   ```

3. **Install the Extension**:
   - Open Zed.
   - Go to Extensions.
   - Click "Install Dev Extension" and select the `typing-sounds` directory.

## Configuration

Currently, the sound is a placeholder. You need to modify `server/src/main.rs` to play your desired sound file.

