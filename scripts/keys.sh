#!/bin/bash

pushd .

# The following line ensure we run from the project root
PROJECT_ROOT=`git rev-parse --show-toplevel`
cd $PROJECT_ROOT

curl http://localhost:9936  -H "Content-Type:application/json;charset=utf-8" -d "@scripts/keys/babe1"
curl http://localhost:9937  -H "Content-Type:application/json;charset=utf-8" -d "@scripts/keys/babe2"
curl http://localhost:9938  -H "Content-Type:application/json;charset=utf-8" -d "@scripts/keys/babe3"
curl http://localhost:9939  -H "Content-Type:application/json;charset=utf-8" -d "@scripts/keys/babe4"
curl http://localhost:9940  -H "Content-Type:application/json;charset=utf-8" -d "@scripts/keys/babe5"

curl http://localhost:9936  -H "Content-Type:application/json;charset=utf-8" -d "@scripts/keys/gran1"
curl http://localhost:9937  -H "Content-Type:application/json;charset=utf-8" -d "@scripts/keys/gran2"
curl http://localhost:9938  -H "Content-Type:application/json;charset=utf-8" -d "@scripts/keys/gran3"
curl http://localhost:9939  -H "Content-Type:application/json;charset=utf-8" -d "@scripts/keys/gran4"
curl http://localhost:9940  -H "Content-Type:application/json;charset=utf-8" -d "@scripts/keys/gran5"

curl http://localhost:9936  -H "Content-Type:application/json;charset=utf-8" -d "@scripts/keys/imol1"
curl http://localhost:9937  -H "Content-Type:application/json;charset=utf-8" -d "@scripts/keys/imol2"
curl http://localhost:9938  -H "Content-Type:application/json;charset=utf-8" -d "@scripts/keys/imol3"
curl http://localhost:9939  -H "Content-Type:application/json;charset=utf-8" -d "@scripts/keys/imol4"
curl http://localhost:9940  -H "Content-Type:application/json;charset=utf-8" -d "@scripts/keys/imol5"

curl http://localhost:9936  -H "Content-Type:application/json;charset=utf-8" -d "@scripts/keys/audi1"
curl http://localhost:9937  -H "Content-Type:application/json;charset=utf-8" -d "@scripts/keys/audi2"
curl http://localhost:9938  -H "Content-Type:application/json;charset=utf-8" -d "@scripts/keys/audi3"
curl http://localhost:9939  -H "Content-Type:application/json;charset=utf-8" -d "@scripts/keys/audi4"
curl http://localhost:9940  -H "Content-Type:application/json;charset=utf-8" -d "@scripts/keys/audi5"

popd