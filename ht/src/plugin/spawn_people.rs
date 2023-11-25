use crate::prelude::*;

pub(crate) struct SpawnPeople;

impl Plugin for SpawnPeople {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, add_object);
    }
}

fn add_object(mut commands: Commands, mut world: ResMut<HtWorld>) {}
