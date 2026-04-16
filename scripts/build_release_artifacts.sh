#!/usr/bin/env bash
set -euo pipefail

TAG="${1:-v0.1.0}"
ARCH="${2:-linux-x86_64}"
BIN_NAME="metal"
RELEASE_DIR="releases/${TAG}"
BIN_PATH="target/release/${BIN_NAME}"
ARTIFACT_BIN="${RELEASE_DIR}/${BIN_NAME}-${ARCH}"
ARTIFACT_TAR="${RELEASE_DIR}/${BIN_NAME}-${TAG}-${ARCH}.tar.gz"

cargo build --release

mkdir -p "${RELEASE_DIR}"
cp "${BIN_PATH}" "${ARTIFACT_BIN}"
tar -C "${RELEASE_DIR}" -czf "${ARTIFACT_TAR}" "${BIN_NAME}-${ARCH}"
sha256sum "${ARTIFACT_BIN}" "${ARTIFACT_TAR}" > "${RELEASE_DIR}/SHA256SUMS.txt"

echo "Built release artifacts in ${RELEASE_DIR}:"
echo "- ${ARTIFACT_BIN}"
echo "- ${ARTIFACT_TAR}"
echo "- ${RELEASE_DIR}/SHA256SUMS.txt"
