# Builder image
FROM --platform=$BUILDPLATFORM rust:alpine AS build
WORKDIR /app/
ARG TARGETARCH

# Setup musl compiler
RUN apk add --no-cache curl make perl musl-dev && \
    ARCH=$(echo $TARGETARCH | sed 's/arm64/aarch64/;s/amd64/x86_64/') && \
    cd /tmp/ && \
    curl -sSLv -o $ARCH-linux-musl-cross.tgz https://musl.cc/$ARCH-linux-musl-cross.tgz && \
    tar -xzvf /tmp/$ARCH-linux-musl-cross.tgz

# Setup rust toolchain
RUN \
    ARCH=$(echo $TARGETARCH | sed 's/arm64/aarch64/;s/amd64/x86_64/') && \
    rustup target add $ARCH-unknown-linux-musl

# Fetch cargo dependencies
ADD Cargo.toml Cargo.lock ./
RUN cargo fetch --locked

# Build
ADD ./src ./src
RUN \
    ARCH=$(echo $TARGETARCH | sed 's/arm64/aarch64/;s/amd64/x86_64/') && \
    export TARGET_CC=/tmp/$ARCH-linux-musl-cross/bin/$ARCH-linux-musl-gcc && \
    export RUSTFLAGS="-Clink-self-contained=yes -Clinker=rust-lld" && \
    cargo build --locked --release --target=$ARCH-unknown-linux-musl && \
    mv ./target/$ARCH-unknown-linux-musl/release/api .

# Runner image
FROM --platform=$TARGETPLATFORM scratch
ENV PORT=8000
EXPOSE $PORT
COPY --from=build /app/api .
CMD ["/api"]
