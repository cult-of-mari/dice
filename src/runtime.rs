use bevy::prelude::*;
use tokio::runtime::{Builder, Runtime};

pub struct TokioPlugin;

#[derive(Debug, Deref, DerefMut, Resource)]
pub struct TokioRuntime(Runtime);

impl Plugin for TokioPlugin {
    fn build(&self, app: &mut App) {
        let result = Builder::new_multi_thread()
            .enable_all()
            .build()
            .map(TokioRuntime);

        let Ok(resource) = result else {
            error!("failed to build tokio runtime: {}", result.unwrap_err());

            return;
        };

        app.insert_resource(resource);
    }
}
