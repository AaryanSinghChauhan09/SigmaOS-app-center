use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum AppCategory { Utilities, Development, Graphics, Internet, Multimedia, Office, System, Games }

#[derive(Debug, Clone, PartialEq)]
pub enum AppInstallState { NotInstalled, Installing, Installed, UpdateAvailable, Removing }

#[derive(Debug, Clone)]
pub struct AppMeta {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub category: AppCategory,
    pub author: String,
    pub install_state: AppInstallState,
    pub stars: u32,
}
impl AppMeta {
    pub fn new(id: &str, name: &str, category: AppCategory) -> Self {
        Self {
            id: id.to_string(), name: name.to_string(), description: String::new(),
            version: "1.0.0".to_string(), category, author: String::new(),
            install_state: AppInstallState::NotInstalled, stars: 0,
        }
    }
}

pub struct AppCatalog {
    apps: HashMap<String, AppMeta>,
}
impl AppCatalog {
    pub fn new() -> Self { Self { apps: HashMap::new() } }
    pub fn add(&mut self, app: AppMeta) { self.apps.insert(app.id.clone(), app); }
    pub fn get(&self, id: &str) -> Option<&AppMeta> { self.apps.get(id) }
    pub fn search(&self, query: &str) -> Vec<&AppMeta> {
        let q = query.to_lowercase();
        self.apps.values().filter(|a| a.name.to_lowercase().contains(&q) || a.description.to_lowercase().contains(&q)).collect()
    }
    pub fn by_category(&self, cat: &AppCategory) -> Vec<&AppMeta> {
        self.apps.values().filter(|a| &a.category == cat).collect()
    }
    pub fn install(&mut self, id: &str) -> Result<(), String> {
        match self.apps.get_mut(id) {
            Some(a) => { a.install_state = AppInstallState::Installed; Ok(()) }
            None => Err(format!("app '{}' not found", id))
        }
    }
    pub fn count(&self) -> usize { self.apps.len() }
}
impl Default for AppCatalog { fn default() -> Self { Self::new() } }
