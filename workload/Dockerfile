FROM alpine:latest

ENV ROCKET_PROFILE=release

COPY ./responder /app/responder
COPY ./Rocket.toml /app/Rocket.toml

WORKDIR /app
ENTRYPOINT ["/app/responder"]