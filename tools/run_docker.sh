#!/bin/bash

cd "$(dirname $0)/.."
docker build -t app .
docker run -p 8080:8080 -e SECRET="$(cat tools/.local_secret)" app
