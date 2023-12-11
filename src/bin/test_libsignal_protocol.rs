use xous_api_log_server as log_server;
use std::{thread, time};
use libsignal_protocol::{SignalProtocolError};

#[path = "../../tests/support/mod.rs"]
mod support;

#[path = "../../tests/session.rs"]
mod session_test;

use support::{initialize_sessions_v3,initialize_sessions_v4};
use session_test::{run_session_interaction};

type TestResult = Result<(), SignalProtocolError>;

fn main() -> ! {
    log_server::init_wait().unwrap();
    log::set_max_level(log::LevelFilter::Info);
    log::info!("my PID is {}", xous::process::id());

    let timeout = time::Duration::from_millis(1000);
    log::info!("trying to initalize sessions v3");
    let (alice_session,bob_session) = initialize_sessions_v3().unwrap();
    log::info!("initalize sessions v3 success");
    log::info!("running interaction");
    run_session_interaction(alice_session, bob_session).unwrap();

    log::info!("trying to initalize sessions v4");
    let (alice_session, bob_session) = initialize_sessions_v4().unwrap();
    log::info!("initalize sessions v4 success");
    log::info!("running interaction");
    run_session_interaction(alice_session, bob_session).unwrap();
    log::info!("both successful");
    loop {
        log::info!("sleeping");
        std::thread::sleep(timeout);
    }
}
