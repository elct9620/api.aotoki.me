use serde::Serialize;
use worker::*;

#[derive(Serialize)]
struct Version {
    version: String,
}

pub fn handler(_: Request, ctx: RouteContext<()>) -> Result<Response> {
    let version = ctx.var("WORKERS_RS_VERSION")?.to_string();
    Response::from_json(&Version { version })
}
