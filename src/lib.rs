use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use states::AppState;

mod assets;
mod states;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_loading_state(
                LoadingState::new(AppState::Loading)
                    .continue_to_state(AppState::InMenu)
                    .load_collection::<assets::ImageAssets>()
                    .load_collection::<assets::VideoAssets>()
                    .load_collection::<assets::AudioAssets>(),
            )
            .add_plugins((
            // CorePlugin,
            // PlayerPlugin,
            // EnemyPlugin,
            // WorldPlugin,
            // UiPlugin,
        ));
    }
}
