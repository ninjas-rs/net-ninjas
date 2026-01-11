#![allow(dead_code)]

use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    Loading,
    InMenu,
    InGame,
    Paused,
}
