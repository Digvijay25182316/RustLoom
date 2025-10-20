use crate::network_handler::{Request, Response};
use crate::services::user_service;

pub fn get_users(_req: &Request) -> Response {
    let data = user_service::fetch_all_users();
    Response::new(200, "application/json", &data)
}

pub fn create_user(req: &Request) -> Response {
    let data = user_service::create_user(&req.body);
    Response::new(201, "application/json", &data)
}
