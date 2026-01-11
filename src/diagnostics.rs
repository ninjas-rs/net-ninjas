#[cfg(feature = "diagnostics")]
use bevy::ecs::component;
use bevy::prelude::*;

use crate::states::AppState;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DiagnosticsText;

#[cfg(feature = "diagnostics")]
pub fn handle_appstate_change(
    mut commands: Commands,
    app_state: Res<State<AppState>>,
    mut query: Query<&mut Text, With<DiagnosticsText>>,
) {
    if let Ok(mut text) = query.single_mut() {
        *text = Text::from(format!("{:?}", app_state.get()));
    } else {
        commands.spawn((
            Text::from(format!("{:?}", app_state.get())),
            DiagnosticsText,
        ));
    }
}
