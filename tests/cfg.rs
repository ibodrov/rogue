extern crate cfg;

#[test]
fn test_load_standard_assets() {
    cfg::assets::load_atlas("assets/atlas_ascii.json").unwrap();
    cfg::ui::load("assets/ui.json").unwrap();
}
