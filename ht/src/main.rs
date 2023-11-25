mod plugin;
mod prelude;

use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugin(bevy_tokio_tasks::TokioTasksPlugin {
            make_runtime: Box::new(|| {
                ht_core::tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .worker_threads(ht_core::num_cpus::get())
                    .build()
                    .unwrap()
            }),
            ..bevy_tokio_tasks::TokioTasksPlugin::default()
        });

    app.add_startup_system(add_people).run();
}

fn add_people(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(ColorMesh2dBundle {
        mesh: meshes
            .add(Mesh::from(shape::Quad {
                size: Vec2::new(100.0, 100.0),
                ..Default::default()
            }))
            .into(),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        material: materials.add(ColorMaterial::from(Color::GREEN)),
        ..Default::default()
    });
}
