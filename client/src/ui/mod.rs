use bevy::prelude::*;

mod debug;
mod main_menu;
mod splash;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(debug::DebugUiPlugin)
            .add_plugin(splash::SplashUiPlugin)
            .add_plugin(main_menu::MainMenuUiPlugin);
    }
}
