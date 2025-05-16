use crate::screens::character::{Character, Weapon, IdleAtlases};
use bevy::prelude::*;

pub fn weapon_switch(keys: Res<ButtonInput<KeyCode>>, mut query: Query<&mut Weapon, With<Character>>) {
    for mut weapon in &mut query {
        if keys.just_pressed(KeyCode::Digit0) {
            *weapon = Weapon::Unarmed;
        } else if keys.just_pressed(KeyCode::Digit1) {
            *weapon = Weapon::Sword;
        } else if keys.just_pressed(KeyCode::Digit2) {
            *weapon = Weapon::Bow;
        } else if keys.just_pressed(KeyCode::Digit3) {
            *weapon = Weapon::Spear;
        }
    }
}

/// Whenever your `Weapon` component changes, swap the sprite’s image + atlas.
pub fn apply_weapon_change(
    atlases: Res<IdleAtlases>,
    mut query: Query<(&Weapon, &mut Sprite), Changed<Weapon>>,
) {
    for (weapon, mut sprite) in &mut query {
        // look up the (image, layout) we stored for this weapon
        let (image_handle, layout_handle) = &atlases.map[weapon];

        // repoint the sprite to the new texture *handle*
        sprite.image = image_handle.clone();                  // ← use `image`, not `texture` :contentReference[oaicite:0]{index=0}

        // swap in the new atlas, and reset to frame 0
        sprite.texture_atlas = Some(TextureAtlas {
            layout: layout_handle.clone(),
            index: 0,
        });
    }
}