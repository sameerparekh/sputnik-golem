#!/usr/bin/env bash

usage() {
  echo "Usage: $0 [-c] component-name"
  echo "  -c           : Use golem cloud"
  echo "  -e           : Environment (default: test)"
  echo "  component-name: API to update"
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

if [ "$#" -ne 0 ]; then
  usage
  exit 1
fi

export ENVIRONMENT
export USE_CLOUD

scripts/deploy-api.sh traderapi
scripts/deploy-api.sh adminapi
scripts/deploy-api.sh ethereummonitor
