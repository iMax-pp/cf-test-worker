use serde_json::json;
use worker::*;

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();

    router
        .get_async("/hello/:name", get_user)
        .run(req, env)
        .await
}

// GET /hello/{name}, returning a JSON with user name and access stat
async fn get_user(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let kv = ctx.kv("stats")?;
    if let Some(name) = ctx.param("name") {
        let id = name.to_lowercase();
        let stat = match kv.get(&id).json::<u64>().await? {
            Some(s) => s + 1,
            None => 1,
        };
        kv.put(&id, stat)?.execute().await?;
        return Response::from_json(&json!({ "name": name, "stat": &stat }));
    }


    Response::error("Bad Request", 400)
}
