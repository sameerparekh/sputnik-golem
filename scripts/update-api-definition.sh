#!/usr/bin/env bash

usage() {
  echo "Usage: $0 [-c] component-name"
  echo "  -c           : Use golem cloud"
  echo "  -e           : Environment (default: test)"
  echo "  component-name: API to update"
}

#set -ex

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

COMPONENT_ID=$("$CMD" --format yaml component list -c "$COMPONENT_NAME" | yq '.[0].componentId')
MATCHING_ENGINE_ID=$("$CMD" --format yaml component list -c "matching-engine" | yq '.[0].componentId')
REGISTRY_ID=$("$CMD" --format yaml component list -c "registry" | yq '.[0].componentId')
ETHEREUMMONITOR_ID=$("$CMD" --format yaml component list -c "ethereummonitor" | yq '.[0].componentId')

TEMP_FILE=$(mktemp)
jsonnet -V component_id="$COMPONENT_ID" \
        -V matching_engine_id="$MATCHING_ENGINE_ID" \
        -V registry_id="$REGISTRY_ID" \
        -V environment="$ENVIRONMENT" \
        -V ethereummonitor_id="$ETHEREUMMONITOR_ID" \
        -o "$TEMP_FILE" \
        api/"$COMPONENT_NAME".jsonnet



COUNT=$("$CMD" --format yaml api-definition list --id "$COMPONENT_NAME" | yq length)
if [ "$COUNT" -eq 0 ]; then
  ADD_OR_UPDATE="add"
else
   ADD_OR_UPDATE="update"
fi

"$CMD" api-definition "$ADD_OR_UPDATE" "$TEMP_FILE"
rm "$TEMP_FILE"

