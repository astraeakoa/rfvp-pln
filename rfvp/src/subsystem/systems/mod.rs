
use crate::subsystem::package::Package;

use crate::subsystem::resources::time::{Time, TimerType, Timers};

use crate::subsystem::resources::audio::Audio;
use crate::subsystem::scene::SceneController;
use crate::subsystem::state::GameState;
use crate::subsystem::systems::default_camera_system::camera_dpi_system;
use crate::subsystem::systems::default_camera_system::default_camera_system;
use crate::subsystem::world::GameData;

use crate::app::AppBuilder;

pub(crate) mod default_camera_system;

pub(crate) struct InternalPackage;
impl Package for InternalPackage {
    fn prepare(&self, data: &mut GameData) {

        let mut timers = Timers::default();
        data.insert_resource(Time::default());
        data.insert_resource(timers);
        data.insert_resource(GameState::default());
        data.insert_resource(SceneController::default());
        data.insert_resource(Audio::default());
    }

    fn load(self, builder: AppBuilder) -> AppBuilder {
        builder
            .with_system(default_camera_system)
            .with_system(camera_dpi_system)
    }
}