#!/bin/bash

pushd .

# The following line ensure we run from the project root
PROJECT_ROOT=`git rev-parse --show-toplevel`
cd $PROJECT_ROOT

./target/release/substrate build-spec --disable-default-bootnode --chain staging > data/customSpec.json
./target/release/substrate build-spec --chain=data/customSpec.json --raw --disable-default-bootnode > data/customSpecRaw.json

popd