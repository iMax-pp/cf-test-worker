use serde::{Deserialize, Serialize};
use serde_json::json;
use worker::*;

#[derive(Deserialize, Serialize)]
struct User {
    name: String,
    stat: Option<u64>,
}

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();

    router
        .put_async("/user/:email", create_user)
        .get_async("/user/:email", get_user)
        .get_async("/user", get_users)
        .run(req, env)
        .await
}

// PUT /user/{email}, creating a user from JSON info
async fn create_user(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let kv = ctx.kv("users")?;
    if let Some(email) = ctx.param("email") {
        let id = email.to_lowercase();
        let body = req.text().await?;
        let user = match serde_json::from_str::<User>(body.as_str()) {
            Ok(u) => User { stat: Some(1), ..u },
            Err(e) => return Response::error(e.to_string(), 400),
        };
        kv.put(&id, &user)?.execute().await?;
        return Response::ok("Created");
    }

    Response::error("Bad Request", 400)
}

// GET /user/{email}, returning a JSON of user info
async fn get_user(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let kv = ctx.kv("users")?;
    if let Some(email) = ctx.param("email") {
        let id = email.to_lowercase();
        let user = match kv.get(&id).json::<User>().await? {
            Some(u) => User {
                stat: Some(u.stat.unwrap() + 1),
                ..u
            },
            None => return Response::error("Not Found", 404),
        };
        kv.put(&id, &user)?.execute().await?;
        return Response::from_json(&json!(&user));
    }

    Response::error("Bad Request", 400)
}

// GET /user, returning all users keys
async fn get_users(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let kv = ctx.kv("users")?;
    let keys = kv.list().execute().await?.keys;
    let users: Vec<&String> = keys.iter().map(|k| &k.name).collect();
    Response::from_json(&json!(&users))
}
