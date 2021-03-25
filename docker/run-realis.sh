#!/bin/bash
IMAGE="daelon02/realis-network"
CONTAINER="realis-node"
docker rm -f ${CONTAINER}
docker rmi ${IMAGE}
docker build -t ${IMAGE} .
docker push ${IMAGE}
docker run -d --name=${CONTAINER} --net=host \
 -v /blockchain_soul/soul/node1:/realis/chain \
${IMAGE} \
/realis/realis \
--chain=realis \
--name MyNode01 \
--rpc-cors '*' \
-d /realis/chain
