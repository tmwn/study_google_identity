#!/bin/bash

cd "$(dirname $0)/.." || exit

SECRET="$(cat ./tools/.local_secret)"
export SECRET
cargo watch -x 'run'
