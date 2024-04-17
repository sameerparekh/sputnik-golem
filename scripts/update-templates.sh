#!/usr/bin/env bash
usage() {
  echo "Usage: $0 [-c]"
  echo "  -c           : Use golem cloud"
}

scripts/update-template.sh "$@" ids
scripts/update-template.sh "$@" registry
scripts/update-template.sh "$@" adminapi
scripts/update-template.sh "$@" accountant
scripts/update-template.sh "$@" matching-engine