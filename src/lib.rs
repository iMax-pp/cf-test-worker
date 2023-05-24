use serde_json::json;
use worker::{*, kv::KvStore};

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let kv = KvStore::create("stats")?;
    let router = Router::with_data(kv);

    // GET /hello/{name}, returning a JSON with its access stats
    router
        .get_async("/hello/:name", |_, ctx| async move {
            if let Some(name) = ctx.param("name") {
                let stat = match ctx.data.get(name).text().await? {
                    Some(s) => {
                        let stat = s.parse::<i64>().unwrap() + 1;
                        ctx.data.put(name, stat.to_string())?.execute().await?;
                        s
                    }
                    None => {
                        ctx.data.put(name, "1")?.execute().await?;
                        String::from("1")
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