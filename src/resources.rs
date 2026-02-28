use bevy::prelude::*;

#[derive(Resource)]
pub struct HighlightTimer(pub Timer);

#[derive(Resource)]
pub struct CurrentNumber(pub u8);

#[derive(Default, PartialEq, Clone, Copy)]
pub enum SequenceMode {
    #[default]
    Random,
    Ordered,
}

#[derive(Resource, Default)]
pub struct SequenceState {
    pub running: bool,
    pub mode: SequenceMode,
    pub current_ordered_value: u8, // 1 to 7
}

#[derive(Default, PartialEq, Clone, Copy, Debug)]
pub enum RhythmMode {
    #[default]
    Constant,
    Accelerate,
}

#[derive(Resource)]
pub struct RhythmState {
    pub duration: f32, // slider value between 0.5 and 3.0
    pub mode: RhythmMode,
    pub accelerate_counter: u8,
}
