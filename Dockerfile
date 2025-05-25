FROM rust:1.87.0-slim AS builder

RUN apt update && apt install lld clang -y
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim AS runtime

# Install OpenSSL - it is dynamically linked by some of our dependencies
# Install ca-certificates - it is needed to verify TLS certificates
# when establishing HTTPS connections
RUN apt-get update -y \
	&& apt-get install -y --no-install-recommends openssl ca-certificates \
	# Clean up
	&& apt-get autoremove -y \
	&& apt-get clean -y \
	&& rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/byte_genius_hosting byte_genius_hosting
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./byte_genius_hosting"]
