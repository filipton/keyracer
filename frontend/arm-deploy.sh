#!/bin/bash

buildx build --platform linux/arm64 --tag "filipton/keyracer-client:$1" --push -f arm.Dockerfile .
docker push "filipton/keyracer-client:$1" filipton/keyracer-client:latest
