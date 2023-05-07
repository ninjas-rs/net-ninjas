use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_screen_diagnostics::ScreenDiagnosticsPlugin;

mod components;
mod debug;
mod main_menu;
mod splash;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(splash::SplashUiPlugin)
            .add_plugin(main_menu::MainMenuUiPlugin)
            .add_plugin(ScreenDiagnosticsPlugin {
                font: "fonts/FiraSans-Bold.ttf".into(),
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        top: Val::Px(5.0),
                        left: Val::Px(10.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            })
            .add_plugin(FrameTimeDiagnosticsPlugin)
            .add_plugin(debug::FpsDiagnosticsPlugin);
    }
}
