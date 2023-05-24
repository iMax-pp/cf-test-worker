use serde_json::json;
use worker::*;

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();

    // GET /hello/{name}, returning a JSON with its access stats
    router
        .get_async("/hello/:name", |_, ctx| async move {
            let kv = ctx.kv("stats")?;
            if let Some(name) = ctx.param("name") {
                let stat = match kv.get(name).json::<i64>().await? {
                    Some(s) => {
                        let stat = s + 1;
                        kv.put(name, stat)?.execute().await?;
                        stat
                    }
                    None => {
                        kv.put(name, 1)?.execute().await?;
                        1
                    }
                };
                Response::from_json(&json!({ "name": name, "stat": &stat }))
            } else {
                Response::error("Bad Request", 400)
            }
        })
        .run(req, env)
        .await
}