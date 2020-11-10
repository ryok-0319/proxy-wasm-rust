use log::info;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_http_context(|_, _| -> Box<dyn HttpContext> {
        Box::new(ActivityLogger{ logger_type: "ActivityLogger".to_string(), ..Default::default() })
    });
}

#[derive(Default)]
struct ActivityLogger {
    finc_user_id: String,
    path: String,
    method: String,
    status: String,
    logger_type: String,
}

impl Context for ActivityLogger {}

impl HttpContext for ActivityLogger {
    fn on_http_request_headers(&mut self, _: usize) -> Action {
        for (name, value) in &self.get_http_request_headers() {
            match &name[..] {
                "x-request-fid" => self.finc_user_id = value.to_string(),
                ":path" => self.path = value.to_string(),
                ":method" => self.method = value.to_string(),
                _ => (),
            }
        }

        Action::Continue
    }

    fn on_http_response_headers(&mut self, _: usize) -> Action {
        for (name, value) in &self.get_http_response_headers() {
            if name == ":status" {
                self.status = value.to_string();
                break
            }
        }

        info!("finc_user_id: {}, path: {}, method: {}, status: {}, logger_type: {}",
            self.finc_user_id,
            self.path,
            self.method,
            self.status,
            self.logger_type);

        Action::Continue
    }
}