use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(AssetCollection, Resource)]
pub struct UiAssets {
    #[asset(path = "img/menu_background.png")]
    pub menu_background: Handle<Image>,

    #[asset(path = "fonts/FiraMono-Medium.ttf")]
    pub fira_mono_medium: Handle<Font>,

    #[asset(path = "fonts/FiraMono-Regular.ttf")]
    pub fira_mono_regular: Handle<Font>,

    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans_bold: Handle<Font>,
}
