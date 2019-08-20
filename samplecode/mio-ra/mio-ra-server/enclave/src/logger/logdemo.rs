pub fn log_demo() {
    println!("------------------------------------------");
    env_logger_gel::init();
    info!("{}", "log test");
    trace!("{}", "log trace test");
    println!("------------------------------------------");

}
