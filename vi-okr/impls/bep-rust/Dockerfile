FROM rust:1.76.0-slim-bookworm as build

# copy build support
COPY src /server/src
COPY Cargo.lock /server/Cargo.lock
COPY Cargo.toml /server/Cargo.toml
COPY Rocket.toml /server/Rocket.toml

# build
WORKDIR /server
RUN ["cargo", "build", "--release"]


FROM debian:12.5

ENV DATA_DIRECTORY=/data
ENV ROCKET_PROFILE=production

EXPOSE 8000
VOLUME /data

RUN apt -y update && apt -y install curl

WORKDIR /server
RUN useradd -ms /bin/bash app
RUN chown -R app:app /server
USER app

COPY --chown=app:app --from=build /server/target/release/bep-rust /server/server
COPY Rocket.toml /server/Rocket.toml

WORKDIR "/server"

CMD ["./server"]
