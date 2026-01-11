#![allow(dead_code)]

use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    #[asset(path = "images/logo.png")]
    pub logo: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct VideoAssets {}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {}

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/CalSans-Regular.ttf")]
    pub cal_sans: Handle<Font>,
}
