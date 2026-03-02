//! Resources used by the HEMA Training Tool to manage simulation state.

use bevy::prelude::*;

/// Timer that controls when the highlighted number should change.
#[derive(Resource)]
pub struct HighlightTimer(pub Timer);

/// Stores the index (0-7) of the currently highlighted target number.
#[derive(Resource)]
pub struct CurrentNumber(pub u8);

/// Defines how the sequence of target numbers is generated.
#[derive(Default, PartialEq, Clone, Copy)]
pub enum SequenceMode {
    /// Numbers are picked randomly.
    #[default]
    Random,
    /// Numbers follow the numeric order (1, 2, 3...).
    Ordered,
}

/// State for the overall training sequence.
#[derive(Resource, Default)]
pub struct SequenceState {
    /// Whether the sequence is currently active.
    pub running: bool,
    /// The current sequence generation mode.
    pub mode: SequenceMode,
    /// The last value used in Ordered mode (to cycle 1..=8).
    pub current_ordered_value: u8, // 1 to 7
}

/// Defines the rhythm (timing) of the sequence.
#[derive(Default, PartialEq, Clone, Copy, Debug)]
pub enum RhythmMode {
    /// Constant interval between steps.
    #[default]
    Constant,
    /// The interval decreases every 8 steps.
    Accelerate,
}

/// State for managing the timing rhythm of the sequence.
#[derive(Resource)]
pub struct RhythmState {
    /// Current duration in seconds between highlights.
    pub duration: f32, // slider value between 0.5 and 3.0
    /// The current rhythm mode.
    pub mode: RhythmMode,
    /// Progress counter used in Accelerate mode to track when to speed up.
    pub accelerate_counter: u8,
}
