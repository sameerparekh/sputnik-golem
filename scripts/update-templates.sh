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

scripts/update-template.sh ids
scripts/update-template.sh registry
scripts/update-template.sh adminapi
scripts/update-template.sh traderapi
scripts/update-template.sh accountant
scripts/update-template.sh matching-engine