use bevy::prelude::*;


pub fn point_light(mut commands:  Commands) {
    commands.spawn_bundle(
        PointLightBundle {
            point_light: PointLight {
                intensity: 1800.0,
                shadows_enabled: true,
                ..default()
        },
        transform: Transform::from_xyz(0.0, 16.0, 0.0),
        ..default()
    });
}