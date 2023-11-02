use bevy::prelude::*;

#[cfg(not(feature = "reload"))]
use systems::*;
#[cfg(feature = "reload")]
use systems_hot::*;

#[cfg(feature = "reload")]
#[hot_lib_reloader::hot_module(dylib = "systems")]
mod systems_hot {
    use bevy::prelude::*;
    pub use components::*;
    hot_functions_from_file!("systems/src/lib.rs");
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_systems(Startup, systems::setup)
        .add_systems(Update, (bevy::window::close_on_esc, reloadable));
    app.run();
}
