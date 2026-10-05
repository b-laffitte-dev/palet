FROM rust:1.85 AS build
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=build /app/target/release/palet-backend /usr/local/bin/palet-backend
CMD ["palet-backend"]
