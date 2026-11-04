use sigmaos_app_center::{Config, AppCenterComponent, Component};
fn main() {
    env_logger::init();
    let mut c = AppCenterComponent::new(Config::default());
    if let Err(e) = c.initialize() { eprintln!("Failed: {}", e); std::process::exit(1); }
    println!("SigmaOS app-center running");
    let _ = c.shutdown();
}
