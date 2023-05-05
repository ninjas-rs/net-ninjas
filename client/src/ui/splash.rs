use bevy::prelude::*;

use crate::app_state::AppState;

pub struct SplashUiPlugin;

impl Plugin for SplashUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(AppState::Splash)))
            .add_system(update.run_if(in_state(AppState::Splash)));
    }
}

pub fn setup() {
    println!("Splash setup")
}

pub fn update() {}
