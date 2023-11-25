use crate::prelude::*;

pub(crate) struct TokioPlugin;
#[derive(Resource)]
pub(crate) struct TokioRuntime(pub(crate) tokio::runtime::Runtime);

impl Plugin for TokioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, add_object);
    }
}

fn add_object(mut commands: Commands) {
    commands.insert_resource(TokioRuntime(
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .worker_threads(num_cpus::get())
            .build()
            .unwrap(),
    ));
}
