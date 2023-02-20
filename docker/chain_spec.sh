#!/bin/bash

pushd .

# The following line ensure we run from the project root
PROJECT_ROOT=`git rev-parse --show-toplevel`
cd $PROJECT_ROOT

docker run -ti --rm \
    -v /opt/data/nodes:/data \
    257042641977.dkr.ecr.us-west-2.amazonaws.com/thxnet-node bash -c 'substrate build-spec --disable-default-bootnode --chain staging > /data/customSpec.json'

docker run -ti --rm \
    -v /opt/data/nodes:/data \
    257042641977.dkr.ecr.us-west-2.amazonaws.com/thxnet-node bash -c 'substrate build-spec --chain=/data/customSpec.json --raw --disable-default-bootnode > data/customSpecRaw.json'

popd