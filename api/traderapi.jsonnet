local component_id = std.extVar('component_id');
local registry_id = std.extVar('registry_id');
local environment = std.extVar('environment');
local matching_engine_id = std.extVar('matching_engine_id');
local ethereummonitor_id = std.extVar('ethereummonitor_id');

{
  id: "traderapi",
  version: "0.0.1",
  draft: true,
  routes: [
      {
        method: "Get",
        path: "/evm-address/{trader}",
        binding: {
          component: ethereummonitor_id,
          workerId: environment,
          functionName: "sputnik:ethereummonitor/api/new-address-for-trader",
          functionParams: [
            "${request.path.trader}"
          ],
          response: "${{headers: {ContentType: 'json'}, body: worker.response[0], status: 200}}"
        }
      },
    {
      method: "Get",
      path: "/orders/{trader}",
      binding: {
        component: component_id,
        workerId: environment,
        functionName: "sputnik:traderapi/api/get-orders",
        functionParams: [
          "${request.path.trader}"
        ],
        response: "${{headers: {ContentType: 'json'}, body: worker.response[0], status: 200}}"
      }
    },
    {
      method: "Get",
      path: "/asset",
      binding: {
        component: registry_id,
        workerId: environment,
        functionName: "sputnik:registry/api/get-assets",
        functionParams: [],
        response: "${{headers: {ContentType: 'json'}, body: worker.response[0], status: 200}}"
      }
    },
    {
      method: "Get",
      path: "/spot-pair",
      binding: {
        component: registry_id,
        workerId: environment,
        functionName: "sputnik:registry/api/get-spot-pairs",
        functionParams: [],
        response: "${{headers: {ContentType: 'json'}, body: worker.response[0], status: 200}}"
      }
    },
    {
      method: "Get",
      path: "/balances/{trader}",
      binding: {
        component: component_id,
        workerId: environment,
        functionName: "sputnik:traderapi/api/get-balances",
        functionParams: [
          "${request.path.trader}"
        ],
        response: "${{headers: {ContentType: 'json'}, body: worker.response[0], status: 200}}"
      }
    },
    {
      method: "Post",
      path: "/orders/{trader}",
      binding: {
        component: component_id,
        workerId: environment,
        functionName: "sputnik:traderapi/api/place-order",
        functionParams: [
          "${request.path.trader}",
          "${request.body}"
        ],
        response: "${{headers: {ContentType: 'json'}, body: worker.response[0], status: 201}}"
      }
    },
    {
      method: "Get",
      path: "/orderbook/{spotpair}",
      binding: {
        component: matching_engine_id,
        workerId: environment + "-${request.path.spotpair}",
        functionName: "sputnik:matching-engine/api/get-order-book",
        functionParams: [],
        response: "${{headers: {ContentType: 'json'}, body: worker.response[0], status: 200}}"
      }
    }
  ]
}