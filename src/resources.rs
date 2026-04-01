//! Simulation resources that manage the global state of the HEMA training tool.
//!
//! These resources store data that exists globally and is shared across systems
//! in the ECS. They define the current state of the training session, including
//! timers, sequence modes, and rhythm settings.

use bevy::prelude::*;

/// Global timer controlling the interval between target number highlights.
///
/// This timer is ticked by the training logic systems and triggers the
/// transition to the next target in the sequence.
#[derive(Resource)]
pub struct HighlightTimer(pub Timer);

/// Stores the index (0-7) of the currently highlighted target number.
///
/// Systems use this value to update the visual appearance of target entities
/// by matching this index against their [`crate::components::NumberIndex`].
#[derive(Resource)]
pub struct CurrentNumber(pub u8);

/// Strategy for generating the sequence of target numbers.
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum SequenceMode {
    /// Numbers are picked randomly using a random number generator.
    /// This mode ensures that the same number is not picked twice in a row.
    #[default]
    Random,
    /// Numbers follow the sequential numeric order (1, 2, 3, 4, 5, 6, 7, 8).
    /// This is used to practice standard cutting sequences or drills.
    Ordered,
}

/// Global state management for the training sequence.
///
/// This resource tracks the active status and the internal progression of the
/// training session.
#[derive(Resource, Default, Debug)]
pub struct SequenceState {
    /// Whether the training sequence is currently running and the timer is ticking.
    pub running: bool,
    /// The current strategy used to determine the next target in the sequence.
    pub mode: SequenceMode,
    /// Internal counter used in [`SequenceMode::Ordered`] to track the last
    /// numeric value (1-8) presented to the user.
    pub current_ordered_value: u8,
}

/// Mode for managing the timing rhythm of the sequence.
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum RhythmMode {
    /// Highlights happen at a fixed, constant interval.
    #[default]
    Constant,
    /// The interval duration decreases every 8 steps to simulate increasing speed.
    /// This provides progressive overload in training.
    Accelerate,
}

/// Parameters for controlling the speed and variability of the training rhythm.
///
/// This resource dictates how often the highlighted number changes and whether
/// that frequency evolves over time.
#[derive(Resource, Debug)]
pub struct RhythmState {
    /// The current time interval (in seconds) between highlights.
    /// Typical values range from 0.5 to 3.0 seconds.
    pub duration: f32,
    /// The current rhythm strategy.
    pub mode: RhythmMode,
    /// Step counter used in [`RhythmMode::Accelerate`] to track when the
    /// rhythm should speed up. Reset to zero after each speed increase.
    pub accelerate_counter: u8,
}

/// Stores the index (0-7) of the target diametrically opposite to the current one.
///
/// This resource is updated whenever [`CurrentNumber`] changes and is used
/// to determine the endpoint of the visual guide arrow.
#[derive(Resource, Default, Debug)]
pub struct ArrowTarget(pub Option<u8>);

/// Tracks the progress of the arrow's "shoot" animation.
///
/// The progress value ranges from 0.0 (arrow just starting at the origin)
/// to 1.0 (arrow fully extended to the destination).
#[derive(Resource, Default, Debug)]
pub struct ArrowAnimationState {
    /// The current interpolation factor (0.0 to 1.0).
    pub progress: f32,
}

/// Stores font handles for the Kinetic Brutalism design system.
///
/// This resource ensures that the specialized typography (Space Grotesk for
/// display/headers and Work Sans for labels) is accessible to all UI systems.
#[derive(Resource, Debug, Default)]
pub struct Typography {
    /// Geometric display font for target numbers and display elements.
    pub space_grotesk: Handle<Font>,
    /// Mechanical, legible font for technical labels and UI controls.
    pub work_sans: Handle<Font>,
}

/// Global state management for the Curriculum Integration.
///
/// This resource tracks the visibility of the curriculum documents, the
/// currently selected grade/document, the current page number, and the
/// loaded page image handles.
#[derive(Resource, Debug)]
pub struct CurriculumState {
    /// Whether the curriculum document view is visible.
    pub is_visible: bool,
    /// The currently selected grade level (e.g., "Niveau 1.1").
    pub selected_grade: Option<String>,
    /// The currently selected document within the grade.
    pub selected_document: Option<String>,
    /// The current page index being displayed (0-based).
    pub current_page: usize,
    /// The asset handles for the images (pages) of the currently selected document.
    pub pages: Vec<Handle<Image>>,
}

impl Default for CurriculumState {
    fn default() -> Self {
        Self {
            is_visible: false,
            selected_grade: Some("Niveau 1.1".to_string()),
            selected_document: Some("Passage de Grade 1.1".to_string()),
            current_page: 1,
            pages: Vec::new(),
        }
    }
}
