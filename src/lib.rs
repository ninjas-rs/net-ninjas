use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use iyes_progress::{Progress, ProgressPlugin, ProgressReturningSystem, ProgressTracker};

use states::AppState;

mod assets;
mod core;
mod diagnostics;
mod states;
mod ui;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK))
            .add_plugins(
                ProgressPlugin::<AppState>::new()
                    .with_state_transition(AppState::Loading, AppState::InMenu),
            )
            .init_state::<AppState>()
            .add_loading_state(
                LoadingState::new(AppState::Loading)
                    .load_collection::<assets::ImageAssets>()
                    .load_collection::<assets::VideoAssets>()
                    .load_collection::<assets::AudioAssets>()
                    .load_collection::<assets::FontAssets>(),
            )
            .add_plugins((
                core::CorePlugin,
                // PlayerPlugin,
                // EnemyPlugin,
                // WorldPlugin,
                ui::UiPlugin,
            ))
            .add_systems(
                Update,
                fake_progress
                    .track_progress::<AppState>()
                    .run_if(in_state(AppState::Loading)),
            );

        #[cfg(feature = "diagnostics")]
        app.add_systems(Update, diagnostics::handle_appstate_change);
    }
}

fn fake_progress(time: Res<Time>, mut elapsed: Local<f32>) -> Progress {
    *elapsed += time.delta_secs();

    (*elapsed >= 2.0).into()
}
