use crate::objects::{Player, PlayerBundle};

use bevy::{
    prelude::{
        Assets,
        Camera2dBundle,
        Color, ColorMaterial,
        Commands,
        default,
        Input, KeyCode,
        Mesh,
        Query, With, Res, ResMut,
        shape,
        Transform, Vec3,
    },
    sprite::MaterialMesh2dBundle,
};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0., 0., 0.),
        projection: bevy::prelude::OrthographicProjection {
            scaling_mode: bevy::render::camera::ScalingMode::Fixed{
                width: 0.01,
                height: 0.01,
            },
            ..default()
        },
        ..default()
    });
    commands.spawn(PlayerBundle {
        material_bundle: MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3::splat(200.)),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            ..default()
        },
        player: Player {
            gravity: Vec3::new(0., -1.0, 0.),
            velocity: Vec3::ZERO,
            jumping: false,
            ..default()
        },
    });
}

pub fn player_update(mut players: Query<(&mut Player, &mut Transform)>) {
    for (mut player, mut transform) in &mut players {
        if transform.translation.y <= 0. {
            if player.building_up {
                player.build_up_frames += 1.;
            }
            if player.jumping {
                player.velocity.y = player.build_up_frames;
                player.build_up_frames = 10.;
            }
        }
        player.building_up = false;
        player.jumping = false;

        transform.translation += player.velocity;
        if transform.translation.y < 0. {
            transform.translation.y = 0.;
        }
        let grav = player.gravity;
        player.velocity += grav;
    }
}

pub fn handle_inputs(
    keyboard: Res<Input<KeyCode>>,
    mut players: Query<&mut Player>,
) {
    let holding = keyboard.pressed(KeyCode::Space);
    let release = keyboard.just_released(KeyCode::Space);
    players.for_each_mut(|mut player| {
        player.building_up |= holding;
        player.jumping |= release;
    });
}
