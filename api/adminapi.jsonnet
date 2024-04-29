local component_id = std.extVar('component_id');
local environment = std.extVar('environment');

{
  id: "adminapi",
  version: "0.0.1",
  routes: [
    {
      method: "Post",
      path: "/asset/{name}",
      binding: {
        component: component_id,
        workerId: environment,
        functionName: "sputnik:adminapi/api/create-asset",
        functionParams: [
          "${request.path.name}",
          "${request.body.decimals}"
        ],
        response: "${{headers: {ContentType: 'json'}, body: worker.response[0], status: 201}}"
      }
    },
    {
      method: "Post",
      path: "/spot-pair/{name}",
      binding: {
        component: component_id,
        workerId: environment,
        functionName: "sputnik:adminapi/api/create-spot-pair",
        functionParams: [
          "${request.path.name}",
          "${request.body.numerator}",
          "${request.body.denominator}"
        ],
        response: "${{headers: {ContentType: 'json'}, body: worker.response[0], status: 201}}"
      }
    },
    {
      method: "Post",
      path: "/trader/{name}",
      binding: {
        component: component_id,
        workerId: environment,
        functionName: "sputnik:adminapi/api/create-trader",
        functionParams: [
          "${request.path.name}"
        ],
       response: "${{headers: {ContentType: 'json'}, body: worker.response[0], status: 201}}"
      }
    }
  ]
}