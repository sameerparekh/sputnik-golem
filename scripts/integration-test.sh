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

echo "Creating assets/pairs"

USDC_ADDRESS=0x1c7D4B196Cb0C7B01d743Fbc6116a902379C7238
EURC_ADDRESS=0x08210F9170F89Ab7658F0B5E3fF39b0E03C594D4
ETH_ADDRESS=0x0000000000000000000000000000000000000000

ETH_ID=$(curl --silent -X POST "$ADMIN_API"/asset/ETH \
  --data "{ \"decimals\": 18, \"token_address\": \"$ETH_ADDRESS\" }" | jq '.ok.id')

echo "ETH: $ETH_ID"

USDC_ID=$(curl --silent -X POST "$ADMIN_API"/asset/USDC \
  --data "{ \"decimals\": 6, \"token_address\": \"$USDC_ADDRESS\" }" | jq '.ok.id')

echo "USDC: $USDC_ID"

EURC_ID=$(curl --silent -X POST "$ADMIN_API"/asset/EURC \
  --data "{ \"decimals\": 6, \"token_address\": \"$EURC_ADDRESS\" }" | jq '.ok.id')

echo "EURC: $EURC_ID"

ETHUSDC_ID=$(curl --silent -X POST "$ADMIN_API"/spot-pair/ETHUSDC \
         --data "{ \"numerator\": $ETH_ID, \"denominator\": $USDC_ID }" | jq '.ok.id')

echo "ETH/USDC: $ETHUSDC_ID"

ETHEURC_ID=$(curl --silent -X POST "$ADMIN_API"/spot-pair/ETHEURC \
         --data "{ \"numerator\": $ETH_ID, \"denominator\": $EURC_ID }" | jq '.ok.id')

echo "ETH/EURC: $ETHEURC_ID"

USDCEURC_ID=$(curl --silent -X POST "$ADMIN_API"/spot-pair/USDCEURC \
                     --data "{ \"numerator\": $USDC_ID, \"denominator\": $EURC_ID }" | jq '.ok.id')

echo "USDC/EURC: $USDCEURC_ID"

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

BLOCK_HEIGHT=$(curl --silent -X GET "$MONITOR_API"/blockheight | jq .height )

echo -n "Trader A ETH: "
curl --silent -X POST "$MONITOR_API"/deposit \
  --data "{\"address\": $TRADER_A_ADDRESS, \"tx\": \"tx1\", \"amount\": 100000000, \"token_address\": \"$ETH_ADDRESS\", \"block_height\": $BLOCK_HEIGHT}" \
  | jq .ok.balance

echo -n "Trader A USDC: "
curl --silent -X POST "$MONITOR_API"/deposit \
  --data "{\"address\": $TRADER_A_ADDRESS, \"tx\": \"tx2\", \"amount\": 6000000, \"token_address\": \"$USDC_ADDRESS\", \"block_height\": $BLOCK_HEIGHT}" \
  | jq .ok.balance

echo -n "Trader B ETH: "
curl --silent -X POST "$MONITOR_API"/deposit \
  --data "{\"address\": $TRADER_B_ADDRESS, \"tx\": \"tx3\", \"amount\": 100000000, \"token_address\": \"$ETH_ADDRESS\", \"block_height\": $BLOCK_HEIGHT}" \
  | jq .ok.balance

echo -n "Trader B USDC: "
curl --silent -X POST "$MONITOR_API"/deposit \
  --data "{\"address\": $TRADER_B_ADDRESS, \"tx\": \"tx4\", \"amount\": 6000000, \"token_address\": \"$USDC_ADDRESS\", \"block_height\": $BLOCK_HEIGHT}" \
  | jq .ok.balance

echo -n "Completing block..."
curl --silent -X POST "$MONITOR_API"/completeblock/"$BLOCK_HEIGHT" >/dev/null
echo

echo -n "New block height: "
curl --silent -X GET "$MONITOR_API"/blockheight

echo

echo -n "Placing trader A order: "

curl --silent -X POST "$TRADER_API"/orders/"$TRADER_A_ID" \
  --data "{\"spot-pair\": $ETHUSDC_ID, \"side\": \"buy\", \"price\": 6000000, \"size\": 10000000}" \
  | jq .ok

echo -n "Trader A balance: "
curl --silent -X GET "$TRADER_API"/balances/"$TRADER_A_ID" | jq .

echo -n "Placing trader B order: "
curl --silent -X POST "$TRADER_API"/orders/"$TRADER_B_ID" \
  --data "{\"spot-pair\": $ETHUSDC_ID, \"side\": \"sell\", \"price\": 7000000, \"size\": 10000000}" \
  | jq .ok

echo -n "Trader B balance: "
curl --silent -X GET "$TRADER_API"/orders/"$TRADER_B_ID" | jq .

echo "Order Book"
curl --silent -X GET "$TRADER_API"/orderbook/"$ETHUSDC_ID" | jq .

echo -n "Placing trader B order: "
curl --silent -X POST "$TRADER_API"/orders/"$TRADER_B_ID" \
  --data "{\"spot-pair\": $ETHUSDC_ID, \"side\": \"sell\", \"price\": 6700000, \"size\": 10000000}" \
  | jq .ok

echo -n "Placing trader B order: "
curl --silent -X POST "$TRADER_API"/orders/"$TRADER_B_ID" \
  --data "{\"spot-pair\": $ETHUSDC_ID, \"side\": \"sell\", \"price\": 7500000, \"size\": 10000000}" \
  | jq .ok

echo -n "Placing trader A order that will match them: "
curl --silent -X POST "$TRADER_API"/orders/"$TRADER_A_ID" \
  --data "{\"spot-pair\": $ETHUSDC_ID, \"side\": \"buy\", \"price\": 7000000, \"size\": 25000000}" \
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
curl --silent -X GET "$TRADER_API"/orderbook/"$ETHUSDC_ID" | jq .

echo "ADMIN_API: $ADMIN_API"
echo "TRADER_API: $TRADER_API"
echo "MONITOR_API: $MONITOR_API"

echo "TRADER_A_ADDRESS": "$TRADER_A_ADDRESS"
echo "TRADER_B_ADDRESS": "$TRADER_B_ADDRESS"

export MONITOR_API
RPC_URL="wss://eth-sepolia.g.alchemy.com/v2/C2AMPkL7J84rizAWoLcCt5rTflAU0tGY"
export RPC_URL
cargo run --package chainmonitor
