#!/usr/bin/env bash
set -euo pipefail

TARGET_DIR="${1:-../metal-ide}"
mkdir -p "$TARGET_DIR"
rsync -a --delete metal-ide-repo/ "$TARGET_DIR/"

if [ ! -d "$TARGET_DIR/.git" ]; then
  git -C "$TARGET_DIR" init
  git -C "$TARGET_DIR" add .
  git -C "$TARGET_DIR" commit -m "Initial Metal IDE extension"
fi

echo "Standalone IDE repo prepared at: $TARGET_DIR"
