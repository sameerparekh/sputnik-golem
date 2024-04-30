#!/usr/bin/env bash
usage() {
  echo "Usage: $0 [-c]"
  echo "  -c           : Use golem cloud"
  echo "  -b           : Do not rebuild"
}

ENVIRONMENT="it-$(openssl rand -hex 5)"

echo "$ENVIRONMENT"

USE_CLOUD="${USE_CLOUD:-false}"
NO_BUILD=false

while getopts ":cb" opt; do
  case $opt in
    c)
      USE_CLOUD=true
      ;;
    b)
      NO_BUILD=true
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
  exit 0
fi

if [ "$USE_CLOUD" = 'true' ]; then
  CMD=golem-cloud-cli
else
  CMD=golem-cli
fi

export USE_CLOUD

if [ "$NO_BUILD" = 'false' ]; then
  scripts/build.sh
fi

source .env

scripts/update-components.sh
scripts/update-api-definitions.sh -e "$ENVIRONMENT"
scripts/launch-workers.sh -e "$ENVIRONMENT"
scripts/deploy-apis.sh -e "$ENVIRONMENT"

ADMIN_API=http://"${ENVIRONMENT}".adminapi.sputnik.dev:"${WORKER_SERVICE_CUSTOM_REQUEST_PORT}"
TRADER_API=http://"${ENVIRONMENT}".traderapi.sputnik.dev:"${WORKER_SERVICE_CUSTOM_REQUEST_PORT}"

BTC_ID=$(curl --silent -X POST "$ADMIN_API"/asset/BTC \
  --data '{ "decimals": 8 }' | jq '.ok.id')

USD_ID=$(curl --silent -X POST "$ADMIN_API"/asset/USD \
  --data '{ "decimals": 2 }' | jq '.ok.id')

BTCUSD_ID=$(curl --silent -X POST "$ADMIN_API"/spot-pair/BTCUSD \
         --data "{ \"numerator\": $BTC_ID, \"denominator\": $USD_ID }" | jq '.ok.id')

curl --silent "$TRADER_API/asset"
curl --silent "$TRADER_API/spot-pair"

TRADER_A_ID=$(curl --silent -X POST "$ADMIN_API"/trader/tradera \
          | jq '.ok.id')

TRADER_B_ID=$(curl --silent -X POST "$ADMIN_API"/trader/traderb \
          | jq '.ok.id')

"$CMD" worker invoke-and-await \
  --component-name accountant \
  --worker-name "${ENVIRONMENT}-${TRADER_A_ID}" \
  --function=sputnik:accountant/api/deposit \
  --parameters="[$BTC_ID, 100000000]"

"$CMD" worker invoke-and-await \
  --component-name accountant \
  --worker-name "${ENVIRONMENT}-${TRADER_A_ID}" \
  --function=sputnik:accountant/api/deposit \
  --parameters="[$USD_ID, 6000000]"

"$CMD" worker invoke-and-await \
  --component-name accountant \
  --worker-name "${ENVIRONMENT}-${TRADER_B_ID}" \
  --function=sputnik:accountant/api/deposit \
  --parameters="[$BTC_ID, 100000000]"

"$CMD" worker invoke-and-await \
  --component-name accountant \
  --worker-name "${ENVIRONMENT}-${TRADER_B_ID}" \
  --function=sputnik:accountant/api/deposit \
  --parameters="[$USD_ID, 6000000]"

curl --silent -X POST "$TRADER_API"/orders/"$TRADER_A_ID" \
  --data "{\"spot-pair\": $BTCUSD_ID, \"side\": \"buy\", \"price\": 6000000, \"size\": 10000000}"

curl --silent -X GET "$TRADER_API"/balances/"$TRADER_A_ID" | jq .
curl --silent -X GET "$TRADER_API"/balances/"$TRADER_B_ID" | jq .

curl --silent -X POST "$TRADER_API"/orders/"$TRADER_B_ID" \
  --data "{\"spot-pair\": $BTCUSD_ID, \"side\": \"sell\", \"price\": 7000000, \"size\": 10000000}"

curl --silent -X GET "$TRADER_API"/orders/"$TRADER_A_ID" | jq .
curl --silent -X GET "$TRADER_API"/orders/"$TRADER_B_ID" | jq .

curl --silent -X GET "$TRADER_API"/orderbook/"$BTCUSD_ID" | jq .

echo "$ENVIRONMENT"
