FROM rust:latest as builder

ARG PORT=8080
ENV PORT=${PORT}
ENV WORKING_DIR=/rust_docker_web


RUN USER=root cargo new --bin $WORKING_DIR
WORKDIR $WORKING_DIR
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./


RUN rm $WORKING_DIR/target/release/deps$WORKING_DIR*
RUN cargo build --release


FROM debian:buster-slim
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE $PORT

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder $WORKING_DIR/target/release$WORKING_DIR ${APP}$WORKING_DIR

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD [${WORKING_DIR}]