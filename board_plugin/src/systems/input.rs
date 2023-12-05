use crate::Board;
use crate::events::TileTriggerEvent;

use bevy::input::ButtonState;
use bevy::input::{mouse::MouseButtonInput};
use bevy::log;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;


// adopted 0.9 to 0.10 https://bevyengine.org/learn/migration-guides/0.9-0.10/#windows-as-entities
pub fn input_handling(
    window_primary_query: Query<&Window, With<PrimaryWindow>>,
    board: Res<Board>,
    mut button_evr: EventReader<MouseButtonInput>,
    mut tile_trigger_ewr: EventWriter<TileTriggerEvent>
) {
    // pattern from bevy migration guide
    let Ok(window) = window_primary_query.get_single() else { return };

    

    for event in button_evr.read() {        
        // adopted 0.7 to 0.8 https://bevyengine.org/learn/migration-guides/0.7-0.8/#rename-elementstate-to-buttonstate
        if let ButtonState::Pressed = event.state {         
            if let Some(click_position) = window.cursor_position() {

                if let Some(tile_coordinates) = board.mouse_position(window, click_position) {
                    match event.button {
                        MouseButton::Left => {
                            log::info!("Trying uncover tile on {}", tile_coordinates);
                            tile_trigger_ewr.send(TileTriggerEvent{
                                coordinates: tile_coordinates
                            });
                            
                        },
                        MouseButton::Right => {
                            log::info!("Trying mark tile on {}", tile_coordinates);
                        },
                        _ => (),
                    }
                }

            };
            
        }
    }

    
}