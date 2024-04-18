#!/usr/bin/env bash
usage() {
  echo "Usage: $0 [-c] template-name"
  echo "  -c           : Use golem cloud"
  echo "  template-name: Template to update"
}

USE_CLOUD=false

while getopts ":c" opt; do
  case $opt in
    c)
      USE_CLOUD=true
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

TEMPLATE_NAME="$1"

if [ "$USE_CLOUD" = 'true' ]; then
  CMD=golem-cloud-cli
else
  CMD=golem-cli
fi

TEMPLATE_NAME_W_UNDERSCORE=$(echo "$TEMPLATE_NAME" | tr '-' '_')
TEMPLATE_COMPOSED_FILENAME=target/wasm32-wasi/debug/"${TEMPLATE_NAME_W_UNDERSCORE}"_composed.wasm
if [ -f "$TEMPLATE_COMPOSED_FILENAME" ]; then
  TEMPLATE_FILENAME="$TEMPLATE_COMPOSED_FILENAME"
else
  TEMPLATE_FILENAME=target/wasm32-wasi/debug/"${TEMPLATE_NAME_W_UNDERSCORE}".wasm
fi

"$CMD" template update --template-name "${TEMPLATE_NAME}" "${TEMPLATE_FILENAME}"