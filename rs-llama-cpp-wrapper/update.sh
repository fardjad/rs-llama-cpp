#!/usr/bin/env bash

set -e
cd "$(dirname "$0")"

echo "Updating llama.cpp..."
pushd llama.cpp &> /dev/null
git fetch origin master
git reset --hard origin/master
git checkout master -f
popd &> /dev/null

echo "Updating Generator..."
npm ci &> /dev/null
npx ncu
npm i &> /dev/null
npm audit fix --quiet --no-progress --no-fund &> /dev/null || true

echo "Building rs-llama-cpp-wrapper..."
npm run --silent generate
npm run --silent format
rm -rf build
mkdir build
pushd build &> /dev/null
cmake .. &> /dev/null
make -j$(nproc) > /dev/null
popd &> /dev/null

echo "All done!"