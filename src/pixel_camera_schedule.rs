use bevy::{app::MainScheduleOrder, ecs::schedule::ScheduleLabel, prelude::*};

#[derive(ScheduleLabel, Default, Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct UpdatePixelCameraSchedule;

#[derive(Default, Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct PixelCameraSchedulePlugin;

impl Plugin for PixelCameraSchedulePlugin {
    fn build(&self, app: &mut App) {
        app.init_schedule(UpdatePixelCameraSchedule);
        app.world_mut()
            .resource_mut::<MainScheduleOrder>()
            .insert_after(Update, UpdatePixelCameraSchedule);
    }
}
