use bevy::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, States)]
pub enum AppState {
    #[default]
    Splash,
    MainMenu,
    GameLoading,
    InGame,
}
