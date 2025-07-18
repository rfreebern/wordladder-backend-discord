use fastly::{Request, Response, Error};

pub fn handle(_req: Request) -> Result<Response, Error> {
    // WebSocket support is not available in Fastly Compute@Edge
    // Return 501 Not Implemented for now
    Ok(Response::from_status(501).with_body("WebSocket not supported"))
}
