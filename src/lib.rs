use serde_json::json;
use worker::*;

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();
    router
        .get("/hello/:name", |_, ctx| {
            if let Some(name) = ctx.param("name") {
                return Response::from_json(&json!({ "name": name }))
            }
            Response::error("Bad Request", 400)
        })
        .run(req, env)
        .await
}