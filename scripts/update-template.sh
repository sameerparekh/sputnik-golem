#!/usr/bin/env bash
TEMPLATE_NAME=$1

golem-cloud-cli template update --template-name "${TEMPLATE_NAME}" target/wasm32-wasi/debug/"${TEMPLATE_NAME}".wasm