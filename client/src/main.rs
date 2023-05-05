use bevy::{prelude::*, window::PresentMode};

#[cfg(feature = "editor")]
use bevy_editor_pls::EditorPlugin;

mod app_state;
mod assets;
mod navigation;
mod ui;

const APP_TITLE: &str = "Net Ninjas 🥷";

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: APP_TITLE.to_string(),
            present_mode: PresentMode::AutoNoVsync, // Reduces input lag.
            fit_canvas_to_parent: true,
            ..default()
        }),
        ..default()
    }))
    .add_plugin(navigation::NavigationPlugin)
    .add_plugin(ui::UiPlugin)
    .add_startup_system(setup);

    #[cfg(feature = "editor")]
    app.add_plugin(EditorPlugin::default()); // only available if the "editor" feature is enabled

    app.run();
}

fn setup() {
    println!("Welcome to {}!", APP_TITLE);
}
