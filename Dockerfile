FROM rust:latest as builder
WORKDIR /usr/src/note-taking
COPY . .
RUN cargo install --path .
FROM debian:buster-slim
RUN apt-get update & apt-get install -y extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/note-taking /usr/local/bin/note-taking
WORKDIR /app
COPY ./static /app/static
CMD ["note-taking"]
