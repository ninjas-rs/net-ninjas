use bevy::prelude::*;

use crate::{app_state::AppState, assets::UiAssets};

use super::components::button::{self, button_state_change};

pub struct MainMenuUiPlugin;

impl Plugin for MainMenuUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(button_state_change.run_if(in_state(AppState::MainMenu)));
    }
}

#[derive(Component)]
pub struct JoinRoomButton;

#[derive(Component)]
pub struct CreateRoomButton;

#[derive(Component)]
pub struct MainMenuText;

pub fn setup(mut commands: Commands, ui_assets: Res<UiAssets>) {
    // Add background
    commands.spawn(SpriteBundle {
        texture: ui_assets.menu_background.clone(),
        ..Default::default()
    });

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                display: Display::Flex,
                justify_content: JustifyContent::SpaceEvenly,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(
                TextBundle::from_section(
                    "Net Ninjas",
                    TextStyle {
                        font: ui_assets.fira_sans_bold.clone(),
                        font_size: 128.0,
                        color: Color::BLACK.with_a(0.9),
                    },
                )
                .with_style(Style {
                    margin: UiRect::bottom(Val::Percent(10.)),
                    ..Default::default()
                }),
            );

            builder
                .spawn(NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        justify_content: JustifyContent::SpaceEvenly,
                        align_items: AlignItems::Stretch,
                        flex_direction: FlexDirection::Column,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|builder| {
                    button::build(
                        builder,
                        ui_assets.fira_sans_bold.clone(),
                        "Join Room",
                        Some(UiRect::bottom(Val::Percent(10.))),
                    );

                    button::build(
                        builder,
                        ui_assets.fira_sans_bold.clone(),
                        "Create Room",
                        None,
                    );
                });
        });
}
