#!/bin/bash

set -e

APP=$1

EXPORT_PATH=$(pwd)/docker/$APP/export

docker build --rm \
  --tag ${APP}_builder \
  --file "$(pwd)"/docker/$APP.ecs.builder.Dockerfile \
  .

echo "Pack exporting "$APP" to "$EXPORT_PATH

docker run --rm \
  --network=docker_compose_benchmarks \
  -v ${SSH_AUTH_SOCK}:/ssh-agent \
  -v "$(pwd)"/docker/$APP/export:/export \
  -v "$(pwd)"/docker/$APP/target:/build/target:rw \
  -v "$(pwd)"/docker/$APP/cargo/git:/usr/local/cargo/git:rw \
  -v "$(pwd)"/docker/$APP/cargo/registry:/usr/local/cargo/registry:rw \
  -v ${EXPORT_PATH}:/export \
  -e SSH_AUTH_SOCK=/ssh-agent \
  ${APP}_builder

echo "Finished docker run"

ls $EXPORT_PATH

docker build --rm \
  --tag benchmarks/${APP} \
  --file "$(pwd)"/docker/$APP.ecs.runner.Dockerfile \
  .
