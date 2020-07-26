#!/bin/bash

set -euo pipefail

timeout=${PLAYGROUND_TIMEOUT:-10}

# TODO update cargo.toml according to env variables!

# Don't use `exec` here. The shell is what prints out the useful
# "Killed" message
timeout --signal=KILL "${timeout}" "$@"
