#!/usr/bin/env bash
usage() {
  echo "Usage: $0 [-c]"
  echo "  -c           : Use golem cloud"
}

USE_CLOUD="${USE_CLOUD:-false}"

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

export USE_CLOUD

scripts/update-component.sh ids
scripts/update-component.sh registry
scripts/update-component.sh adminapi
scripts/update-component.sh traderapi
scripts/update-component.sh accountant
scripts/update-component.sh matching-engine