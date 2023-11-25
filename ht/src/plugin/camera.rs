use crate::prelude::*;

pub(crate) struct CameraPlugin;
#[derive(Component)]
pub(crate) struct CameraComponent;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, add_object);
    }
}

fn add_object(mut commands: Commands) {
    commands.spawn((CameraComponent, Camera2dBundle::default()));
}
