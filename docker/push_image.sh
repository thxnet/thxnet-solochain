#!/bin/bash
set -e

TAG=$1

if [ "$#" -ne 1 ]; then
	TAG="latest"
fi

GITUSER=257042641977.dkr.ecr.us-west-2.amazonaws.com
GITREPO=thxnet-node

aws ecr get-login-password --region us-west-2 | docker login --username AWS --password-stdin 257042641977.dkr.ecr.us-west-2.amazonaws.com

echo Push $GITUSER/$GITREPO:$TAG
docker push $GITUSER/$GITREPO:$TAG