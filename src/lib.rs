use regex::Regex;
use serde_json::json;
use worker::*;

mod utils;

fn get_random_buf() -> Result<[u8; 32]> {
    let mut buf = [0u8; 32];
    let _ = getrandom::getrandom(&mut buf);
    Ok(buf)
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    utils::log_request(&req);
    utils::set_panic_hook();

    let router = Router::new();

    router
        .get("/", |_, _| {
            Response::redirect(Url::parse(
                "https://github.com/ghostdevv/mnemonic-worker#routes",
            )?)
        })
        .get("/new", |_, _| {
            let bytes = get_random_buf().unwrap();
            let result = mnemonic::to_string(&bytes);

            let words: Vec<String> = Regex::new("(-)+")
                .unwrap()
                .split(&result)
                .map(|x| x.to_string())
                .collect();

            return Response::from_json(&json!(words));
        })
        .run(req, env)
        .await
}
