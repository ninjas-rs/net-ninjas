use bevy::prelude::*;
use iyes_progress::ProgressTracker;

use crate::states::AppState;

#[derive(Component)]
pub struct LoadingText;

pub fn spawn_loading(mut commands: Commands) {
    commands.spawn((
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            position_type: PositionType::Relative,
            height: Val::Percent(100.),
            width: Val::Percent(100.),
            ..Default::default()
        },
        children![
            (
                Text::new("net ninjas."),
                TextFont::default().with_font_size(36.),
            ),
            (Text::new(format!("Loading... 0%")), LoadingText)
        ],
    ));
}

pub fn despawn_loading(mut commands: Commands, query: Query<Entity, With<Text>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn update_loading(
    progress_tracker: Res<ProgressTracker<AppState>>,
    mut last_done: Local<u32>,
    mut query: Query<&mut Text, With<LoadingText>>,
) {
    let progress = progress_tracker.get_global_progress();

    if progress.done > *last_done {
        *last_done = progress.done;
    }

    let total = if progress.total == 0 {
        1
    } else {
        progress.total
    };
    let percentage_done = (progress.done / total) * 100;
    if let Ok(mut text) = query.single_mut() {
        *text = Text::new(format!("Loading... {}%", percentage_done));
    }
}
