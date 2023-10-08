use xous_api_log_server as log_server;
use std::{thread, time};

fn main() -> ! {
    log_server::init_wait().unwrap();
    log::set_max_level(log::LevelFilter::Info);
    log::info!("my PID is {}", xous::process::id());

    let timeout = time::Duration::from_millis(1000);
    let mut count = 0;
    loop {
        log::info!("test loop {}", count);
        count += 1;
        thread::sleep(timeout);
    }
}
