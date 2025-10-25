#!/usr/bin/env bash
set -euo pipefail

cargo package --allow-dirty
cargo publish --allow-dirty


