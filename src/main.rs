#![allow(unused)]

use bevy::prelude::*;




fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04,0.04,0.04)))
        .insert_resource(WindowDescriptor {
            title: "Rust Invaders".to_string(),
            width: 598.0,
            height:676.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_system)
        .run();
}


fn setup_system(mut commands: Commands) {
    // Add a camera
    commands.spawn_bundle(Camera2dBundle::default());

    // Add a rectangle
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.25),
            custom_size: Some(Vec2::new(150.,150.)),
            ..Default::default()
        },
        ..Default::default()
    });
}