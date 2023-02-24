#!/usr/bin/env bash
set -e

pushd .

# The following line ensure we run from the project root
PROJECT_ROOT=`git rev-parse --show-toplevel`
cd $PROJECT_ROOT

# Install srtool
type srtool
unalias srtool ||
cargo install --git https://github.com/chevdor/srtool-cli

time srtool build --package thxnet-runtime  --runtime-dir node/runtime .
mkdir -p -- target/runtime
rsync -avh node/runtime/target/srtool/release/wbuild/thxnet-runtime target/runtime
rm -r node/runtime/target

popd