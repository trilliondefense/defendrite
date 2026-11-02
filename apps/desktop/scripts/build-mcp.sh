
#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../../.." && pwd)"

RESOURCE_DIR="$ROOT_DIR/apps/desktop/src-tauri/resources/filesystem"

cd "$ROOT_DIR"

HOST_TARGET="$(rustc -vV | sed -n 's/^host: //p')"

echo "Building filesystem MCP server for $HOST_TARGET"

case "$HOST_TARGET" in
    *windows*)
        PLATFORM="windows"
        BINARY="filesystem.exe"
        ;;
    *apple-darwin*)
        PLATFORM="macos"
        BINARY="filesystem"
        ;;
    *linux*)
        PLATFORM="linux"
        BINARY="filesystem"
        ;;
    *)
        echo "Unsupported target: $HOST_TARGET"
        exit 1
        ;;
esac

DEST="$RESOURCE_DIR/$PLATFORM/$BINARY"

# Build MCP server.
cargo build --release -p filesystem --target "$HOST_TARGET"

# Prepare resource directory.
mkdir -p "$(dirname "$DEST")"

# Copy executable.
cp "$ROOT_DIR/target/$HOST_TARGET/release/$BINARY" "$DEST"

# Ensure executable permissions on Unix.
if [[ "$PLATFORM" != "windows" ]]; then
    chmod +x "$DEST"
fi

echo "MCP filesystem server ready:"
echo "$DEST"
