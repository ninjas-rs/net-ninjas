use bevy::prelude::*;
use iyes_progress::prelude::*;

use crate::app_state::AppState;

pub struct SplashUiPlugin;

impl Plugin for SplashUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(AppState::Splash)))
            .add_system(update.run_if(in_state(AppState::Splash)))
            .add_system(
                fake_loading
                    .track_progress()
                    .run_if(in_state(AppState::Splash)),
            );
    }
}

#[derive(Component)]
pub struct LoadingText;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Add loading text
    commands
        .spawn(Text2dBundle {
            text: Text::from_sections([TextSection {
                value: "Loading... ".to_string(),
                style: TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 60.0,
                    color: Color::WHITE,
                },
            }]),
            ..default()
        })
        .insert(LoadingText);
}

pub fn update() {}

fn fake_loading(time: Res<Time>) -> Progress {
    if time.elapsed_seconds_f64() > 3.0 {
        info!("Long task is completed");
        true.into()
    } else {
        false.into()
    }
}
