// health_controller.rs
use crate::network_handler::{Request, Response};

pub fn check_health(_req: &Request) -> Response {
    Response::new(200, "application/json", "{\"status\":\"ok\"}")
}
