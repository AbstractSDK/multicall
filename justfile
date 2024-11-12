# Install the tools that are used in this justfile
install-tools:
  cargo install cargo-nextest --locked || true
  cargo install taplo-cli --locked || true
  cargo install cargo-watch || true
  cargo install cargo-limit || true

## Development Helpers ##

# Test everything
test:
  cargo nextest run

watch-test:
  cargo watch -x "nextest run"

# Format your code and `Cargo.toml` files
fmt:
  cargo fmt --all
  find . -type f -iname "*.toml" -print0 | xargs -0 taplo format

lint:
  cargo clippy --all -- -D warnings

lintfix:
  cargo clippy --fix --allow-staged --allow-dirty --all-features
  just fmt

watch:
  cargo watch -x "lcheck --all-features"

juno-local:
  docker kill juno_node_1 || true
  docker volume rm -f junod_data || true
  docker run --rm -d \
    --name juno_node_1 \
    -p 1317:1317 \
    -p 26656:26656 \
    -p 26657:26657 \
    -p 9090:9090 \
    -e STAKE_TOKEN=ujunox \
    -e UNSAFE_CORS=true \
    --mount type=volume,source=junod_data,target=/root \
    ghcr.io/cosmoscontracts/juno:15.0.0 \
    ./setup_and_run.sh juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y # You can add used sender addresses here

wasm:
  #!/usr/bin/env bash

  # Delete all the current wasms first
  rm -rf ./artifacts/*.wasm

  if [[ $(arch) == "arm64" ]]; then
    image="cosmwasm/optimizer-arm64:0.16.0"
  else
    image="cosmwasm/optimizer:0.16.0"
  fi

  # Optimized builds
  docker run --rm -v "$(pwd)":/code \
    --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
    --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
    ${image}

## Exection commands ##

run-script script +CHAINS:
  cargo run --bin {{script}} --features="daemon-bin" -- --network-ids {{CHAINS}}

deploy +CHAINS:
  #!/usr/bin/env bash
  set -euxo pipefail

  if [ -d "artifacts" ]; then
    echo "Build found ✅";
  else
    just wasm
  fi
  just run-script deploy {{CHAINS}}