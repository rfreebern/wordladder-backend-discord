use fastly::{Request, Response, Error};
use serde_json::{Value};

use crate::auth;
use crate::util;

pub fn handle(mut req: Request) -> Result<Response, Error> {
    let path = req.get_path();
    let guild_user = path.trim_start_matches("/kv/").to_string();
    let parts: Vec<&str> = guild_user.split(':').collect();
    if parts.len() != 2 {
        return Ok(Response::from_status(400));
    }
    let (guild, user) = (parts[0], parts[1]);

    let jwt = req.get_header("Authorization")
        .ok_or(Error::msg("no auth"))?
        .to_str()?
        .trim_start_matches("Discord ")
        .to_string();
    
    let claims = auth::verify(&jwt)?;
    if claims.guild_id != guild || claims.user_id != user {
        return Ok(Response::from_status(403));
    }

    // TODO: when KV store is implemented, this is the key format. Remove underscore.
    let _key = format!("user:{guild}:{user}");

    match req.get_method_str() {
        "GET" => {
            // For now, return empty response as we need to implement KV store properly
            Ok(Response::from_body(r#"{"date":""}"#))
        }
        "PUT" => {
            if util::rate_limit(guild, user) {
                return Ok(Response::from_status(429));
            }
            let body_str = req.take_body_str();
            let mut body: Value = serde_json::from_str(&body_str)?;
            body["date"] = Value::String(util::today_gmt());
            // TODO: Implement proper KV storage
            Ok(Response::from_status(204))
        }
        _ => Ok(Response::from_status(405)),
    }
}
