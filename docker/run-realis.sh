#!/bin/bash
IMAGE="daelon02/realis-network"
CONTAINER="realis-test"
docker rm -f ${CONTAINER}
docker rmi ${IMAGE}
docker run -d --name=${CONTAINER} --net=host \
 -v /blockchain_soul/soul/nikita2:/realis/chain \
${IMAGE} \
/realis/realis \
--chain ./realis.json \
--ws-port 9945  \
--rpc-port 9934  \
--validator  \
--rpc-methods=Unsafe  \
--reserved-nodes /ip4/161.97.142.255/tcp/30333/p2p/12D3KooWGQjuxmkG2DYaSposPend484Xn3McG6Q7UQ5uRPU4Wox6 \
--name MyNode02 \
--unsafe-ws-external \
--unsafe-rpc-external \
--rpc-cors '*' \
-d /realis/chain
