#!/bin/bash

buildx build --platform linux/arm64 --tag filipton/keyracer-client:latest --load -f arm.Dockerfile .

docker image push filipton/keyracer-client:latest
docker image tag filipton/keyracer-client:latest "filipton/keyracer-client:$1"
docker image push "filipton/keyracer-client:$1"
