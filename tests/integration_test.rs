use sigmaos_app_center::{AppCatalog, AppCategory, AppCenterComponent, AppInstallState, AppMeta, Config, Component};

#[test]
fn test_catalog_add_search_aaa() {
    let mut catalog = AppCatalog::new();
    catalog.add(AppMeta::new("firefox", "Firefox Browser", AppCategory::Internet));
    catalog.add(AppMeta::new("gedit", "Gedit Text Editor", AppCategory::Utilities));
    let results = catalog.search("firefox");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "firefox");
}

#[test]
fn test_catalog_install_aaa() {
    let mut catalog = AppCatalog::new();
    catalog.add(AppMeta::new("vim", "Vim Editor", AppCategory::Development));
    let result = catalog.install("vim");
    assert!(result.is_ok());
    let app = catalog.get("vim").unwrap();
    assert_eq!(app.install_state, AppInstallState::Installed);
}

#[test]
fn test_catalog_by_category_aaa() {
    let mut catalog = AppCatalog::new();
    catalog.add(AppMeta::new("gimp", "GIMP Image Editor", AppCategory::Graphics));
    catalog.add(AppMeta::new("inkscape", "Inkscape Vector", AppCategory::Graphics));
    catalog.add(AppMeta::new("vlc", "VLC Media Player", AppCategory::Multimedia));
    let graphics = catalog.by_category(&AppCategory::Graphics);
    assert_eq!(graphics.len(), 2);
}

#[test]
fn test_component_lifecycle_aaa() {
    let mut c = AppCenterComponent::new(Config::default());
    assert!(c.initialize().is_ok());
    assert!(c.health_check().is_ok());
    assert!(c.shutdown().is_ok());
}

#[test]
fn test_component_name_aaa() {
    let c = AppCenterComponent::new(Config::default());
    assert_eq!(c.name(), "sigmaos-app-center");
}
