FROM rust:1.78.0 AS build

RUN apt-get update && \
    apt-get install -y protobuf-compiler && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /opt/app

COPY .env ./.env
COPY grpc-service ./grpc-service
COPY rest-service ./rest-service
COPY shared ./shared
COPY Cargo.toml ./Cargo.toml
COPY Cargo.lock ./Cargo.lock

RUN cargo build --release

FROM rust:1.78.0

WORKDIR /opt/app

COPY --from=build /opt/app/target/release .
COPY run-app.sh .
CMD ["/bin/sh", "run-app.sh"]