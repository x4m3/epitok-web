FROM ekidd/rust-musl-builder:stable as builder

# build depedencies first, that way they will be cached
# there is a very high chance that project's files will be updated rather than dependencies

# note: this will be useful if you build it locally, on CI it builds from scratch every time.
RUN USER=rust cargo new --bin epitok-web
WORKDIR ./epitok-web
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*

# build actual project
ADD src/ ./src/
ADD templates/ ./templates/
RUN rm ./target/x86_64-unknown-linux-musl/release/deps/epitok_web*
RUN cargo build --release

# now we can prepare to run the project
FROM alpine:latest

ARG APP_PATH=/usr/src/app

# copy binary to final folder
COPY --from=builder /home/rust/src/epitok-web/target/x86_64-unknown-linux-musl/release/epitok-web ${APP_PATH}/epitok-web

WORKDIR ${APP_PATH}

# add static folder required for runtime
ADD static/ ./static/

EXPOSE 4343
CMD ["./epitok-web"]