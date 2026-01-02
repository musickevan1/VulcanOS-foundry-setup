# VulcanOS Speech-to-Text (S2T)

A privacy-focused, offline speech-to-text system integrated into VulcanOS. Use your voice to type text into any application.

---

## Overview

VulcanOS S2T provides system-wide dictation that works with any application. It uses:

- **whisper-overlay**: Wayland-native overlay that displays during recording
- **python-faster-whisper**: Fast offline transcription (no cloud or internet required)
- **wtype**: Wayland-compatible text injection
- **PipeWire**: Direct audio capture

### Key Features

- **Privacy-First**: All processing happens offline on your machine
- **Global Dictation**: Works in terminals, browsers, editors, and GUI apps
- **Hold-to-Speak**: Default mode - hold the hotkey, release to transcribe
- **Configurable**: Adjust model, language, audio device, and more
- **Waybar Integration**: Visual status indicator showing recording state
- **Wofi Settings Menu**: Easy configuration with familiar UI

### What You Can Do

- Dictate emails, messages, and documents
- Type code snippets and commands
- Add notes to any text field
- Work in multiple languages
- Switch between typing directly or copying to clipboard

---

## Quick Start

### 1. Check Dependencies

```bash
vulcan-s2t-deps
```

This checks if all required packages are installed. If anything is missing, it will show installation instructions.

### 2. Start S2T

```bash
vulcan-s2t start
```

This starts the transcription server and the dictation client. The first time you run it, the Whisper model will download (~70-500MB depending on size).

### 3. Start Dictating

Hold **Super + `** (grave accent) and start speaking. When you release the key, your speech will be transcribed and typed into the active window.

### 4. Stop S2T

```bash
vulcan-s2t stop
```

Stops both the server and client.

---

## Command Reference

### Control Commands

| Command | Description |
|---------|-------------|
| `vulcan-s2t start` | Start S2T server and client |
| `vulcan-s2t stop` | Stop S2T server and client |
| `vulcan-s2t restart` | Restart S2T system |
| `vulcan-s2t status` | Show current S2T status |
| `vulcan-s2t toggle` | Toggle S2T on/off |

### Settings Commands

| Command | Description |
|---------|-------------|
| `vulcan-s2t settings` | Open Wofi settings menu |
| `vulcan-s2t set-model <model>` | Change Whisper model |
| `vulcan-s2t set-hotkey <key>` | Change activation hotkey |
| `vulcan-s2t set-language <code>` | Change recognition language |
| `vulcan-s2t set-audio-device <device>` | Change audio input device |
| `vulcan-s2t set-output-mode <mode>` | Change output mode (type/clipboard) |
| `vulcan-s2t set-debug <true|false>` | Toggle debug mode |
| `vulcan-s2t audio-devices` | List available audio devices |

### Utility Commands

| Command | Description |
|---------|-------------|
| `vulcan-s2t help` | Show help message |
| `vulcan-s2t-deps` | Check and show dependencies |

### Advanced Commands

| Command | Description |
|---------|-------------|
| `vulcan-s2t-server status` | Check server status only |
| `vulcan-dictation` | Launch client directly (server must be running) |

---

## Keyboard Shortcuts

### S2T Shortcuts

| Shortcut | Action |
|----------|--------|
| `Super + `` | Start dictation (hold to speak, release to transcribe) |
| `Super + Shift + `` | Open S2T settings menu |
| `Super + Ctrl + `` | Toggle S2T on/off |

### Waybar Interactions

- **Click**: Open settings menu
- **Right-click**: Toggle S2T

---

## Settings Explanation

### Model Size (S2T_MODEL)

Controls the Whisper AI model used for transcription. Larger models are more accurate but slower.

| Model | Size | Speed | Accuracy | Download | Use Case |
|-------|------|-------|----------|----------|----------|
| `tiny` | ~40MB | Fastest | Low | Good | Quick dictation, clear speech |
| `base` | ~75MB | Fast | Low-Medium | Good | General use, quiet environment |
| `small` (default) | ~250MB | Medium | Good | Good | Balanced performance |
| `medium` | ~760MB | Slower | Very Good | Longer | Professional dictation |
| `large-v3` | ~1.5GB | Slowest | Best | Long | Highest accuracy needed |

**Recommended for T2 MacBook Pro**: `small` (balanced) or `tiny` (faster)

### Hotkey (S2T_HOTKEY)

The key combination to activate dictation.

| Key Value | Shortcut |
|------------|----------|
| `KEY_GRAVE` (default) | Super + ` |
| `KEY_F12` | Super + F12 |
| `KEY_F11` | Super + F11 |
| `KEY_F10` | Super + F10 |

### Language (S2T_LANGUAGE)

Sets the recognition language. Empty string uses auto-detect.

**Common codes**:
- `en` - English
- `es` - Spanish
- `fr` - French
- `de` - German
- `it` - Italian
- `pt` - Portuguese
- `ru` - Russian
- `zh` - Chinese
- `ja` - Japanese
- `ko` - Korean

Leave empty to auto-detect language.

### Audio Device (S2T_AUDIO_DEVICE)

Selects which microphone to use. Use `vulcan-s2t audio-devices` to list available devices.

- `default` - Use system default microphone
- `alsa_input.pci-0000_00_1f.3.analog-stereo` - Example specific device

### Output Mode (S2T_OUTPUT_MODE)

Controls how transcribed text is delivered.

| Mode | Description |
|------|-------------|
| `type` (default) | Types text directly into the active window |
| `clipboard` | Copies text to clipboard for manual pasting |

### Port (S2T_PORT)

Network port for the transcription server. Default is `7007`.

### Debug Mode (S2T_DEBUG)

Enables verbose logging for troubleshooting.

- `false` (default) - Normal operation, minimal logging
- `true` - Verbose logging to `~/.local/share/vulcan-s2t/server.log`

---

## Performance Tips for T2 Hardware

### Recommended Configuration

The T2 MacBook Pro has limited CPU power for AI processing. For best results:

```bash
vulcan-s2t set-model small
vulcan-s2t set-language en
```

### Model Selection

- **Best Balance**: `small` model - Good accuracy, acceptable speed
- **Fastest**: `tiny` model - Quick transcription, lower accuracy
- **Avoid**: `large-v3` model - Too slow for real-time use

### Language Setting

If you primarily speak one language, set it explicitly for faster transcription:

```bash
vulcan-s2t set-language en
```

Auto-detect (`S2T_LANGUAGE=""`) is slower because the model checks all languages.

### Audio Quality

- Use the built-in microphone for best quality
- Speak clearly and at a moderate pace
- Avoid background noise when possible
- Check audio device: `vulcan-s2t audio-devices`

### System Load

S2T works best when:
- Few applications are open
- CPU is not heavily loaded
- System is not under thermal stress

---

## Troubleshooting

### S2T won't start

**Problem**: `vulcan-s2t start` fails with an error.

**Solutions**:
1. Check dependencies: `vulcan-s2t-deps`
2. Check logs: `cat ~/.local/share/vulcan-s2t/server.log`
3. Enable debug mode: `vulcan-s2t set-debug true` and try again
4. Verify Whisper model downloaded: `ls ~/.cache/huggingface/hub/`

### No text appears when I speak

**Problem**: Recording works but nothing is typed.

**Solutions**:
1. Check Waybar status - is S2T in "ready" or "recording" state?
2. Try clipboard mode: `vulcan-s2t set-output-mode clipboard`
3. Verify audio input: `pactl list sources short`
4. Test audio recording: `arecord -f cd -d 5 test.wav && aplay test.wav`
5. Check wtype is installed: `which wtype`

### Transcription is inaccurate

**Problem**: Text appears but has many errors.

**Solutions**:
1. Upgrade to larger model: `vulcan-s2t set-model medium`
2. Set language explicitly: `vulcan-s2t set-language en`
3. Improve audio quality:
   - Reduce background noise
   - Speak closer to microphone
   - Speak at a moderate pace
   - Set correct audio device: `vulcan-s2t audio-devices`

### Transcription is slow

**Problem**: Takes too long to transcribe after releasing the hotkey.

**Solutions**:
1. Use smaller model: `vulcan-s2t set-model tiny`
2. Set language explicitly: `vulcan-s2t set-language en`
3. Check CPU usage: `htop` (should be high during transcription)
4. Close other applications to free CPU

### Hotkey doesn't work

**Problem**: Pressing the hotkey does nothing.

**Solutions**:
1. Check hotkey is configured: `vulcan-s2t status`
2. Try alternative hotkey: `vulcan-s2t set-hotkey KEY_F12`
3. Verify S2T is running: `vulcan-s2t status`
4. Check Hyprland bindings: `cat ~/.config/hypr/bindings.conf | grep -i s2t`
5. Restart Hyprland: `super + shift + q` then `super + return`

### Waybar icon shows idle

**Problem**: Icon doesn't change from idle state.

**Solutions**:
1. Check if S2T is running: `vulcan-s2t status`
2. Restart Waybar: `killall waybar && waybar &`
3. Check Waybar config: `cat ~/.config/waybar/config.jsonc | grep -A10 custom/s2t`

### Audio device issues

**Problem**: Can't hear or select correct microphone.

**Solutions**:
1. List devices: `vulcan-s2t audio-devices`
2. Set device explicitly: `vulcan-s2t set-audio-device alsa_input.pci-...`
3. Check PipeWire: `pactl info`
4. Restart PipeWire: `systemctl --user restart pipewire wireplumber`

### Model download issues

**Problem**: First start hangs or fails downloading model.

**Solutions**:
1. Check internet connection
2. Ensure disk space: `df -h`
3. Manual download location: `~/.cache/huggingface/hub/`
4. Try specific model:
   ```bash
   export HF_HUB_CACHE=~/.cache/huggingface/hub
   python3 -c "from faster_whisper import WhisperModel; WhisperModel('tiny', download_root='$HF_HUB_CACHE')"
   ```

### Server crashes

**Problem**: Server exits unexpectedly.

**Solutions**:
1. Check logs: `cat ~/.local/share/vulcan-s2t/server.log`
2. Enable debug mode: `vulcan-s2t set-debug true`
3. Verify model exists: `ls ~/.cache/huggingface/hub/`
4. Check for memory issues: `free -h`
5. Restart: `vulcan-s2t restart`

### Text appears in wrong location

**Problem**: Text types but not where expected.

**Solutions**:
1. Make sure window has focus before dictating
2. Click into text field before starting
3. Try clipboard mode and paste manually
4. Check wtype compatibility with application

### Touch Bar not responding during dictation

**Problem**: Touch Bar becomes unresponsive.

**Solutions**:
1. This is a known limitation with tiny-dfr and audio recording
2. Workaround: Use external keyboard shortcuts
3. The Touch Bar should recover after dictation completes

---

## Advanced Usage

### Manual Server Management

You can control the server separately from the client:

```bash
# Start only the server
vulcan-s2t-server start

# Check server status
vulcan-s2t-server status

# Stop server
vulcan-s2t-server stop
```

### Custom Audio Configuration

For non-standard audio setups, you can specify device paths:

```bash
# List all audio sources
pactl list sources short

# Set specific device
vulcan-s2t set-audio-device "alsa_input.pci-0000_00_1f.3.analog-stereo"
```

### Scripting S2T

Integrate S2T into your workflows:

```bash
#!/bin/bash
# Start S2T, do work, then stop
vulcan-s2t start
# Do your dictation work...
vulcan-s2t stop
```

---

## Configuration Files

All S2T configuration is stored in your home directory:

```
~/.config/vulcan-s2t/
├── settings.conf       # User settings (model, hotkey, etc.)
└── server.conf        # Server settings (port, audio config)

~/.local/share/vulcan-s2t/
├── server.pid        # Server process ID
├── client.pid        # Client process ID
└── server.log       # Server logs (when debug enabled)
```

Edit these files directly or use the `vulcan-s2t set-*` commands.

---

## Getting Help

- **Documentation**: See `docs/S2T-INSTALL.md` for installation details
- **Status Check**: `vulcan-s2t status`
- **Debug Mode**: `vulcan-s2t set-debug true` for verbose logs
- **Logs**: `~/.local/share/vulcan-s2t/server.log`

---

## Technical Details

### Architecture

```
whisper-overlay (client)
    ↓ (WebSocket)
realtime-stt-server.py (backend)
    ↓
faster-whisper (AI engine)
    ↓
Audio from PipeWire
```

### Dependencies

- **whisper-overlay** - Wayland-native recording overlay
- **python-faster-whisper** - Fast Whisper implementation
- **wtype** - Wayland text injection
- **PipeWire** - Audio capture
- **Hyprland** - Window compositor integration

### License

S2T uses:
- Whisper models (MIT license)
- faster-whisper (MIT license)
- whisper-overlay (MIT license)
