###
FROM ubuntu:18.04 as builder
RUN apt update && apt install curl build-essential libclang-dev clang git -y
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

RUN . $HOME/.cargo/env && \
    rustup install nightly && \
    rustup target add wasm32-unknown-unknown --toolchain nightly

COPY . .
RUN . $HOME/.cargo/env && \  
      cargo build --release

###

FROM ubuntu:20.04
RUN apt-get update && apt-get install ca-certificates -y && update-ca-certificates

ARG NODENAME=REALIS-NODE
ARG RESERVEDNODES=/ip4/65.21.121.115/tcp/30333/p2p/12D3KooWHdKTMJdPQjofKFnFXYANZE8eheuEq2uQgmqVdNAYXnVa

ENV NODENAME=$NODENAME
ENV RESERVEDNODES=$RESERVEDNODES


RUN mkdir -p /realis-blockchain/data
WORKDIR /realis-blockchain
COPY realis.json /realis-blockchain/realis.json
COPY --from=builder ./target/release/realis /realis-blockchain/realis

ENTRYPOINT ["/bin/bash", "-c", \
            "/realis-blockchain/realis \
            --chain=/realis-blockchain/realis.json \
            --reserved-nodes ${RESERVEDNODES} \
            --ws-port=9944 \
            --rpc-port=9933 \
            --validator \
            --rpc-methods=Unsafe \
            --name=${NODENAME} \
            --unsafe-ws-external \
            --unsafe-rpc-external \
            --rpc-cors='*' \
            --base-path=/realis-blockchain/data"]
