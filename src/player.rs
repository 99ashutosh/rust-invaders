use crate::{
    components::{Player, Velocity},
    GameTextures, WinSize, PLAYER_SIZE, PLAYER_SPRITE, SPRITE_SCALE, TIME_STEP, BASE_SPEED,
};
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
            .add_system(player_movement_system);
    }
}

fn player_spawn_system(
    mut commands: Commands,
    game_testures: Res<GameTextures>,
    win_size: Res<WinSize>,
) {
    // Add a player
    let bottom = -win_size.h / 2.;

    commands
        .spawn_bundle(SpriteBundle {
            texture: game_testures.player.clone(),
            transform: Transform {
                translation: Vec3::new(0., bottom + PLAYER_SIZE.1 / 2. * SPRITE_SCALE + 5., 10.),
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(Velocity { x: 1., y: 0. });
}

fn player_movement_system(mut query: Query<(&Velocity, &mut Transform), With<Player>>) {
    for (velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
    }
}