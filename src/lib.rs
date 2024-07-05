//! # Pixel Camera
//!
//! A simple set of coponents, schedules, systems and helper functions to create any type of pixel camera
//!
//! **NOTE** this is a work in progress, and if you have any feedback feel free to open an issue or pull request
//!
//! ## Usage
//!
//! The following example shows a simple usage of the libary. This example utilises the `PixelCameraSnapping`
//! and `PixelCanvasSmoothing` components to make it appear as if the the objects themselves are pixelated rather
//! than the camera. Yet if you want the retro style feel you can ommit the components
//!
//! ```rust
//! use bevy::{
//!     prelude::*,
//!     render::{camera::ScalingMode, view::RenderLayers},
//! };
//! use pixel_camera::prelude::*;
//!
//! fn main() {
//!     App::new()
//!         .add_plugins((DefaultPlugins, PixelCameraPlugin) // Ordering important as Pixel Camera add's its own schedule
//!         .add_systems(Startup, setup)
//!         .run();
//! }
//!
//! pub const PIXELS_PER_UNIT: f32 = 8.0;
//! pub const HIGH_RES_LAYER: RenderLayers = RenderLayers::layer(0);
//! pub const PIXEL_PERFECT_LAYER: RenderLayers = RenderLayers::layer(1);
//!
//! #[derive(Component)]
//! pub struct MainCamera;
//!
//! pub fn setup_cameras(mut commands: Commands, images: ResMut<Assets<Image>>) {
//!     let mut cam_2d = Camera2dBundle::default();
//!     cam_2d.projection.scaling_mode = ScalingMode::AutoMin {
//!         min_width: 16.0,
//!         min_height: 8.0,
//!     };
//!
//!     // Window camera to view the pixel canvas
//!     let main_cam = commands
//!         .spawn((cam_2d, HIGH_RES_LAYER, MainCamera))
//!         .id();
//!
//!     // Creating the Pixel image to be stored in the assests resource
//!     let image = create_pixel_image(images);
//!
//!     // Creating the pixel camera which is the place in which the pixelized image is rendered
//!     let pixel_camera = create_pixel_camera(commands.reborrow(), image.clone(), PIXEL_PERFECT_LAYER);
//!
//!     // Creating the pixel canvas which is the image upon which the pixelated render is viewed
//!     let pixel_canvas = create_pixel_canvas(
//!         &PixelCanvasConfig::new(PIXELS_PER_UNIT, 8.0, 4.0),
//!         commands.reborrow(),
//!         image,
//!         pixel_camera,
//!         HIGH_RES_LAYER,
//!     );
//!
//!     // Add pixel camera snapping. This stops the flickering artifacts of
//!     // non-pixelized sprites when the camera moves (this is subjective)
//!     commands.entity(pixel_camera).insert(PixelCameraSnapping);
//!
//!     // Add pixel canvas smoothing. This lerps the canvas rect to
//!     // make the pixelated view move smoothly.
//!     //
//!     // This requires pixel camera snapping, or every 5-10 seconds there
//!     // will be a highly noticeable flicker in the pixelated view
//!     commands.entity(pixel_canvas).insert(PixelCanvasSmoothing);
//! }
//!
//! ```

use bevy::prelude::*;
use pixel_camera_schedule::{PixelCameraSchedulePlugin, UpdatePixelCameraSchedule};
use pixel_camera_snapping::update_pixel_camera_pos_snapped;
use pixel_canvas::{
    update_pixel_camera, update_pixel_canvas, update_pixel_canvas_rect, update_pixel_canvas_sprite,
};
use pixel_canvas_smoothing::update_pixel_canvas_rect_smoothed;

pub mod pixel_camera;
pub mod pixel_camera_schedule;
pub mod pixel_camera_snapping;
pub mod pixel_canvas;
pub mod pixel_canvas_smoothing;
pub mod pixel_image;

pub mod prelude;

#[derive(SystemSet, Default, Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct PixelCameraSet;

#[derive(Default, Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct BasicPixelCameraPlugin;

impl Plugin for BasicPixelCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PixelCameraSchedulePlugin)
            .insert_resource(Msaa::Off)
            .add_systems(
                UpdatePixelCameraSchedule,
                (
                    (
                        update_pixel_canvas_sprite,
                        update_pixel_canvas,
                        update_pixel_camera,
                        update_pixel_canvas_rect,
                    ),
                    (
                        update_pixel_canvas_rect_smoothed,
                        update_pixel_camera_pos_snapped,
                    ),
                )
                    .chain()
                    .in_set(PixelCameraSet),
            );
    }
}
