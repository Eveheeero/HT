pub mod object;
pub mod plugin;

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_tokio_tasks::TokioTasksRuntime;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugin(bevy_tokio_tasks::TokioTasksPlugin {
            make_runtime: Box::new(|| {
                tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .worker_threads(num_cpus::get())
                    .build()
                    .unwrap()
            }),
            ..bevy_tokio_tasks::TokioTasksPlugin::default()
        })
        .add_plugin(EguiPlugin);

    unsafe {
        app.load_plugin("ht_core");
    }

    app.add_system(ui_example_system)
        .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .add_startup_system(add_people)
        .add_system(greet_people)
        .run();
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            trace!("hello {}!", name.0);
            // println!("hello {}!", name.0);
        }
    }
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
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
fn ui_example_system(mut contexts: EguiContexts, runtime: ResMut<TokioTasksRuntime>) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("world");
    });
}
