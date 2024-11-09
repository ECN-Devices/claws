pub fn init_logger() {
    std::env::set_var("RUST_LOG", "claws");
    pretty_env_logger::init();
}
