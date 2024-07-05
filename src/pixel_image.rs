use bevy::{
    prelude::*,
    render::{
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        texture::{ImageSampler, ImageSamplerDescriptor},
    },
};

pub fn create_pixel_image(mut images: ResMut<Assets<Image>>) -> Handle<Image> {
    let image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size: Extent3d::default(),
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        sampler: ImageSampler::Descriptor(ImageSamplerDescriptor::nearest()),
        ..default()
    };

    images.add(image)
}
