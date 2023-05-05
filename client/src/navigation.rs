use bevy::prelude::*;
use iyes_progress::prelude::*;

use crate::{app_state::AppState, assets::load_ui_assets};

pub struct NavigationPlugin;

impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .add_plugin(
                ProgressPlugin::new(AppState::Splash)
                    .continue_to(AppState::MainMenu)
                    .track_assets(),
            )
            .add_plugin(ProgressPlugin::new(AppState::GameLoading).continue_to(AppState::InGame))
            .add_system(load_ui_assets.in_schedule(OnEnter(AppState::Splash)));
    }
}
