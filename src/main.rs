use bevy::prelude::*;
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};

use net_ninjas::GamePlugin;

const APP_TITLE: &str = "Net Ninjas 🥷";

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: APP_TITLE.to_string(),
            present_mode: bevy::window::PresentMode::AutoNoVsync, // Reduces input lag.
            fit_canvas_to_parent: true,
            ..default()
        }),
        ..default()
    }))
    .add_plugins(GamePlugin);

    #[cfg(feature = "diagnostics")]
    {
        app.add_plugins(ScreenDiagnosticsPlugin::default());
        app.add_plugins(ScreenFrameDiagnosticsPlugin);
    }

    app.run();
}
