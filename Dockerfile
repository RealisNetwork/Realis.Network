FROM ubuntu:18.04 as builder

# Install any needed packages
RUN apt-get update

WORKDIR /ReAlis-Network/target/release/
COPY . .

RUN ./target/release/realis --chain ./realis.json --ws-port 9944  --rpc-port 9933  --validator  --rpc-methods=Unsafe  --listen-addr /ip4/0.0.0.0/tcp/30333 --name MyNode01 --unsafe-ws-external --unsafe-rpc-external --rpc-cors '*' -d ../soul/nikita

# ===========================================================
FROM nginx:stable-alpine

# The following is mainly for doc purpose to show which ENV is supported
ENV WS_URL=

WORKDIR /usr/share/nginx/html

COPY env.sh .

RUN apk add --no-cache bash; chmod +x env.sh

COPY docker/nginx.conf /etc/nginx/nginx.conf

EXPOSE 9944

CMD ["/bin/bash", "-c", "/usr/share/nginx/html/env.sh && nginx -g \"daemon off;\""]
