mod auth;
mod ws;
mod kv;
mod util;

use fastly::{Request, Response, Error};

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    match req.get_path() {
        "/ws" => ws::handle(req),
        p if p.starts_with("/kv") => kv::handle(req),
        "/wld-multiplayer.js" => serve_static_js(),
        _ => serve_wordladder(req),
    }
}

fn serve_wordladder(req: Request) -> Result<Response, Error> {
    let origin = std::env::var("ORIGIN").unwrap_or_else(|_| "https://discord.wordladder.fun".to_string());
    let mut resp = Request::get(format!("{}{}", origin, req.get_path()))
        .send("origin")?;
    
    let content_type = resp.get_header("content-type")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("text/html");
    
    if content_type.starts_with("text/html") {
        let body = resp.take_body_str()
            .replace("</head>", r#"<script type="module" src="/wld-multiplayer.js"></script></head>"#);
        Ok(Response::from_body(body).with_header("content-type", "text/html; charset=utf-8"))
    } else {
        Ok(resp)
    }
}

fn serve_static_js() -> Result<Response, Error> {
    let js_content = include_str!("../static/wld-multiplayer.js");
    Ok(Response::from_body(js_content).with_header("content-type", "application/javascript"))
}
