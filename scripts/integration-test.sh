#!/usr/bin/env bash
usage() {
  echo "Usage: $0 [-c]"
  echo "  -c           : Use golem cloud"
  echo "  -b           : Do not rebuild"
}

ENVIRONMENT="it-$(openssl rand -hex 5)"

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
  echo -n Building...
  scripts/build.sh >/dev/null 2>/dev/null
  echo
fi

source .env

echo -n Installing...
scripts/update-components.sh >/dev/null
scripts/update-api-definitions.sh -e "$ENVIRONMENT" >/dev/null
scripts/launch-workers.sh -e "$ENVIRONMENT" >/dev/null
scripts/deploy-apis.sh -e "$ENVIRONMENT" >/dev/null
echo

ADMIN_API=http://"${ENVIRONMENT}".adminapi.sputnik.golem:"${WORKER_SERVICE_CUSTOM_REQUEST_PORT}"
TRADER_API=http://"${ENVIRONMENT}".traderapi.sputnik.golem:"${WORKER_SERVICE_CUSTOM_REQUEST_PORT}"
MONITOR_API=http://"${ENVIRONMENT}".ethereummonitor.sputnik.golem:"${WORKER_SERVICE_CUSTOM_REQUEST_PORT}"
#
#set -ex

echo "Creating assets/pairs"

BTC_ID=$(curl --silent -X POST "$ADMIN_API"/asset/BTC \
  --data '{ "decimals": 8 }' | jq '.ok.id')

echo "BTC: $BTC_ID"

USD_ID=$(curl --silent -X POST "$ADMIN_API"/asset/USD \
  --data '{ "decimals": 2 }' | jq '.ok.id')

echo "USD: $USD_ID"

BTCUSD_ID=$(curl --silent -X POST "$ADMIN_API"/spot-pair/BTCUSD \
         --data "{ \"numerator\": $BTC_ID, \"denominator\": $USD_ID }" | jq '.ok.id')

echo "BTCUSD: $BTCUSD_ID"

echo "Listing assets & pairs"

curl --silent "$TRADER_API/asset" | jq .
curl --silent "$TRADER_API/spot-pair" | jq .

echo "Creating traders"

TRADER_A_ID=$(curl --silent -X POST "$ADMIN_API"/trader/tradera \
          | jq '.ok.id')

TRADER_B_ID=$(curl --silent -X POST "$ADMIN_API"/trader/traderb \
          | jq '.ok.id')

echo "Trader A: $TRADER_A_ID"
echo "Trader B: $TRADER_B_ID"

echo "Funding accounts..."

#set -ex

TRADER_A_ADDRESS=$(curl --silent -X GET "$TRADER_API"/evm-address/"${TRADER_A_ID}")
TRADER_B_ADDRESS=$(curl --silent -X GET "$TRADER_API"/evm-address/"${TRADER_B_ID}")

echo -n "Trader A BTC: "
curl --silent -X POST "$MONITOR_API"/deposit \
  --data "{\"address\": $TRADER_A_ADDRESS, \"tx\": \"tx1\", \"amount\": 100000000, \"asset_id\": $BTC_ID, \"block_height\": 10}" \
  | jq .ok.balance

echo -n "Trader A USD: "
curl --silent -X POST "$MONITOR_API"/deposit \
  --data "{\"address\": $TRADER_A_ADDRESS, \"tx\": \"tx2\", \"amount\": 6000000, \"asset_id\": $USD_ID, \"block_height\": 10}" \
  | jq .ok.balance

echo -n "Trader B BTC: "
curl --silent -X POST "$MONITOR_API"/deposit \
  --data "{\"address\": $TRADER_B_ADDRESS, \"tx\": \"tx3\", \"amount\": 100000000, \"asset_id\": $BTC_ID, \"block_height\": 10}" \
  | jq .ok.balance

echo -n "Trader B USD: "
curl --silent -X POST "$MONITOR_API"/deposit \
  --data "{\"address\": $TRADER_B_ADDRESS, \"tx\": \"tx4\", \"amount\": 6000000, \"asset_id\": $USD_ID, \"block_height\": 10}" \
  | jq .ok.balance

echo

echo -n "Placing trader A order: "

curl --silent -X POST "$TRADER_API"/orders/"$TRADER_A_ID" \
  --data "{\"spot-pair\": $BTCUSD_ID, \"side\": \"buy\", \"price\": 6000000, \"size\": 10000000}" \
  | jq .ok

echo -n "Trader A balance: "
curl --silent -X GET "$TRADER_API"/balances/"$TRADER_A_ID" | jq .

echo -n "Placing trader B order: "
curl --silent -X POST "$TRADER_API"/orders/"$TRADER_B_ID" \
  --data "{\"spot-pair\": $BTCUSD_ID, \"side\": \"sell\", \"price\": 7000000, \"size\": 10000000}" \
  | jq .ok

echo -n "Trader B balance: "
curl --silent -X GET "$TRADER_API"/orders/"$TRADER_B_ID" | jq .

echo "Order Book"
curl --silent -X GET "$TRADER_API"/orderbook/"$BTCUSD_ID" | jq .

echo -n "Placing trader B order: "
curl --silent -X POST "$TRADER_API"/orders/"$TRADER_B_ID" \
  --data "{\"spot-pair\": $BTCUSD_ID, \"side\": \"sell\", \"price\": 6700000, \"size\": 10000000}" \
  | jq .ok

echo -n "Placing trader B order: "
curl --silent -X POST "$TRADER_API"/orders/"$TRADER_B_ID" \
  --data "{\"spot-pair\": $BTCUSD_ID, \"side\": \"sell\", \"price\": 7500000, \"size\": 10000000}" \
  | jq .ok

echo -n "Placing trader A order that will match them: "
curl --silent -X POST "$TRADER_API"/orders/"$TRADER_A_ID" \
  --data "{\"spot-pair\": $BTCUSD_ID, \"side\": \"buy\", \"price\": 7000000, \"size\": 25000000}" \
  | jq .ok

echo "Trader A Orders"
curl --silent -X GET "$TRADER_API"/orders/"$TRADER_A_ID" | jq .

echo "Trader A Balances"
curl --silent -X GET "$TRADER_API"/balances/"$TRADER_A_ID" | jq .

echo "Trader B Orders"
curl --silent -X GET "$TRADER_API"/orders/"$TRADER_B_ID" | jq .

echo "Trader B Balances"
curl --silent -X GET "$TRADER_API"/balances/"$TRADER_B_ID" | jq .

echo "Order Book"
curl --silent -X GET "$TRADER_API"/orderbook/"$BTCUSD_ID" | jq .    

set +ex

echo "ADMIN_API: $ADMIN_API"
echo "TRADER_API: $TRADER_API"

