use bevy::{
    prelude::*,
    render::{camera::ScalingMode, render_resource::Extent3d, view::RenderLayers},
};

use crate::{pixel_camera::PixelCamera, pixel_canvas_smoothing::PixelCanvasSmoothing};

#[derive(Component, Debug, PartialEq, Clone, Copy)]
pub struct PixelCanvas {
    pub pixels_per_unit: f32,
    pub unit_width: f32,
    pub unit_height: f32,
    pub camrera_id: Entity,
    pub update_image: bool,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PixelCanvasConfig {
    pub pixels_per_unit: f32,
    pub unit_width: f32,
    pub unit_height: f32,
}

impl PixelCanvasConfig {
    pub const fn new(pixels_per_unit: f32, unit_width: f32, unit_height: f32) -> Self {
        Self {
            pixels_per_unit,
            unit_width,
            unit_height,
        }
    }
}

pub fn generate_pixel_camera_target_extent(
    pixels_per_unit: f32,
    unit_width: f32,
    unit_height: f32,
) -> Extent3d {
    Extent3d {
        width: (pixels_per_unit * unit_width + 2.0) as u32,
        height: (pixels_per_unit * unit_height + 2.0) as u32,
        ..default()
    }
}

pub fn generate_pixel_canvas_rect(
    pixels_per_unit: f32,
    unit_width: f32,
    unit_height: f32,
    offset_x: f32,
    offset_y: f32,
) -> Rect {
    Rect {
        min: Vec2::new(1.0 + offset_x, 1.0 + offset_y),
        max: Vec2::new(
            1.0 + offset_x + (pixels_per_unit * unit_width),
            1.0 + offset_y + (pixels_per_unit * unit_height),
        ),
    }
}

pub fn create_pixel_canvas(
    config: &PixelCanvasConfig,
    mut commands: Commands,
    image: Handle<Image>,
    camera: Entity,
    render_layers: RenderLayers,
) -> Entity {
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(config.unit_width, config.unit_height)),
                    rect: Some(generate_pixel_canvas_rect(
                        config.pixels_per_unit,
                        config.unit_width,
                        config.unit_height,
                        0.0,
                        0.0,
                    )),
                    ..default()
                },
                texture: image,
                transform: Transform::from_xyz(0.0, 0.0, -1.0),
                ..default()
            },
            PixelCanvas {
                pixels_per_unit: config.pixels_per_unit,
                unit_width: config.unit_width,
                unit_height: config.unit_height,
                camrera_id: camera,
                update_image: true,
            },
            render_layers,
        ))
        .id()
}

pub fn update_pixel_canvas_sprite(mut query: Query<(&mut Sprite, &PixelCanvas)>) {
    for (mut sprite, pixel_canvas) in &mut query {
        sprite.custom_size = Some(Vec2::new(pixel_canvas.unit_width, pixel_canvas.unit_height));
    }
}

pub fn update_pixel_canvas(
    mut images: ResMut<Assets<Image>>,
    mut query: Query<(&Handle<Image>, &mut PixelCanvas)>,
) {
    for (image_handle, mut pixel_canvas) in &mut query {
        if pixel_canvas.update_image {
            images
                .get_mut(image_handle)
                .unwrap()
                .resize(generate_pixel_camera_target_extent(
                    pixel_canvas.pixels_per_unit,
                    pixel_canvas.unit_width,
                    pixel_canvas.unit_height,
                ));

            pixel_canvas.update_image = false
        }
    }
}

pub fn update_pixel_camera(
    mut query: Query<&PixelCanvas>,
    mut projections: Query<&mut OrthographicProjection, With<PixelCamera>>,
) {
    for pixel_canvas in &mut query {
        if let Ok(mut projection) = projections.get_mut(pixel_canvas.camrera_id) {
            projection.scaling_mode = ScalingMode::WindowSize(pixel_canvas.pixels_per_unit);
        }
    }
}

pub fn update_pixel_canvas_rect(
    mut query: Query<(&mut Sprite, &PixelCanvas), Without<PixelCanvasSmoothing>>,
) {
    for (mut sprite, pixel_canvas) in &mut query {
        sprite.rect = Some(generate_pixel_canvas_rect(
            pixel_canvas.pixels_per_unit,
            pixel_canvas.unit_width,
            pixel_canvas.unit_height,
            0.0,
            0.0,
        ))
    }
}
