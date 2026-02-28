mod components;
mod constants;
mod logic;
mod resources;
mod ui;

use bevy::prelude::*;
use bevy_ui_widgets::slider_self_update;

use logic::TrainingPlugin;
use resources::*;
use ui::UiPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(HighlightTimer(Timer::from_seconds(
            1.0,
            TimerMode::Repeating,
        )))
        .insert_resource(CurrentNumber(1))
        .insert_resource(SequenceState {
            running: false,
            mode: SequenceMode::Random,
            current_ordered_value: 0,
        })
        .insert_resource(RhythmState {
            duration: 1.0,
            mode: RhythmMode::Constant,
            accelerate_counter: 0,
        })
        .add_plugins(TrainingPlugin)
        .add_plugins(UiPlugin)
        // Self-update the SliderValue component upon dragging
        .add_observer(slider_self_update)
        .run();
}
