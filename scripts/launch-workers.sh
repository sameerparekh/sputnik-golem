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

"$CMD" worker add --component-name ids --worker-name "${ENVIRONMENT}"
"$CMD" worker add --component-name registry --worker-name "${ENVIRONMENT}"

IDS_COMPONENT_ID=$("$CMD" --format yaml component list -c ids | yq '.[0].componentId')
REGISTRY_COMPONENT_ID=$("$CMD" --format yaml component list -c registry | yq '.[0].componentId')
MATCHING_ENGINE_COMPONENT_ID=$("$CMD" --format yaml component list -c matching-engine | yq '.[0].componentId')
ACCOUNTANT_COMPONENT_ID=$("$CMD" --format yaml component list -c accountant | yq '.[0].componentId')
ETHEREUMMONITOR_COMPONENT_ID=$("$CMD" --format yaml component list -c ethereummonitor | yq '.[0].componentId')

"$CMD" worker add --component-name adminapi --worker-name "$ENVIRONMENT" \
  --env REGISTRY_COMPONENT_ID="$REGISTRY_COMPONENT_ID" \
  --env IDS_COMPONENT_ID="$IDS_COMPONENT_ID" \
  --env MATCHING_ENGINE_COMPONENT_ID="$MATCHING_ENGINE_COMPONENT_ID" \
  --env ACCOUNTANT_COMPONENT_ID="$ACCOUNTANT_COMPONENT_ID" \
  --env ETHEREUMMONITOR_COMPONENT_ID="$ETHEREUMMONITOR_COMPONENT_ID" \
  --env ENVIRONMENT="$ENVIRONMENT"

"$CMD" worker add --component-name traderapi --worker-name "$ENVIRONMENT" \
  --env REGISTRY_COMPONENT_ID="$REGISTRY_COMPONENT_ID" \
  --env IDS_COMPONENT_ID="$IDS_COMPONENT_ID" \
  --env MATCHING_ENGINE_COMPONENT_ID="$MATCHING_ENGINE_COMPONENT_ID" \
  --env ACCOUNTANT_COMPONENT_ID="$ACCOUNTANT_COMPONENT_ID" \
  --env ENVIRONMENT="$ENVIRONMENT"

"$CMD" worker add --component-name ethereummonitor --worker-name "$ENVIRONMENT" \
  --env REGISTRY_COMPONENT_ID="$REGISTRY_COMPONENT_ID" \
  --env IDS_COMPONENT_ID="$IDS_COMPONENT_ID" \
  --env MATCHING_ENGINE_COMPONENT_ID="$MATCHING_ENGINE_COMPONENT_ID" \
  --env ACCOUNTANT_COMPONENT_ID="$ACCOUNTANT_COMPONENT_ID" \
  --env ENVIRONMENT="$ENVIRONMENT" \
  --env PRIVATE_KEY="xprv9s21ZrQH143K41dsYLPrjnZsj1nh8ZBzAe2F57GEkMP1zpycr7YeSshbirbWfb4ujYA3mYp32kMG9rJ3rMSypCGzmX6h8PEwA5aTkMugbL6"
