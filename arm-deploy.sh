#!/bin/bash

read -p "Enter the version tag: " version

echo "Deploying as version $version..."
bash ./frontend/arm-deploy.sh $version
bash ./keyracer_server/arm-deploy.sh $version
