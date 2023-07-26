#!/bin/bash

docker run --rm --privileged multiarch/qemu-user-static --reset -p yes
buildx create --name multiarch --driver docker-container --use
buildx inspect --bootstrap
