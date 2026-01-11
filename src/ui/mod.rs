use bevy::prelude::*;

use crate::states::AppState;

mod components;
mod loading;
mod menu;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Loading), loading::spawn_loading);
        app.add_systems(OnExit(AppState::Loading), loading::despawn_loading);
        app.add_systems(
            Update,
            loading::update_loading.run_if(in_state(AppState::Loading)),
        );

        app.add_systems(OnEnter(AppState::InMenu), menu::spawn_menu);
        app.add_systems(OnExit(AppState::InMenu), menu::despawn_menu);
    }
}
