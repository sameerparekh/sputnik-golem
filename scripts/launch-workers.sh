#!/usr/bin/env bash
usage() {
  echo "Usage: $0 [-c]"
  echo "  -c           : Use golem cloud"
  echo "  -e env       : Set environment (default: test)"
}

USE_CLOUD="${USE_CLOUD:-false}"
ENVIRONMENT="test"

while getopts "ce:" opt; do
  case $opt in
    c)
      USE_CLOUD=true
      ;;
    e)
      ENVIRONMENT=${OPTARG}
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

"$CMD" worker add --template-name ids --worker-name "${ENVIRONMENT}"
"$CMD" worker add --template-name registry --worker-name "${ENVIRONMENT}"

IDS_TEMPLATE_ID=$("$CMD" template list -t ids | yq '.[0].templateId')
REGISTRY_TEMPLATE_ID=$("$CMD" template list -t registry | yq '.[0].templateId')
MATCHING_ENGINE_TEMPLATE_ID=$("$CMD" template list -t matching-engine | yq '.[0].templateId')
ACCOUNTANT_TEMPLATE_ID=$("$CMD" template list -t accountant | yq '.[0].templateId')

"$CMD" worker add --template-name adminapi --worker-name "$ENVIRONMENT" \
  --env REGISTRY_TEMPLATE_ID="$REGISTRY_TEMPLATE_ID" \
  --env IDS_TEMPLATE_ID="$IDS_TEMPLATE_ID" \
  --env MATCHING_ENGINE_TEMPLATE_ID="$MATCHING_ENGINE_TEMPLATE_ID" \
  --env ACCOUNTANT_TEMPLATE_ID="$ACCOUNTANT_TEMPLATE_ID" \
  --env ENVIRONMENT="$ENVIRONMENT"

"$CMD" worker add --template-name traderapi --worker-name "$ENVIRONMENT" \
  --env REGISTRY_TEMPLATE_ID="$REGISTRY_TEMPLATE_ID" \
  --env IDS_TEMPLATE_ID="$IDS_TEMPLATE_ID" \
  --env MATCHING_ENGINE_TEMPLATE_ID="$MATCHING_ENGINE_TEMPLATE_ID" \
  --env ACCOUNTANT_TEMPLATE_ID="$ACCOUNTANT_TEMPLATE_ID" \
  --env ENVIRONMENT="$ENVIRONMENT"
