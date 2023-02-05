#!/bin/bash

typesDir="./frontend/src/lib"
typesFile="types.ts"
apiUrl="https://kr.filipton.space/api"
currDir="$(pwd)"

cp "$typesDir/$typesFile" "$typesDir/$typesFile.old"
echo "export const apiUrl = \"$apiUrl\";" > "$typesDir/$typesFile"
cat "$typesDir/$typesFile.old" | tail -n+2 >> "$typesDir/$typesFile"

cd "$currDir/frontend"
bash ./arm-deploy.sh

cd "$currDir/keyracer_server"
bash ./arm-deploy.sh

cd "$currDir"
mv "$typesDir/$typesFile.old" "$typesDir/$typesFile"
