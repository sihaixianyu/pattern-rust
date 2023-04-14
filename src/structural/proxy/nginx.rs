use std::collections::HashMap;

use super::app::Application;

#[repr(u16)]
#[derive(Debug)]
pub enum StatusCode {
    OK = 200,
    Created = 201,
    Forbidden = 403,
    NotFound = 404,
}

pub trait Server {
    fn handle_request(&mut self, url: &str, method: &str) -> (StatusCode, String);
}

pub struct NginxServer {
    app: Application,
    max_req: usize,
    rates: HashMap<String, usize>,
}

impl NginxServer {
    pub fn new() -> NginxServer {
        NginxServer {
            app: Application::default(),
            max_req: 2,
            rates: HashMap::new(),
        }
    }
}

impl Server for NginxServer {
    fn handle_request(&mut self, url: &str, method: &str) -> (StatusCode, String) {
        let r = self.rates.entry(url.to_string()).or_insert(1);

        if *r > self.max_req {
            return (StatusCode::Forbidden, "max request exceeded!".into());
        }

        *r += 1;
        self.app.handle_request(url, method)
    }
}
