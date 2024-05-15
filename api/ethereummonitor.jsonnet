local component_id = std.extVar('component_id');
local environment = std.extVar('environment');

{
  id: "ethereummonitor",
  version: "0.0.1",
  draft: true,
  routes: [
    {
      method: "Get",
      path: "/blockheight",
      binding: {
        component: component_id,
        workerId: environment,
        functionName: "sputnik:ethereummonitor/api/block-height",
        functionParams: [],
        response: "${{headers: {ContentType: 'json'}, body: worker.response[0], status: 200}}"
      }
    },
    {
      method: "Post",
      path: "/deposit",
      binding: {
        component: component_id,
        workerId: environment,
        functionName: "sputnik:ethereummonitor/api/process-deposit",
        functionParams: [
          "${request.body.address}",
          "${request.body.tx}",
          "${request.body.amount}",
          "${request.body.asset_id}",
          "${request.body.block_height}",
        ],
        response: "${{headers: {ContentType: 'json'}, body: worker.response[0], status: 200}}"
      }
    },
    {
      method: "Post",
      path: "/completeblock/{height}",
      binding: {
        component: component_id,
        workerId: environment,
        functionName: "sputnik:ethereummonitor/api/complete-block",
        functionParams: [
          "${request.path.height}"
        ],
       response: "${{headers: {ContentType: 'json'}, body: worker.response[0], status: 200}}"
      }
    }
  ]
}