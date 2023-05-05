use bevy::prelude::*;

use crate::{app_state::AppState, assets::UiAssets};

pub struct MainMenuUiPlugin;

impl Plugin for MainMenuUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(update.run_if(in_state(AppState::MainMenu)));
    }
}

pub fn setup(mut commands: Commands, ui_assets: Res<UiAssets>) {
    commands.spawn(ImageBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            position_type: PositionType::Absolute,
            ..default()
        },
        image: UiImage::new(ui_assets.menu_background.clone()),
        ..default()
    });
}

pub fn update() {}
