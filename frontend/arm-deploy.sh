#!/bin/bash

buildx build --platform linux/arm64 --tag filipton/keyracer-client:latest --push -f arm.Dockerfile .
