local component_id = std.extVar('component_id');
local environment = std.extVar('environment');

{
  id: "traderapi",
  version: "0.0.1",
  routes: [
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
    }
  ]
}