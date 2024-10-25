# Use NEAR maintained image as a base
FROM nearprotocol/contract-builder:latest

# Setup tooling
# RUN rustup toolchain install stable-2024-07-25 && \
#     rustup default stable-2024-07-25 && \
#     rustup target add wasm32-unknown-unknown
RUN rustup toolchain install stable-2024-07-25 && \
    rustup default stable-2024-07-25 && \
    rustup target add wasm32-unknown-unknown

# Define working directory (instead of root)
WORKDIR /builder

# Copy source code to the builder directory
COPY . /builder

# Define custom flags for Rust compiler
ENV RUSTFLAGS="-C link-arg=-s"

# Define custom cargo home directory
ENV CARGO_HOME=/var/cache/cargo

# Define a volume for built artifacts and dependency cache
VOLUME "/var/cache/cargo"

# Define a volume for the output artifacts
VOLUME "/output"

CMD cargo build --target wasm32-unknown-unknown --release --workspace --exclude tests && \
    cp target/wasm32-unknown-unknown/release/exchange_contract.wasm /output/contract.wasm
