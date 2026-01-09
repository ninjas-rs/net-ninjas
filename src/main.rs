use bevy::prelude::*;

const APP_TITLE: &str = "Net Ninjas 🥷";

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: APP_TITLE.to_string(),
                present_mode: bevy::window::PresentMode::AutoNoVsync, // Reduces input lag.
                fit_canvas_to_parent: true,
                ..default()
            }),
            ..default()
        }))
        .run();
}
