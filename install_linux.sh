#!/bin/bash
set -e

HOST_NAME="com.focussentinel"
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TARGET_DIR="$DIR/host/target/release"
EXE_PATH="$TARGET_DIR/focus_sentinel_host"
MANIFEST_PATH="$DIR/host/com.focussentinel.json"
DEST_DIR="$HOME/.config/google-chrome/NativeMessagingHosts"

# Ensure build exists
if [ ! -f "$EXE_PATH" ]; then
    echo "Warning: Executable not found at $EXE_PATH. Did you run 'cargo build --release'?"
fi

# Create Manifest
cat > "$MANIFEST_PATH" <<EOF
{
  "name": "$HOST_NAME",
  "description": "FocusSentinel Native Host",
  "path": "$EXE_PATH",
  "type": "stdio",
  "allowed_origins": [
    "chrome-extension://jbclhaqbgpcmilmjjpldpcknneegmfgk/"
  ]
}
EOF

# Install Manifest
mkdir -p "$DEST_DIR"
cp "$MANIFEST_PATH" "$DEST_DIR/$HOST_NAME.json"

echo "Native Host registered at $DEST_DIR/$HOST_NAME.json"
echo "Make sure to update 'allowed_origins' with your actual Extension ID."
