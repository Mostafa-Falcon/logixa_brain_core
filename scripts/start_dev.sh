#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
if [ ! -f brain_config.toml ]; then
  cp brain_config.example.toml brain_config.toml
  echo "Created brain_config.toml from example. Edit model_path and llama_server_binary before real inference."
fi
cargo run --bin logixa-brain-daemon
