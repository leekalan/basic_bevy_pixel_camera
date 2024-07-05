use bevy::prelude::*;

use crate::pixel_canvas::{generate_pixel_canvas_rect, PixelCanvas};

#[derive(Component, Default, Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct PixelCanvasSmoothing;

pub fn update_pixel_canvas_rect_smoothed(
    transforms: Query<&Transform>,
    mut query: Query<(&mut Sprite, &PixelCanvas, &Parent), With<PixelCanvasSmoothing>>,
) {
    for (mut sprite, pixel_canvas, parent) in &mut query {
        let Ok(transform) = transforms.get(parent.get()) else {
            continue;
        };

        let pixel_translation = transform.translation.truncate() * pixel_canvas.pixels_per_unit;
        let offset = pixel_translation - pixel_translation.round();

        sprite.rect = Some(generate_pixel_canvas_rect(
            pixel_canvas.pixels_per_unit,
            pixel_canvas.unit_width,
            pixel_canvas.unit_height,
            offset.x,
            -offset.y,
        ))
    }
}
