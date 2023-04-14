use super::nginx::{Server, StatusCode};

#[derive(Default)]
pub struct Application;

impl Server for Application {
    fn handle_request(&mut self, url: &str, method: &str) -> (StatusCode, String) {
        if url == "/app/status" && method == "GET" {
            return (StatusCode::OK, "OK".into());
        }

        if url == "/create/user" && method == "POST" {
            return (StatusCode::Created, "Created".into());
        }

        (StatusCode::NotFound, "Not Found".into())
    }
}
