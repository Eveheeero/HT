mod plugin;
mod prelude;

use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins((
        plugin::TokioPlugin,
        plugin::CameraPlugin,
        plugin::SpawnPeople,
    ));

    app.insert_resource(ht_core::World::default());

    app.add_systems(Startup, add_quad_sample).run();
}

fn add_quad_sample(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
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
