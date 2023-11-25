mod camera;
mod spawn_people;
mod tokio;

pub(crate) use camera::{CameraComponent, CameraPlugin};
pub(crate) use spawn_people::SpawnPeople;
pub(crate) use tokio::{TokioPlugin, TokioRuntime};
