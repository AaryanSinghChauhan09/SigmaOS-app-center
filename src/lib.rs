#[derive(Debug, Clone)]
pub struct AppListing {
    pub app_id: String,
    pub name: String,
    pub required_permissions: Vec<String>,
}

#[derive(Debug, Default)]
pub struct AppCenterCatalog {
    apps: Vec<AppListing>,
}

impl AppCenterCatalog {
    pub fn new() -> Self { Self::default() }
    pub fn register(&mut self, id: &str, name: &str, perms: Vec<&str>) {
        self.apps.push(AppListing {
            app_id: id.to_string(),
            name: name.to_string(),
            required_permissions: perms.into_iter().map(String::from).collect(),
        });
    }
    pub fn count(&self) -> usize { self.apps.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_app_center_registration() {
        let mut catalog = AppCenterCatalog::new();
        catalog.register("io.elementary.code", "Code Editor", vec!["filesystem.readwrite"]);
        assert_eq!(catalog.count(), 1);
    }
}
