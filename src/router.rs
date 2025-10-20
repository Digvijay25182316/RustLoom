
use crate::network_handler::{Request, Response, RequestType};
use crate::controllers::{user_controller, health_controller};

pub fn handle_route(req: Request) -> Response {
    match (&req.method, req.path.as_str()) {
        (&RequestType::GET, "/") => {
            Response::new(200, "application/json", "{\"message\":\"Welcome!\"}")
        }

        (&RequestType::GET, "/health") => health_controller::check_health(&req),
        (&RequestType::GET, "/users") => user_controller::get_users(&req),
        (&RequestType::POST, "/users") => user_controller::create_user(&req),

        _ => Response::new(404, "application/json", "{\"error\":\"Not Found\"}"),
    }
}
