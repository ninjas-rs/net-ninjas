use bevy::prelude::*;

#[cfg(feature = "editor")]
use bevy_editor_pls::EditorPlugin;

mod app_state;
mod assets;
mod navigation;
mod ui;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugin(navigation::NavigationPlugin)
        .add_plugin(ui::UiPlugin)
        .add_startup_system(setup);

    #[cfg(feature = "editor")]
    app.add_plugin(EditorPlugin::default()); // only available if the "editor" feature is enabled

    app.run();
}

fn setup() {
    println!("Welcome to Net Ninjas 🥷")
}
