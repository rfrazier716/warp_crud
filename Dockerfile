FROM rust:1.53.0

# Do our building in an app dir
WORKDIR /app 

# COPY Project into container directory
COPY . .

# We want the app to be configured for production
ENV RUN_ENV=production

# Build in Release Mode
RUN cargo build --release

# THe Entry point should start the app
ENTRYPOINT ["./target/release/warp_crud"]