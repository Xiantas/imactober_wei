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
        transform: Transform::from_xyz(200., 150., 0.),
        ..default()
    });
    commands.spawn(PlayerBundle {
        material_bundle: MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3::splat(128.)),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            ..default()
        },
        player: Player {
            gravity: Vec3::new(0., -0.6, 0.),
            velocity: Vec3::ZERO,
            jumping: false,
        },
    });
}

pub fn player_update(mut players: Query<(&mut Player, &mut Transform)>) {
    for (mut player, mut transform) in &mut players {
        if player.jumping {
            if transform.translation.y <= 0. {
                player.velocity.y = 20.;
            }
            player.jumping = false;
        }

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
    if keyboard.just_pressed(KeyCode::Space) {
        players.for_each_mut(|mut player| {
            player.jumping = true;
        });
    }
}
