#!/bin/bash

pushd .

# The following line ensure we run from the project root
PROJECT_ROOT=`git rev-parse --show-toplevel`
cd $PROJECT_ROOT

docker run -ti --rm \
    -v /opt/thx-network/data:/data \
    fstnjp/substrate bash -c 'substrate build-spec --disable-default-bootnode --chain staging > /data/customSpec.json'

docker run -ti --rm \
    -v /opt/thx-network/data:/data \
    fstnjp/substrate bash -c 'substrate build-spec --chain=/data/customSpec.json --raw --disable-default-bootnode > data/customSpecRaw.json'

popd