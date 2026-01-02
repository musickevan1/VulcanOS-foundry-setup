# VulcanOS Speech-to-Text - Installation Guide

This guide walks you through installing and configuring the VulcanOS Speech-to-Text system on your VulcanOS machine.

---

## Prerequisites

Before installing S2T, ensure you have:

### System Requirements

- **Operating System**: VulcanOS or Arch Linux with Wayland
- **Compositor**: Hyprland (recommended) or other Wayland compositor
- **Audio**: PipeWire running and configured
- **CPU**: T2 MacBook Pro or equivalent x86_64 processor
- **RAM**: Minimum 4GB (8GB+ recommended)
- **Disk Space**: 2GB+ for models

### Required Packages

| Package | Purpose | Source |
|---------|---------|---------|
| `python-faster-whisper` | STT engine | AUR |
| `rust` | Rust toolchain | Official |
| `cargo` | Rust package manager | Official |
| `whisper-overlay` | Wayland overlay client | AUR |
| `wtype` | Wayland text input | Official |
| `python` | Python runtime | Official |
| `pipewire` | Audio capture | Official (pre-installed) |
| `wireplumber` | PipeWire session manager | Official (pre-installed) |

### Optional Packages

| Package | Purpose |
|---------|---------|
| `notify-send` | Desktop notifications (libnotify) |
| `pavucontrol` | Audio configuration GUI |

---

## Installation

### Step 1: Install Official Packages

First, install the packages available from the official repositories:

```bash
sudo pacman -S python wtype
```

### Step 2: Install Rust Toolchain

Install Rust and Cargo:

```bash
sudo pacman -S rust cargo
```

Verify installation:

```bash
rustc --version
cargo --version
```

### Step 3: Install python-faster-whisper (AUR)

The `python-faster-whisper` package is available from the AUR. Use yay or your preferred AUR helper:

```bash
# Using yay
yay -S python-faster-whisper

# Using paru
paru -S python-faster-whisper

# Manual installation
git clone https://aur.archlinux.org/python-faster-whisper.git
cd python-faster-whisper
makepkg -si
```

Verify installation:

```bash
python -c "import faster_whisper; print(faster_whisper.__version__)"
```

### Step 4: Install whisper-overlay (AUR)

The `whisper-overlay` package provides the Wayland-native overlay for dictation:

```bash
# Using yay
yay -S whisper-overlay

# Using paru
paru -S whisper-overlay

# Manual installation
git clone https://aur.archlinux.org/whisper-overlay.git
cd whisper-overlay
makepkg -si
```

Verify installation:

```bash
whisper-overlay --help
```

### Step 5: Download Server Script

The `realtime-stt-server.py` script is required and will be downloaded automatically on first run. However, you can pre-download it:

```bash
mkdir -p ~/.local/share/vulcan-s2t
cd ~/.local/share/vulcan-s2t
wget https://raw.githubusercontent.com/realtime-stt/realtime-stt/main/server/realtime-stt-server.py
chmod +x realtime-stt-server.py
```

### Step 6: Install VulcanOS S2T Scripts

If you're building VulcanOS from source, the S2T scripts are included in `dotfiles/scripts/` and will be installed via stow. If you're installing on an existing system:

```bash
# Clone or copy scripts to ~/.local/bin
git clone https://github.com/your-repo/VulcanOS.git
cd VulcanOS/dotfiles/scripts

# Make scripts executable
chmod +x vulcan-s2t vulcan-s2t-server vulcan-dictation vulcan-s2t-settings vulcan-s2t-deps

# Copy to bin
cp vulcan-s2t* ~/.local/bin/
```

Or use stow if you have the full VulcanOS dotfiles:

```bash
cd /path/to/VulcanOS/dotfiles
stow scripts
```

---

## First-Time Configuration

### Initialize Configuration

Run the dependency checker:

```bash
vulcan-s2t-deps
```

This will:
- Check if all required packages are installed
- Show missing dependencies with installation commands
- Verify audio system is available

### Create Configuration Files

Configuration files are automatically created on first run. If you want to customize them beforehand:

```bash
# Create config directory
mkdir -p ~/.config/vulcan-s2t

# Copy default settings
cp /path/to/dotfiles/vulcan-s2t/.config/vulcan-s2t/settings.conf.default ~/.config/vulcan-s2t/settings.conf
cp /path/to/dotfiles/vulcan-s2t/.config/vulcan-s2t/server.conf.default ~/.config/vulcan-s2t/server.conf
```

### Configure Audio Device

Check available audio devices:

```bash
vulcan-s2t audio-devices
```

Set your preferred device:

```bash
vulcan-s2t set-audio-device default
```

### Choose Model Size

For T2 MacBook Pro, the `small` model is recommended:

```bash
vulcan-s2t set-model small
```

### Set Language (Optional)

If you primarily speak one language:

```bash
vulcan-s2t set-language en
```

---

## Verification

### Test Dependencies

```bash
vulcan-s2t-deps
```

All checks should pass (green).

### Test Audio

Check audio capture works:

```bash
# Record test audio
arecord -f cd -d 5 test.wav

# Playback
aplay test.wav

# Clean up
rm test.wav
```

### Test Server Start

Start the server (without client):

```bash
vulcan-s2t-server start
```

Check status:

```bash
vulcan-s2t-server status
```

You should see:
- Server running
- Listening on localhost:7007

Stop the server:

```bash
vulcan-s2t-server stop
```

### Test Full System

Start the complete S2T system:

```bash
vulcan-s2t start
```

This will:
1. Download the Whisper model (first time only, ~70-500MB)
2. Start the transcription server
3. Start the dictation overlay client

Once started:
- Press and hold `Super + ``
- Speak clearly
- Release the key
- Text should appear in your terminal

Stop the system:

```bash
vulcan-s2t stop
```

---

## Troubleshooting Installation

### "Command not found" errors

If `vulcan-s2t` is not found:

```bash
# Check if scripts are in PATH
echo $PATH | grep local/bin

# If not, add to PATH
export PATH="$HOME/.local/bin:$PATH"

# Make permanent (add to ~/.bashrc)
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
```

### AUR package fails to build

If `python-faster-whisper` or `whisper-overlay` fails:

```bash
# Update system first
sudo pacman -Syu

# Update AUR helper
yay -Syu

# Clean build directory
cd /path/to/package
makepkg -si --clean
```

### Rust not found

If `cargo` command fails:

```bash
# Install rust via pacman
sudo pacman -S rust cargo

# Or use rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### Python import errors

If `import faster_whisper` fails:

```bash
# Reinstall package
pip uninstall faster-whisper
yay -S python-faster-whisper

# Check Python version
python --version  # Should be 3.10+

# Try system-wide installation
sudo pip install faster-whisper
```

### whisper-overlay not found

If `whisper-overlay` command doesn't work:

```bash
# Check if installed
which whisper-overlay

# Reinstall
yay -S whisper-overlay --rebuild

# Or build manually
git clone https://aur.archlinux.org/whisper-overlay.git
cd whisper-overlay
makepkg -si
```

### PipeWire not running

Check audio system:

```bash
# Check PipeWire status
pactl info

# If not running, start services
systemctl --user start pipewire
systemctl --user start pipewire-pulse
systemctl --user start wireplumber

# Enable on boot
systemctl --user enable pipewire pipewire-pulse wireplumber
```

### Model download fails

If model doesn't download:

```bash
# Check internet connection
ping -c 3 huggingface.co

# Set HuggingFace cache directory
export HF_HUB_CACHE=~/.cache/huggingface/hub
mkdir -p "$HF_HUB_CACHE"

# Try manual download
python3 -c "from faster_whisper import WhisperModel; WhisperModel('tiny', download_root='$HF_HUB_CACHE')"
```

---

## Configuration Reference

### settings.conf

Located at `~/.config/vulcan-s2t/settings.conf`:

```bash
S2T_MODEL="small"              # Whisper model size
S2T_HOTKEY="KEY_GRAVE"        # Activation hotkey
S2T_LANGUAGE=""               # Language code (empty = auto)
S2T_HOST="localhost"          # Server address
S2T_PORT="7007"               # Server port
S2T_AUDIO_DEVICE="default"     # Audio device name
S2T_OUTPUT_MODE="type"        # Output: type or clipboard
S2T_DEBUG="false"             # Enable debug logging
```

### server.conf

Located at `~/.config/vulcan-s2t/server.conf`:

```bash
S2T_MODEL_SIZE="small"        # Model size (syncs with settings)
S2T_DEVICE="auto"             # Audio device (usually "auto")
S2T_LANGUAGE=""               # Language (syncs with settings)
S2T_SAMPLE_RATE=16000         # Audio sample rate
S2T_CHANNELS=1               # Audio channels
S2T_LOG_LEVEL="info"         # Logging level
```

---

## Uninstallation

To completely remove S2T:

### 1. Stop S2T

```bash
vulcan-s2t stop
```

### 2. Remove Packages

```bash
yay -R python-faster-whisper
yay -R whisper-overlay
sudo pacman -R wtype
```

Keep `python` and `rust` if you use them for other purposes.

### 3. Remove Configuration Files

```bash
rm -rf ~/.config/vulcan-s2t
rm -rf ~/.local/share/vulcan-s2t
```

### 4. Remove Scripts

```bash
rm ~/.local/bin/vulcan-s2t*
rm ~/.local/bin/vulcan-dictation
```

### 5. Remove Models (Optional)

To reclaim disk space:

```bash
rm -rf ~/.cache/huggingface/hub/
```

---

## Updating

### Update AUR Packages

```bash
yay -Syu
# Or:
yay -S python-faster-whisper --rebuild
yay -S whisper-overlay --rebuild
```

### Update VulcanOS Scripts

If using stow:

```bash
cd /path/to/VulcanOS
git pull
cd dotfiles
stow -R scripts
```

Or manually copy updated scripts:

```bash
cp /path/to/VulcanOS/dotfiles/scripts/vulcan-s2t* ~/.local/bin/
chmod +x ~/.local/bin/vulcan-s2t*
```

---

## Next Steps

- Read the [User Guide](S2T.md) for usage instructions
- Configure [Hyprland bindings](../KEYBINDINGS.md#speech-to-text)
- Set up [Waybar integration](../docs/S2T.md#advanced-usage)
- Check [Troubleshooting](../docs/S2T.md#troubleshooting) for common issues

---

## Support

For issues or questions:

1. Check the [Troubleshooting](../docs/S2T.md#troubleshooting) section
2. Enable debug mode: `vulcan-s2t set-debug true`
3. Review logs: `cat ~/.local/share/vulcan-s2t/server.log`
4. Check status: `vulcan-s2t status`

---

## Credits

- **Whisper**: OpenAI's speech recognition system
- **faster-whisper**: Optimized Whisper implementation
- **whisper-overlay**: Wayland-native dictation overlay
- **VulcanOS**: Integration and configuration
