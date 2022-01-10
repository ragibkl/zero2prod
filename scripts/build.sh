#!/usr/bin/env bash
set -x
set -eo pipefail

IMAGE=ragibkl/zero2prod
TAG=$IMAGE:$(date -u +%Y%m%d_%H%M%S)

docker build --pull -t $IMAGE .
docker push $IMAGE

docker tag $IMAGE $TAG
docker push $TAG
