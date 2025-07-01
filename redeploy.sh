#!/usr/bin/env bash

set -e  # Exit on error
set -o pipefail

SERVICE_NAME="dvs-backend"
PROJECT_DIR="/root/dvs-backend"
BIN_PATH="/usr/local/bin/${SERVICE_NAME}"
BUILD_OUTPUT="${PROJECT_DIR}/target/release/${SERVICE_NAME}"

cd "$PROJECT_DIR"

echo "📥 Pulling latest changes from git..."
git pull

echo "🛠 Building Rust project..."
cargo build --release

if [ -f "$BUILD_OUTPUT" ]; then
    echo "✅ Build succeeded."
else
    echo "❌ Build failed. Aborting deployment."
    exit 1
fi

echo "🚚 Moving binary to $BIN_PATH..."
rm "$BIN_PATH"
sudo mv "$BUILD_OUTPUT" "$BIN_PATH"
sudo chmod +x "$BIN_PATH"

echo "🔄 Restarting systemd service: $SERVICE_NAME"
sudo systemctl daemon-reexec
sudo systemctl daemon-reload
sudo systemctl restart "$SERVICE_NAME"
sudo systemctl enable "$SERVICE_NAME"

echo "✅ Deployment complete. Tailing logs..."
journalctl -u "$SERVICE_NAME" -f
