#!/bin/bash

docker buildx build --platform linux/amd64 --tag filipton/keyracer-client:latest --push -f x86.Dockerfile .
