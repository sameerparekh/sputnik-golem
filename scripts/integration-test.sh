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

scripts/update-components.sh
scripts/update-api-definitions.sh -e "$ENVIRONMENT"
scripts/launch-workers.sh -e "$ENVIRONMENT"
scripts/deploy-apis.sh -e "$ENVIRONMENT"

USD_ID=$("$CMD" --format yaml worker invoke-and-await \
    --component-name=adminapi \
    --worker-name="$ENVIRONMENT" \
    --function=sputnik:adminapi/api/create-asset \
    --parameters='["USD", 2]' | yq .[0].ok.id)

BTC_ID=$("$CMD" --format yaml worker invoke-and-await \
        --component-name=adminapi \
        --worker-name="$ENVIRONMENT" \
        --function=sputnik:adminapi/api/create-asset \
        --parameters='["BTC", 8]' | yq .[0].ok.id)

BTCUSD_ID=$("$CMD" --format yaml worker invoke-and-await \
                --component-name=adminapi \
                --worker-name="$ENVIRONMENT" \
                --function=sputnik:adminapi/api/create-spot-pair \
                --parameters="[\"BTCUSD\", $BTC_ID, $USD_ID]" | yq .[0].ok.id)

PAIR_ID=$("$CMD" --format yaml worker invoke-and-await \
  --component-name=registry \
  --worker-name="$ENVIRONMENT" \
  --function=sputnik:registry/api/get-spot-pairs \
  --parameters='[]' | yq .[0].[0].id)

TRADER_A_ID=$("$CMD" --format yaml worker invoke-and-await \
  --component-name=adminapi \
  --worker-name="$ENVIRONMENT" \
  --function=sputnik:adminapi/api/create-trader \
  --parameters='["trader a"]' | yq .[0].ok.id)

TRADER_B_ID=$("$CMD" --format yaml worker invoke-and-await \
  --component-name=adminapi \
  --worker-name="$ENVIRONMENT" \
  --function=sputnik:adminapi/api/create-trader \
  --parameters='["trader b"]' | yq .[0].ok.id)

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

"$CMD" worker invoke-and-await \
  --component-name traderapi \
  --worker-name "${ENVIRONMENT}" \
  --function=sputnik:traderapi/api/place-order \
  --parameters="[$TRADER_A_ID, {\"spot-pair\": $BTCUSD_ID, \"side\": \"buy\", \"price\": 6000000, \"size\": 10000000}]"

"$CMD" worker invoke-and-await \
  --component-name traderapi \
  --worker-name "${ENVIRONMENT}" \
  --function=sputnik:traderapi/api/place-order \
  --parameters="[$TRADER_B_ID, {\"spot-pair\": $BTCUSD_ID, \"side\": \"sell\", \"price\": 7000000, \"size\": 10000000}]"


"$CMD" worker invoke-and-await \
  --component-name traderapi \
  --worker-name "${ENVIRONMENT}" \
  --function=sputnik:traderapi/api/get-orders \
  --parameters="[$TRADER_A_ID]"

"$CMD" worker invoke-and-await \
  --component-name traderapi \
  --worker-name "${ENVIRONMENT}" \
  --function=sputnik:traderapi/api/get-orders \
  --parameters="[$TRADER_B_ID]"

"$CMD" worker invoke-and-await \
  --component-name matching-engine \
  --worker-name "${ENVIRONMENT}-${BTCUSD_ID}" \
  --function=sputnik:matching-engine/api/get-order-book \
  --parameters='[]'

echo "$ENVIRONMENT"
