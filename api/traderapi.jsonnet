local component_id = std.extVar('component_id');
local environment = std.extVar('environment');

{
  id: component_id,
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
        response: "${{headers: {ContentType: 'json'}, body: worker.response, status: 200}}"
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
        response: "${{headers: {ContentType: 'json'}, body: worker.response, status: 201}}"
      }
    }
  ]
}