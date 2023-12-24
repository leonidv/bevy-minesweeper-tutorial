use bevy::log::LogPlugin;
use bevy::log;
use bevy::{prelude::*, window::WindowResolution};

use bevy_inspector_egui::quick::ResourceInspectorPlugin;
#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use board_plugin::components::Coordinates;
use board_plugin::resources::{BoardOptions, BoardSize};
use board_plugin::resources::TileSize::Fixed;
use board_plugin::BoardPlugin;

#[cfg_attr(feature = "debug", derive(Reflect))]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum AppState {
    NewGame,
    #[default] InGame,
    Pause,
    EndGame
}
fn main() {
    let mut app = App::new();
    let mut primary_window = Window::default();
    primary_window.resolution = WindowResolution::new(850., 850.0);
    primary_window.title = "Mine Sweeper!".to_string();
    // adapted from 0.8
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(primary_window),
                exit_condition: bevy::window::ExitCondition::OnPrimaryClosed,
                close_when_requested: true,
            })
            .set(LogPlugin {
                //level: bevy::log::Level::DEBUG,
                ..Default::default()
            }),
    );

    app.insert_resource(BoardOptions {
        map_size: BoardSize { columns: 20, rows: 20 },
        bomb_count: 60,
        position: board_plugin::resources::BoardPosition::Centered { offset: Vec3::ZERO },            
        tile_padding: 3.0,
        // different from tutorial due to WindowDescriptor is not available as a resource
        tile_size: Fixed(35.0),
        safe_start: true,
        game_state: AppState::InGame,
        pause_state: AppState::Pause,
    });

    app.add_state::<AppState>();
    app.add_plugins(BoardPlugin{
        game_state: AppState::InGame,
        pause_state: AppState::Pause,
    });

    // adapted from 0.8, 0.10 to 0.11
    app.add_systems(Startup, camera_setup);

    #[cfg(feature = "debug")]
    {
        app.add_plugins(WorldInspectorPlugin::new());
        app.register_type::<Coordinates>();
        app.register_type::<board_plugin::components::bomb::Bomb>();
        app.register_type::<board_plugin::components::uncover::Uncover>();
        app.register_type::<board_plugin::resources::board::Board>();
        app.add_plugins(ResourceInspectorPlugin::<
            board_plugin::resources::board::Board,
        >::default());
    }
    
    app.run();
}

fn camera_setup(mut commands: Commands) {
    // adopted 0.7 to 0.8
    commands.spawn(Camera2dBundle::default());
}
