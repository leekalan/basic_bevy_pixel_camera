use std::f32::consts::PI;

use basic_bevy_pixel_camera::{
    pixel_camera::create_pixel_camera,
    pixel_camera_snapping::PixelCameraSnapping,
    pixel_canvas::{create_pixel_canvas, PixelCanvasConfig},
    pixel_canvas_smoothing::PixelCanvasSmoothing,
    pixel_image::create_pixel_image,
    BasicPixelCameraPlugin,
};
use bevy::{
    prelude::*,
    render::{camera::ScalingMode, view::RenderLayers},
};

pub fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BasicPixelCameraPlugin))
        .add_systems(Startup, (setup_cameras, spawn_sprite))
        .add_systems(Update, (rotate_box, camera_controller))
        .run();
}

pub const PIXELS_PER_UNIT: f32 = 8.0;
pub const HIGH_RES_LAYER: RenderLayers = RenderLayers::layer(0);
pub const PIXEL_PERFECT_LAYER: RenderLayers = RenderLayers::layer(1);

#[derive(Component)]
pub struct MainCamera;

pub fn setup_cameras(mut commands: Commands, images: ResMut<Assets<Image>>) {
    let mut cam_2d = Camera2dBundle::default();
    cam_2d.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 16.0,
        min_height: 8.0,
    };
    let main_cam = commands.spawn((cam_2d, HIGH_RES_LAYER, MainCamera)).id();

    let image = create_pixel_image(images);

    let pixel_camera = create_pixel_camera(commands.reborrow(), image.clone(), PIXEL_PERFECT_LAYER);

    let pixel_canvas = create_pixel_canvas(
        &PixelCanvasConfig::new(PIXELS_PER_UNIT, 8.0, 4.0),
        commands.reborrow(),
        image,
        pixel_camera,
        HIGH_RES_LAYER,
    );

    commands.entity(pixel_camera).insert(PixelCameraSnapping);
    commands.entity(pixel_canvas).insert(PixelCanvasSmoothing);
    commands
        .entity(main_cam)
        .push_children(&[pixel_camera, pixel_canvas]);
}

fn spawn_sprite(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(1.0, 1.0, 0.0),
                rotation: Quat::from_rotation_z(PI / 4.0),
                ..default()
            },
            ..default()
        },
        Box,
        PIXEL_PERFECT_LAYER,
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(-1.0, -1.0, 0.0),
                rotation: Quat::from_rotation_z(PI / 4.0),
                ..default()
            },
            ..default()
        },
        Box,
        HIGH_RES_LAYER,
    ));
}

#[derive(Component)]
pub struct Box;

pub const RADS_PER_SECOND: f32 = PI / 8.0;

pub fn rotate_box(mut transform: Query<&mut Transform, With<Box>>, time: Res<Time>) {
    for mut transform in &mut transform {
        transform.rotate(Quat::from_rotation_z(
            RADS_PER_SECOND * time.delta_seconds(),
        ));
    }
}

fn camera_controller(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<MainCamera>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for mut transform in &mut query {
        if keyboard_input.pressed(KeyCode::KeyW) {
            transform.translation.y += PIXELS_PER_UNIT * time.delta_seconds() * 0.1;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            transform.translation.y -= PIXELS_PER_UNIT * time.delta_seconds() * 0.1;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            transform.translation.x -= PIXELS_PER_UNIT * time.delta_seconds() * 0.1;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            transform.translation.x += PIXELS_PER_UNIT * time.delta_seconds() * 0.1;
        }
    }
}
