#!/usr/bin/env bash

usage() {
  echo "Usage: $0 [-c] component-name"
  echo "  -c           : Use golem cloud"
  echo "  -e           : Environment (default: test)"
}

USE_CLOUD="${USE_CLOUD:-false}"
ENVIRONMENT="${ENVIRONMENT:-test}"

while getopts "ce:" opt; do
  case $opt in
    c)
      USE_CLOUD=true
      ;;
    e)
      ENVIRONMENT="$OPTARG"
      ;;
    *)
      usage
      exit 1
      ;;
  esac
done

shift $((OPTIND - 1))

if [ "$#" -ne 1 ]; then
  usage
  exit 1
fi

COMPONENT_NAME="$1"

if [ "$USE_CLOUD" = 'true' ]; then
  CMD=golem-cloud-cli
else
  CMD=golem-cli
fi

source .env

"$CMD" api-deployment deploy --id "$COMPONENT_NAME" \
  --version "0.0.1" \
  --host "sputnik.dev:${WORKER_SERVICE_CUSTOM_REQUEST_PORT}" \
  --subdomain "${ENVIRONMENT}.${COMPONENT_NAME}"