pub mod catalog;
pub mod config;
pub mod error;
pub mod service;
pub use catalog::{AppCategory, AppCatalog, AppInstallState, AppMeta};
pub use config::Config;
pub use error::{ComponentError, Result};
pub use service::{Service, ServiceStatus};
pub trait Component {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn initialize(&mut self) -> error::Result<()>;
    fn shutdown(&mut self) -> error::Result<()>;
    fn health_check(&self) -> error::Result<()>;
}
pub struct AppCenterComponent { service: service::Service }
impl AppCenterComponent {
    pub fn new(config: Config) -> Self { Self { service: service::Service::new(config) } }
}
impl Component for AppCenterComponent {
    fn name(&self) -> &str { "sigmaos-app-center" }
    fn version(&self) -> &str { env!("CARGO_PKG_VERSION") }
    fn initialize(&mut self) -> error::Result<()> { self.service.start() }
    fn shutdown(&mut self) -> error::Result<()> { self.service.stop() }
    fn health_check(&self) -> error::Result<()> {
        if self.service.is_running() { Ok(()) }
        else { Err(error::ComponentError::Service("app-center not running".to_string())) }
    }
}
