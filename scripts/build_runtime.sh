#!/usr/bin/env bash
set -e

pushd .

# The following line ensure we run from the project root
PROJECT_ROOT=`git rev-parse --show-toplevel`
cd $PROJECT_ROOT

if ! [ -x "$(command -v srtool)" ]; then
  echo 'Error: srtool is not installed.' >&2
  cargo install --git https://github.com/chevdor/srtool-cli
fi

srtool pull
time srtool build --root --package thxnet-runtime --runtime-dir node/runtime .
mkdir -p -- target/runtime
rsync -avh node/runtime/target/srtool/release/wbuild/thxnet-runtime target/runtime

popd