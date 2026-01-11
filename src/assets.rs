use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {}

#[derive(AssetCollection, Resource)]
pub struct VideoAssets {}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {}
