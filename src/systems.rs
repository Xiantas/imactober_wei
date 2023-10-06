use crate::objects::{Player, PlayerBundle};

use bevy::{
    prelude::{
        Assets,
        Camera2dBundle,
        Color,
        ColorMaterial,
        Commands,
        default,
        Input,
        KeyCode,
        Mesh,
        Query,
        Res,
        ResMut,
        shape,
        Transform,
        Vec3,
        With,
    },
    sprite::MaterialMesh2dBundle,
};

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(PlayerBundle {
        material_bundle: MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform::from_xyz(100., 100., 0.).with_scale(Vec3::splat(128.)),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            ..default()
        },
        ..default()
    });
}

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
pub fn player_update(mut sprite_position: Query<&mut Transform, With<Player>>) {
    for mut transform in &mut sprite_position {
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
