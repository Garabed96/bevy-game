use crate::screens::game::TEXT_COLOR;
use crate::screens::game::{BOTTOM_WALL, GAP_BETWEEN_PADDLE_AND_FLOOR};
use crate::screens::move_character::sprite_movement;
use bevy::asset::{AssetServer, Assets, Handle};
use bevy::image::{Image, TextureAtlas, TextureAtlasLayout};
use bevy::math::{UVec2, Vec2, Vec3};
use bevy::prelude::*;
use bevy::prelude::{
    default, Commands, Component, Deref, DerefMut, JustifyText, Query, Res, ResMut, Sprite,
    SpriteImageMode, Text2d, TextColor, TextFont, TextLayout, Time, Timer, TimerMode, Transform,
};
use std::collections::HashMap;

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

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerState {
    Idle,
    Run,
    Roll,
    Jump,
    Die,
    Climb,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Weapon {
    #[default]
    Unarmed,
    Sword,
    Bow,
    Spear,
}

// One Atlas per weapon, TextureAtlas; in Bevy is basically a sprite-sheet
#[derive(Resource)]
pub struct IdleAtlases {
    pub map: HashMap<Weapon, (Handle<Image>, Handle<TextureAtlasLayout>)>,
}

#[derive(Component, Debug, Clone)]
pub struct Player {
    pub state: PlayerState,
    pub health: u32,
    pub max_health: u32,
    pub jump_speed: f32,
    pub speed: f32,
    pub gravity: f32,
    pub jump_count: u32,
}

pub fn setup_idle_atlases(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {

    let mut map = HashMap::new();

    // Unarmed / Sword / Bow is 8x(300x256):

    // Unarmed
    let unarmed_character_idle: Handle<Image> =
        asset_server.load("textures/player/idle/unarmed_player_idle.png");
    let layout_unarmed_idle = TextureAtlasLayout::from_grid(UVec2::new(300, 256), 8, 1, None, None);
    let handle_unarmed_handle = layouts.add(layout_unarmed_idle);
    map.insert(Weapon::Unarmed, (unarmed_character_idle.clone(), handle_unarmed_handle));

    // let animation_indices_character = AnimationIndices { first: 0, last: 7 };

    // Sword
    let sword_character_idle: Handle<Image> = asset_server.load("textures/player/idle/sword_idle.png");
    let layout_sword_idle = TextureAtlasLayout::from_grid(UVec2::new(300, 256), 8, 1, None, None);
    let handle_sword_handle = layouts.add(layout_sword_idle);
    map.insert(Weapon::Sword, (sword_character_idle.clone(), handle_sword_handle));

    // Bow
    let bow_character_idle: Handle<Image> = asset_server.load("textures/player/idle/bow_idle.png");
    let layout_bow_idle = TextureAtlasLayout::from_grid(UVec2::new(300, 256), 8, 1, None, None);
    let handle_bow_handle = layouts.add(layout_bow_idle);
    map.insert(Weapon::Bow, (bow_character_idle.clone(), handle_bow_handle));

    // Spear is 8x(540x320):
    let spear_character_idle: Handle<Image> = asset_server.load("textures/player/idle/spear_idle.png");
    let layout_spear_idle = TextureAtlasLayout::from_grid(UVec2::new(540, 320), 8, 1, None, None);
    let handle_spear_handle = layouts.add(layout_spear_idle);
    map.insert(Weapon::Spear, (spear_character_idle.clone(), handle_spear_handle));

    // store in a resource for later
    commands.insert_resource(IdleAtlases { map });
}

#[derive(Bundle)]
pub struct PlayerBundle {
    /// the `Sprite` with its atlas baked in
    sprite: Sprite,
    /// entity positioning in world
    transform: Transform,
    global_transform: GlobalTransform,

    /// your marker so you can `Query<With<Character>>`
    character: Character,
    /// current weapon choice
    weapon: Weapon,
    /// frame‐range for the current animation
    indices: AnimationIndices,
    /// driving the timer for the sprite flip
    timer: AnimationTimer,
}

pub fn setup_player(mut commands: Commands, atlases: Res<IdleAtlases>) {
    let (image_handle, layout_handle) = &atlases.map[&Weapon::Unarmed];
    let y = BOTTOM_WALL + GAP_BETWEEN_PADDLE_AND_FLOOR + 50.0;

    commands.spawn(PlayerBundle {
        sprite: Sprite::from_atlas_image(
            image_handle.clone(),
            TextureAtlas {
                layout: layout_handle.clone(),
                index: 0,
            },
        ),
        transform: Transform {
            translation: Vec3::new(0.0, y, 0.0),
            scale: Vec3::splat(0.25),
            ..Default::default()
        },
        global_transform: Default::default(),
        character: Character,
        weapon: Weapon::Unarmed,
        indices: AnimationIndices { first: 0, last: 7 },
        timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    });
}
