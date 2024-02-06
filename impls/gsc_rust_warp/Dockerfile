FROM ubuntu:23.10 as build

RUN apt-get update
RUN yes | apt-get install gcc zlib1g-dev build-essential wget curl
RUN apt-get update

# Get Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

ADD . /build

WORKDIR /build

RUN [ "cargo", "build", "--release"]

FROM ubuntu:23.10

RUN yes | apt-get update

EXPOSE 2326

COPY --from=build build/.env   /.env
COPY --from=build build/target/release/gsc_rust_warp   /gsc_rust_warp
