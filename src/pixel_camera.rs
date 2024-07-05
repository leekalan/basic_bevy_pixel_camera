use bevy::{
    prelude::*,
    render::{camera::RenderTarget, view::RenderLayers},
};

#[derive(Component, Default, Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct PixelCamera;

pub fn create_pixel_camera(
    mut commands: Commands,
    image: Handle<Image>,
    render_layers: RenderLayers,
) -> Entity {
    let camera = Camera2dBundle {
        camera: Camera {
            order: -1,
            target: RenderTarget::Image(image),
            ..default()
        },
        ..default()
    };

    commands.spawn((camera, render_layers, PixelCamera)).id()
}
