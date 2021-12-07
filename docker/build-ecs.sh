#!/bin/bash

set -e

echo "starting build-ecs: $PROJECT"

CARGO_PROJECT=$PROJECT
BINARY=$PROJECT-bootstrap
cargo fetch
cargo build --release --tests --package $CARGO_PROJECT --bin $BINARY
cargo test --release -- --nocapture

echo "EXPORTING: "$PROJECT" to "$EXPORT_DIR

cp target/release/$BINARY $EXPORT_DIR/$PROJECT

ls $EXPORT_DIR

echo "completed build-ecs"
