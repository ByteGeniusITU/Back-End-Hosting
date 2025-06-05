FROM lukemathwalker/cargo-chef:latest-rust-1.87.0-slim AS chef

WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef AS planner
COPY . .
## Compute lock-like file for our project
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build our project dependecies
RUN cargo chef cook --release --recipe-path recipe.json
# If our dependecy tree stays the same, all layers should be cached
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim AS runtime

# Install OpenSSL - it is dynamically linked by some of our dependencies
# Install ca-certificates - it is needed to verify TLS certificates
# when establishing HTTPS connections
RUN apt-get update -y \
	&& apt-get install -y --no-install-recommends openssl ca-certificates \
	# Install Kubectl
	&& apt-get install -y curl \
	&& curl -LO "https://dl.k8s.io/release/$(curl -L -s https://dl.k8s.io/release/stable.txt)/bin/linux/amd64/kubectl" \
	&& chmod +x kubectl \
	&& mv ./kubectl /usr/local/bin/kubectl \
	# Install Helm
	&& curl -fsSL -o get_helm.sh https://raw.githubusercontent.com/helm/helm/main/scripts/get-helm-3 \
	&& chmod 700 get_helm.sh \
	&& ./get_helm.sh \
	&& helm repo add mc-charts https://itzg.github.io/minecraft-server-charts/ \
	# Clean up
	&& apt-get autoremove -y \
	&& apt-get clean -y \
	&& rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/byte_genius_hosting byte_genius_hosting
ENV APP_ENVIRONMENT=production
ENTRYPOINT ["./byte_genius_hosting"]
