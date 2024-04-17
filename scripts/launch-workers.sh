#!/usr/bin/env bash
usage() {
  echo "Usage: $0 [-c]"
  echo "  -c           : Use golem cloud"
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

if [ "$#" -ne 0 ]; then
  usage
  exit 1
fi

if [ "$USE_CLOUD" = 'true' ]; then
  CMD=golem-cloud-cli
else
  CMD=golem-cli
fi

"$CMD" worker add --template-name ids --worker-name ids
"$CMD" worker add --template-name registry --worker-name registry

IDS_TEMPLATE_ID=$("$CMD" template list -t ids | yq '.[0].templateId')
REGISTRY_TEMPLATE_ID=$("$CMD" template list -t registry | yq '.[0].templateId')
MATCHING_ENGINE_TEMPLATE_ID=$("$CMD" template list -t matching-engine | yq '.[0].templateId')
ACCOUNTANT_TEMPLATE_ID=$("$CMD" template list -t accountant | yq '.[0].templateId')

"$CMD" worker add --template-name adminapi --worker-name adminapi \
  --env REGISTRY_TEMPLATE_ID="$REGISTRY_TEMPLATE_ID" \
  --env IDS_TEMPLATE_ID="$IDS_TEMPLATE_ID" \
  --env MATCHING_ENGINE_TEMPLATE_ID="$MATCHING_ENGINE_TEMPLATE_ID" \
  --env ACCOUNTANT_TEMPLATE_ID="$ACCOUNTANT_TEMPLATE_ID"




