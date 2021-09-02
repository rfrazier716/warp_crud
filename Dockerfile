FROM lukemathwalker/cargo-chef:latest-rust-1.53.0 AS chef
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json

# Build application
COPY . .
RUN cargo build --release --bin warp_crud

# We do not need the Rust toolchain to run the binary!
FROM ubuntu:20.04 AS runtime

# install libssl
RUN apt-get update
RUN apt-get install -y libssl-dev

WORKDIR app
# Copy Over Config files
COPY --from=builder /app/static/ /app/static/
COPY --from=builder /app/config/ /app/config/ 
# Copy the executable
COPY --from=builder /app/target/release/warp_crud /usr/local/bin

# Set ENV Variables and entrypoint
ENV RUN_ENV=Production
ENTRYPOINT ["/usr/local/bin/warp_crud"]