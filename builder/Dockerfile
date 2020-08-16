FROM rust:1.42

ARG rust_version="1.42.0"

ENV WSB_WORKSPACE="/workspace"

WORKDIR $WSB_WORKSPACE
COPY init.sh $WSB_WORKSPACE
RUN ./init.sh
