#!/bin/bash
IMAGE="daelon02/realis-network"
CONTAINER="realis-test"
docker rm -f ${CONTAINER}
docker rmi ${IMAGE}
docker build -t ${IMAGE} .
docker push ${IMAGE}
docker run -d --name=${CONTAINER} --net=host \
 -v /blockchain_soul/soul/nikita1:/realis/chain \
${IMAGE} \
/realis/realis \
--chain ../realis.json \
--ws-port 9944 \
--rpc-port 9933  \
--validator  \
--rpc-methods=Unsafe  \
--reserved-nodes /ip4/161.97.142.255/tcp/30333/p2p/12D3KooWAAd19rb8AQhpnybsYYwDk5gaL9GGWZFJ4QrcXqrLM8w3 \
--name MyNode01 \
--unsafe-ws-external \
--unsafe-rpc-external \
--rpc-cors '*' \
-d /realis/chain
