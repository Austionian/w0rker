use serde::{Deserialize, Serialize};
use worker::*;

#[derive(Deserialize, Serialize)]
struct Post {
    id: String,
    read_count: u32,
}

#[event(fetch, respond_with_errors)]
pub async fn main(request: Request, env: Env, _ctx: Context) -> Result<Response> {
    Router::new()
        .get_async("/:id", |_, ctx| async move {
            let id = ctx.param("id").unwrap();
            let d1 = ctx.env.d1("DB")?;
            let statement = d1.prepare("SELECT * FROM posts WHERE id = ?1");
            let query = statement.bind(&[id.into()])?;
            let result = query.first::<Post>(None).await?;
            match result {
                Some(post) => Response::from_json(&post),
                None => Response::error("Not found", 404),
            }
        })
        .post_async("/:id", |_, ctx| async move {
            let id = ctx.param("id").unwrap();
            let d1 = ctx.env.d1("DB")?;
            let statement =
                d1.prepare("UPDATE posts SET read_count = read_count + 1 WHERE id = ?1");
            let query = statement.bind(&[id.into()])?;
            let result = query.run().await;
            match result {
                Ok(_) => Response::ok("Row updated."),
                Err(_) => Response::error("Row not updated", 401),
            }
        })
        .post_async("/new/:id", |_, ctx| async move {
            let id = ctx.param("id").unwrap();
            let d1 = ctx.env.d1("DB")?;
            let statement = d1.prepare("INSERT INTO posts (id, read_count) VALUES (?1, 0)");
            let query = statement.bind(&[id.into()])?;
            let result = query.run().await;
            match result {
                Ok(_) => Response::ok("Row created."),
                Err(_) => Response::error("Row not created.", 500),
            }
        })
        .run(request, env)
        .await
}
