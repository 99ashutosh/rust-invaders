use std::f32::consts::PI;

use bevy::{prelude::*, time::FixedTimestep, ecs::schedule::ShouldRun};
use rand::{thread_rng, Rng};

use crate::{
    components::{Enemy, SpriteSize, Movable, Laser, FromEnemy, Velocity},
    EnemyCount, GameTextures, WinSize, ENEMY_LASER_SIZE, ENEMY_MAX, ENEMY_SIZE, PLAYER_SIZE,
    SPRITE_SCALE,
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.))
                .with_system(enemy_spawn_system),
        )
        .add_system_set(
            SystemSet::new()
            .with_run_criteria(enemy_fire_criteria)
            .with_system(enemy_fire_system),
        );

    }
}

fn enemy_spawn_system(
    mut commands: Commands,
    game_testures: Res<GameTextures>,
    win_size: Res<WinSize>,
    mut enemy_count: ResMut<EnemyCount>,
) {
    if enemy_count.0 < ENEMY_MAX {
        let mut rng = thread_rng();
        let w_span = win_size.w / 2. - 100.;
        let h_span = win_size.h / 2. - 100.;
        let x = rng.gen_range(-w_span..w_span);
        let y = rng.gen_range(-h_span..h_span);

        commands
            .spawn_bundle(SpriteBundle {
                texture: game_testures.enemy.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, 10.),
                    scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Enemy)
            .insert(SpriteSize::from(ENEMY_SIZE));

        enemy_count.0 += 1;
    }
}

fn enemy_fire_criteria() -> ShouldRun {
    if thread_rng().gen_bool(1./60.) {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn enemy_fire_system (
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    for &tf in enemy_query.iter() {
        let (x,y) = (tf.translation.x, tf.translation.y);
        commands
        .spawn_bundle(SpriteBundle {
            texture: game_textures.enemy_laser.clone(),
            transform: Transform {
                translation: Vec3::new(x, y - 15., 0.),
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                rotation: Quat::from_rotation_x(PI),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Laser)
        .insert(SpriteSize::from(ENEMY_LASER_SIZE))
        .insert(FromEnemy)
        .insert(Movable {auto_despawn: true})
        .insert(Velocity {x: 0., y: -1.});
    }
}