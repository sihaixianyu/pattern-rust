pub mod app;
pub mod nginx;

#[cfg(test)]
mod tests {
    use std::{
        sync::{Arc, RwLock},
        thread,
    };

    use super::nginx::{NginxServer, Server};

    #[test]
    fn test_proxy() {
        let app_status = "/app/status";
        let create_user = "/create/user";

        let nginx = Arc::new(RwLock::new(NginxServer::new()));
        let mut handles = vec![];

        for _ in 0..3 {
            let nginx = nginx.clone();

            let handle = thread::spawn(move || {
                let (code, info) = nginx.write().unwrap().handle_request(app_status, "GET");
                println!(
                    "Url: {}\nHttpCode: {:?}\nInfo: {}\n",
                    app_status, code, info
                );
            });

            handles.push(handle);
        }

        for _ in 0..2 {
            let nginx = nginx.clone();

            let handle = thread::spawn(move || {
                let (code, info) = nginx.write().unwrap().handle_request(create_user, "POST");
                println!(
                    "Url: {}\nHttpCode: {:?}\nInfo: {}\n",
                    create_user, code, info
                );
            });

            handles.push(handle);
        }

        for _ in 0..2 {
            let nginx = nginx.clone();

            let handle = thread::spawn(move || {
                let (code, info) = nginx.write().unwrap().handle_request(create_user, "GET");
                println!(
                    "Url: {}\nHttpCode: {:?}\nInfo: {}\n",
                    create_user, code, info
                );
            });

            handles.push(handle);
        }

        handles.into_iter().for_each(|h| h.join().unwrap());
    }
}
