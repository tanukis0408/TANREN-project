#!/usr/bin/env bash
set -euo pipefail

TAG="${1:-v0.1.0}"
ARCH="${2:-linux-x86_64}"
RELEASE_DIR="releases/${TAG}"
BIN_FILE="${RELEASE_DIR}/metal-${ARCH}"
TAR_FILE="${RELEASE_DIR}/metal-${TAG}-${ARCH}.tar.gz"
SHA_FILE="${RELEASE_DIR}/SHA256SUMS.txt"
NOTES_FILE="${RELEASE_DIR}/RELEASE_NOTES.md"

if ! command -v gh >/dev/null 2>&1; then
  echo "gh CLI is not installed. Install it first: https://cli.github.com/"
  exit 1
fi

if [ ! -f "$BIN_FILE" ] || [ ! -f "$TAR_FILE" ] || [ ! -f "$SHA_FILE" ]; then
  echo "Release artifacts are missing for ${TAG}/${ARCH}."
  echo "Generate them first: ./scripts/build_release_artifacts.sh ${TAG} ${ARCH}"
  exit 1
fi

if [ ! -f "$NOTES_FILE" ]; then
  echo "Release notes not found: $NOTES_FILE"
  exit 1
fi

gh release create "$TAG" \
  "$BIN_FILE" \
  "$TAR_FILE" \
  "$SHA_FILE" \
  --notes-file "$NOTES_FILE" \
  --title "Metal $TAG"
