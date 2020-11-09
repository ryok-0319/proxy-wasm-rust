use log::info;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use std::time::Duration;

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_root_context(|_| -> Box<dyn RootContext> {
        Box::new(Scraper)
    });
}

struct Scraper;

impl Context for Scraper {}

impl RootContext for Scraper {
    fn on_vm_start(&mut self, _: usize) -> bool {
        info!("Hello!");
        self.set_tick_period(Duration::from_secs(5));
        true
    }

    fn on_tick(&mut self) {
        info!("World!!!");
    }
}