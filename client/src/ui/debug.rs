use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_screen_diagnostics::{Aggregate, ScreenDiagnostics};

pub struct FpsDiagnosticsPlugin;

impl Plugin for FpsDiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup);
    }
}

#[derive(Component)]
pub struct FpsText;

pub fn startup(mut diags: ResMut<ScreenDiagnostics>) {
    diags
        .add("fps".to_string(), FrameTimeDiagnosticsPlugin::FPS)
        .aggregate(Aggregate::Value)
        .diagnostic_color(Color::RED)
        .toggle_name()
        .format(|v| format!("FPS: {v:.0}"));
}
