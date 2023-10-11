use bevy::{
    ecs::{
        bundle::Bundle,
        component::Component,
    },
    math::Vec3,
    sprite::{
        Material2d,
        MaterialMesh2dBundle,
    },
};

#[derive(Component, Default)]
pub struct Player {
    pub jumping: bool,
    pub building_up: bool,
    pub build_up_frames: f32,

    pub velocity: Vec3,
    pub gravity: Vec3,
}

#[derive(Bundle, Default)]
pub struct PlayerBundle<M: Material2d> {
    pub player: Player,
    pub material_bundle: MaterialMesh2dBundle<M>,
}
