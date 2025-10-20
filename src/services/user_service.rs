use serde_json::json;

pub fn fetch_all_users() -> String {
    json!([
        { "id": 1, "name": "Alice" },
        { "id": 2, "name": "Bob" }
    ])
    .to_string()
}

pub fn create_user(body: &str) -> String {
    // You can parse the body JSON if you like
    json!({
        "message": "User created successfully",
        "body": body
    })
    .to_string()
}
