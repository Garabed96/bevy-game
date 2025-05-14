use crate::screens::game::TEXT_COLOR;
use bevy::asset::{AssetServer, Assets, Handle};
use bevy::image::{Image, TextureAtlas, TextureAtlasLayout};
use bevy::math::{UVec2, Vec2, Vec3};
use bevy::prelude::{
    Commands, Component, Deref, DerefMut, JustifyText, Query, Res, ResMut, Sprite, SpriteImageMode,
    Text2d, TextColor, TextFont, TextLayout, Time, Timer, TimerMode, Transform,
};

use crate::screens::move_character::{sprite_movement};

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

// Setup the character
#[derive(Component)]
pub struct Character;

#[derive(Component, Clone)]
pub struct AnimationIndices {
    first: usize,
    last: usize,
}

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = if atlas.index == indices.last {
                    indices.first
                } else {
                    atlas.index + 1
                };
            }
        }
    }
}

struct SpriteSheet {
    size: Vec2,
    text: String,
    transform: Transform,
    texture: Handle<Image>,
    image_mode: SpriteImageMode,
    atlas: TextureAtlas,
    indices: AnimationIndices,
    timer: AnimationTimer,
}

pub fn setup_texture_character(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    // CHARACTER COMPONENTS

    let character: Handle<Image> = asset_server.load("textures/player_idle_sheet.png");
    let animation_indices_character = AnimationIndices { first: 0, last: 7 };
    let character_atlas = TextureAtlas {
        layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
            UVec2::new(300, 256),
            8,
            1,
            None,
            None,
        )),
        index: animation_indices_character.first,
    };

    // TODO: Add ability to name your character, save in json for now
    let sprint_sheets = [SpriteSheet {
        size: Vec2::new(300., 256.),
        text: "Champ Champ".to_string(),
        transform: Transform {
            translation: Vec3::new(1. * 300. * 0.25, 0.0, 0.0),
            ..Transform::from_scale(Vec3::splat(0.25))
        },
        texture: character.clone(),
        image_mode: SpriteImageMode::Auto,
        atlas: character_atlas.clone(),
        indices: animation_indices_character.clone(),
        timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    }];

    for sprite_sheet in sprint_sheets {
        let mut cmd = commands.spawn((
            Sprite {
                image_mode: sprite_sheet.image_mode,
                custom_size: Some(sprite_sheet.size),
                ..Sprite::from_atlas_image(sprite_sheet.texture.clone(), sprite_sheet.atlas.clone())
            },
            Character,
            sprite_sheet.indices,
            sprite_sheet.timer,
            sprite_sheet.transform,
        ));

        cmd.with_children(|builder| {
            builder.spawn((
                Text2d::new(sprite_sheet.text),
                TextLayout::new_with_justify(JustifyText::Center),
                TextColor(TEXT_COLOR),
                TextFont::from_font_size(55.),
                Transform::from_xyz(0., -256. * 0.5 - 10., 0.),
                bevy::sprite::Anchor::TopCenter,
            ));
        });
    }
}
