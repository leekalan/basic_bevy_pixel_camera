use bevy::prelude::*;

use crate::{pixel_camera::PixelCamera, pixel_canvas::PixelCanvas};

#[derive(Component, Default, Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct PixelCameraSnapping;

#[allow(clippy::type_complexity)]
pub fn update_pixel_camera_pos_snapped(
    mut transforms: Query<&mut Transform>,
    pixel_camera: Query<
        (&Parent, Entity),
        (
            With<PixelCamera>,
            With<PixelCameraSnapping>,
            With<Transform>,
        ),
    >,
    pixel_canvas: Query<&PixelCanvas>,
) {
    for canvas in &pixel_canvas {
        let Ok((parent, id)) = pixel_camera.get(canvas.camrera_id) else {
            continue;
        };
        let Ok(parent_transform) = transforms.get(parent.get()) else {
            continue;
        };

        let pixel_translation = parent_transform.translation.truncate() * canvas.pixels_per_unit;
        let offset_scaled = pixel_translation - pixel_translation.round();
        let offset = offset_scaled / canvas.pixels_per_unit;

        let mut transform = transforms.get_mut(id).unwrap(); // as With<Transform> was used

        transform.translation.x = -offset.x;
        transform.translation.y = -offset.y;
    }
}
