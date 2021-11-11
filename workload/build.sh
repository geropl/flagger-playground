#!/bin/bash

set -euxo pipefail

CWD=$(cd $(dirname $BASH_SOURCE) && pwd)

cargo build --release

DIR=$(mktemp -d)
cp $CWD/../target/x86_64-unknown-linux-musl/release/responder $DIR/responder
cp $CWD/responder/Rocket.toml $DIR/Rocket.toml
cp $CWD/Dockerfile $DIR/Dockerfile

docker build $DIR -t geropl/responder:latest