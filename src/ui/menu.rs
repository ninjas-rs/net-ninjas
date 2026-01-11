use bevy::prelude::*;

use crate::assets::ImageAssets;

pub fn spawn_menu(mut commands: Commands, image_assets: Res<ImageAssets>) {
    commands.spawn(Sprite::from_image(image_assets.logo.clone()));
}

pub fn despawn_menu(mut commands: Commands, query: Query<Entity, With<Sprite>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
