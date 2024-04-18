#!/usr/bin/env bash
ENVIRONMENT=$(openssl rand -base64 12)

scripts/build.sh
scripts/update-templates.sh
scripts/launch-workers.sh -e "$ENVIRONMENT"

USD_ID=$(golem-cli worker invoke-and-await \
    --template-name=adminapi \
    --worker-name="$ENVIRONMENT" \
    --function=sputnik:adminapi/api/create-asset \
    --parameters='["USD", 2]' | yq .[0].ok.id)

BTC_ID=$(golem-cli worker invoke-and-await \
        --template-name=adminapi \
        --worker-name="$ENVIRONMENT" \
        --function=sputnik:adminapi/api/create-asset \
        --parameters='["BTC", 8]' | yq .[0].ok.id)

BTCUSD_ID=$(golem-cli worker invoke-and-await \
                --template-name=adminapi \
                --worker-name="$ENVIRONMENT" \
                --function=sputnik:adminapi/api/create-spot-pair \
                --parameters="[\"BTCUSD\", $BTC_ID, $USD_ID]" | yq .[0].ok.id)

golem-cli worker invoke-and-await \
  --template-name=registry \
  --worker-name="$ENVIRONMENT" \
  --function=sputnik:registry/api/get-spot-pairs \
  --parameters='[]'
  


