use log::{debug, error, info, trace};
use crate::log::{init_log, LogLevel, update_log_level};

#[test]
fn test_log() {
    init_log(LogLevel::INFO);
    info!("--- 1. finish log ---");
    debug!("--- 1. finish log ---");
    error!("--- 1. finish log ---");
    trace!("--- 1. finish log ---");
}

#[test]
fn test_trace_log() {
    init_log(LogLevel::TRACE);
    info!("--- 2. finish log ---");
    debug!("--- 2. finish log ---");
    error!("--- 2. finish log ---");
    trace!("--- 2. finish log ---");
}