FROM rust:1.78.0 AS build

WORKDIR /opt/app

COPY migrations ./migrations
COPY src ./src
COPY .env ./.env
COPY Cargo.toml ./Cargo.toml
COPY Cargo.lock ./Cargo.lock
COPY diesel.toml ./diesel.toml

RUN cargo build --release

FROM rust:1.78.0

WORKDIR /opt/app

COPY --from=build /opt/app/target/release .

CMD [ "./country-service" ]