FROM ubuntu:18.04
RUN apt update && apt install curl build-essential libclang-dev clang git -y
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

RUN . $HOME/.cargo/env && \
    rustup install nightly && \
    rustup target add wasm32-unknown-unknown --toolchain nightly

COPY . .
RUN . $HOME/.cargo/env && \  
      cargo build --release





#ADD /blockchain_soul/ReAlis-Network/target/release/realis /realis/realis0
#ENTRYPOINT ["bash","entrypoint.prod.sh"]tttt
#ADD ./realis /realis/realis
#RUN chmod +x /realis/realis
#RUN apt-get update
#RUN apt-get install ca-certificates -y
#RUN update-ca-certificates
#ADD ./realis.json /realis/realis.json
#WORKDIR /realis/chain
#EXPOSE 9944 9044 9033
