#!/usr/bin/env bash
set -eo pipefail

cargo hack check --feature-powerset --depth 1 \
  --group-features std,map --group-features std,map-fxhash --group-features std,map-indexmap \
  --ignore-unknown-features \
  "${@}"
