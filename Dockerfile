FROM rust:1.53.0

# Do our building in an app dir
WORKDIR /app 

# COPY Project into container directory
COPY . .

RUN cargo build --release

ENTRYPOINT ["./target/release/warp_crud"]