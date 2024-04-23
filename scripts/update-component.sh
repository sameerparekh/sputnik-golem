#!/usr/bin/env bash
usage() {
  echo "Usage: $0 [-c] component-name"
  echo "  -c           : Use golem cloud"
  echo "  -a           : Add component"
  echo "  component-name: Component to update"
}

USE_CLOUD="${USE_CLOUD:-false}"
ADD_OR_UPDATE="update"

while getopts ":ca" opt; do
  case $opt in
    c)
      USE_CLOUD=true
      ;;
    a)
      ADD_OR_UPDATE="add"
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

COMPONENT_NAME_W_UNDERSCORE=$(echo "$COMPONENT_NAME" | tr '-' '_')
COMPONENT_COMPOSED_FILENAME=target/wasm32-wasi/debug/"${COMPONENT_NAME_W_UNDERSCORE}"_composed.wasm
if [ -f "$COMPONENT_COMPOSED_FILENAME" ]; then
  COMPONENT_FILENAME="$COMPONENT_COMPOSED_FILENAME"
else
  COMPONENT_FILENAME=target/wasm32-wasi/debug/"${COMPONENT_NAME_W_UNDERSCORE}".wasm
fi

"$CMD" component "$ADD_OR_UPDATE" --component-name "${COMPONENT_NAME}" "${COMPONENT_FILENAME}"