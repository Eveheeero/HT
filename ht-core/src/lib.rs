use bevy::prelude::*;

#[derive(DynamicPlugin)]
struct SamplePlugin;

impl Plugin for SamplePlugin {
    fn build(&self, app: &mut App) {
        println!("Hello!");
    }
}
