#!/usr/bin/env bash

usage() {
  echo "Usage: $0 [-c] component-name"
  echo "  -c           : Use golem cloud"
  echo "  -e           : Environment (default: test)"
}

USE_CLOUD="${USE_CLOUD:-false}"
ENVIRONMENT="${ENVIRONMENT:-test}"

while getopts ":c:e" opt; do
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

COMPONENT_ID=$("$CMD" --format yaml component list -c "$COMPONENT_NAME" | yq '.[0].componentId')
"$CMD" api-deployment deploy --id "$COMPONENT_ID" --version "0.0.1" --host localhost --subdomain "$ENVIRONMENT.$COMPONENT_NAME"