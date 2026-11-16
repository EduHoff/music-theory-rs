FROM rust:latest AS builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libfontconfig1-dev \
    binaryen \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install wasm-pack
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app

COPY Cargo.toml Cargo.lock build.rs ./
COPY src ./src
COPY index.html ./

RUN wasm-pack build --release --target web

FROM nginx:alpine

COPY --from=builder /app/pkg /usr/share/nginx/html/pkg
COPY --from=builder /app/index.html /usr/share/nginx/html/index.html

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]