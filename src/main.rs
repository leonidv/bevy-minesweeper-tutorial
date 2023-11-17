use bevy::{prelude::*, window::WindowResolution};

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    let mut app = App::new();

    let mut primary_window = Window::default();
    primary_window.resolution = WindowResolution::new(700.0, 800.0);
    primary_window.title = "Mine Sweeper!".to_string();
    // adapted from 0.8
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(primary_window),
        exit_condition: bevy::window::ExitCondition::OnPrimaryClosed,
        close_when_requested: true
    }));

    #[cfg(feature = "debug")]
    app.add_plugins(WorldInspectorPlugin::new());

    // adapted from 0.8, 0.10 to 0.11
    app.add_systems(Startup, camera_setup);
    app.run();
}

fn camera_setup(mut commands: Commands) {    
    // adopted 0.7 to 0.8
    commands.spawn(Camera2dBundle::default());
}