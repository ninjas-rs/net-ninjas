use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use iyes_progress::prelude::*;

use crate::{app_state::AppState, assets::UiAssets};

pub struct NavigationPlugin;

impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .add_loading_state(LoadingState::new(AppState::Splash))
            .add_collection_to_loading_state::<_, UiAssets>(AppState::Splash)
            .add_plugin(ProgressPlugin::new(AppState::Splash).continue_to(AppState::MainMenu));
    }
}
