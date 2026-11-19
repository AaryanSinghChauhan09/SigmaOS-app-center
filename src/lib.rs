#![forbid(unsafe_code)]
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


impl AppCenterCatalog {
    pub fn find_by_id(&self, id: &str) -> Option<&AppListing> {
        self.apps.iter().find(|a| a.app_id == id)
    }
    pub fn is_empty(&self) -> bool {
        self.apps.is_empty()
    }
    pub fn permissions_for(&self, id: &str) -> Vec<String> {
        self.find_by_id(id)
            .map(|a| a.required_permissions.clone())
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod app_center_extra_tests {
    use super::*;

    #[test]
    fn test_find_by_id_existing_aaa() {
        // Arrange
        let mut catalog = AppCenterCatalog::new();
        catalog.register("org.editor", "Text Editor", vec!["filesystem"]);
        // Act
        let found = catalog.find_by_id("org.editor");
        // Assert
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "Text Editor");
    }

    #[test]
    fn test_find_by_id_missing_returns_none_aaa() {
        // Arrange
        let catalog = AppCenterCatalog::new();
        // Act
        let found = catalog.find_by_id("non.existent");
        // Assert
        assert!(found.is_none());
        assert!(catalog.is_empty());
    }

    #[test]
    fn test_permissions_retrieval_aaa() {
        // Arrange
        let mut catalog = AppCenterCatalog::new();
        catalog.register("org.camera", "Camera App", vec!["video", "microphone"]);
        // Act
        let perms = catalog.permissions_for("org.camera");
        // Assert
        assert_eq!(perms, vec!["video", "microphone"]);
        assert!(catalog.permissions_for("missing").is_empty());
    }

    #[test]
    fn test_default_catalog_is_empty_aaa() {
        // Arrange & Act
        let catalog = AppCenterCatalog::default();
        // Assert
        assert_eq!(catalog.count(), 0);
        assert!(catalog.is_empty());
    }
}
