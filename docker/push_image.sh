#!/bin/bash
set -e

if [ "$#" -lt 1 ]; then
	echo "Please provide the image ID!"
	exit 1
fi

IMAGE_ID=$1
TAG=$2
if [ -z "$TAG" ]; then
	TAG="latest"
fi

docker tag $IMAGE_ID 257042641977.dkr.ecr.us-west-2.amazonaws.com/thx_node:$TAG
docker push 257042641977.dkr.ecr.us-west-2.amazonaws.com/thx_node:$TAG