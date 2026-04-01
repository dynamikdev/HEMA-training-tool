//! Marker components used to identify and query entities within the HEMA Training Tool.
//!
//! These components are primarily used by Bevy's ECS to filter entities in systems
//! and to associate data or behavior with specific UI elements.

use bevy::prelude::*;

/// Associated with target number entities in the circular layout.
///
/// Stores the zero-based index (0-7) of the target. This index is used to map
/// the entity to its corresponding value in [`crate::constants::LABELS`] and to
/// determine its position on the circular UI.
#[derive(Component)]
pub struct NumberIndex(pub u8);

/// Marker for the UI button that toggles the training sequence's execution state.
///
/// Systems query for this component to handle click events that start or stop
/// the highlighting timer.
#[derive(Component)]
pub struct SequenceControlButton;

/// Marker for the UI button that switches the sequence generation strategy.
///
/// Used to identify the button that toggles between [`crate::resources::SequenceMode::Random`]
/// and [`crate::resources::SequenceMode::Ordered`].
#[derive(Component)]
pub struct SequenceModeButton;

/// Marker for the UI button that toggles the rhythm acceleration behavior.
///
/// Used to identify the button that switches between [`crate::resources::RhythmMode::Constant`]
/// and [`crate::resources::RhythmMode::Accelerate`].
#[derive(Component)]
pub struct RhythmModeButton;

/// Marker for the text element that displays the current rhythm duration.
///
/// Systems use this marker to find and update the text whenever the rhythm
/// duration changes (e.g., via the slider or during acceleration).
#[derive(Component)]
pub struct RhythmText;

/// Marker for the UI button that toggles the visibility of the curriculum documents.
#[derive(Component)]
pub struct CurriculumToggleButton;

/// Marker for the UI button that selects the curriculum grade.
#[derive(Component)]
pub struct CurriculumGradeButton;

/// Marker for the UI button that selects the curriculum document.
#[derive(Component)]
pub struct CurriculumDocumentButton;

/// Marker for the UI button that navigates to the previous page.
#[derive(Component)]
pub struct CurriculumPrevPageButton;

/// Marker for the UI button that navigates to the next page.
#[derive(Component)]
pub struct CurriculumNextPageButton;

/// Marker for the text element that displays the current curriculum page number.
#[derive(Component)]
pub struct CurriculumPageText;

/// Marker for the curriculum document image display node.
#[derive(Component)]
pub struct CurriculumDocumentImage;
