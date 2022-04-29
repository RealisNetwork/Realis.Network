###
FROM ubuntu:18.04 as builder
RUN apt update && apt install curl build-essential libclang-dev clang git -y
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

RUN . $HOME/.cargo/env && \
    rustup install nightly-2021-10-20 && \
    rustup target add wasm32-unknown-unknown --toolchain nightly-2021-10-20

COPY . .
RUN . $HOME/.cargo/env && \
      cargo build --release

###

FROM ubuntu:20.04
RUN apt-get update && apt-get install ca-certificates -y && update-ca-certificates

ARG NODENAME=REALIS-NODE
ENV NODENAME=$NODENAME

RUN mkdir -p /realis-blockchain/data
WORKDIR /realis-blockchain
COPY realis.json /realis-blockchain/realis.json
COPY --from=builder ./target/release/realis /realis-blockchain/realis

ENTRYPOINT ["/bin/bash", "-c", \
            "/realis-blockchain/realis \
            --dev \
            --tmp \
            --ws-port=9943 \
            --rpc-port=9922 \
            --validator \
            --rpc-methods=Unsafe \
            --listen-addr /ip4/0.0.0.0/tcp/30333 \
            --name=${NODENAME} \
            --unsafe-ws-external \
            --unsafe-rpc-external \
            --rpc-cors='*'"]
