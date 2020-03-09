FROM rust as build-env
WORKDIR /app
ADD . /app
RUN rustup default nightly
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=build-env /app/target/release/fizz_buzz /
CMD ["./fizz_buzz"]
EXPOSE 8000
