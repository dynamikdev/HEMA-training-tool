//! UI components for the settings sidebar in the HEMA Training Tool.
//!
//! This module provides functions to spawn the configuration panel, including
//! controls for sequence modes, rhythm settings, and playback state.

use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;
use bevy_ui_widgets::{
    observe, slider_self_update, Slider, SliderRange, SliderStep, SliderThumb, SliderValue,
    TrackClick,
};

use crate::components::*;
use crate::constants::{PANEL_WIDTH, SURFACE_LOW, NEUTRAL_TEXT, GHOST_BORDER};
use crate::resources::Typography;

/// Spawns the settings configuration panel.
///
/// The panel is positioned on the right side of the screen and contains
/// all the controls for the training tool.
pub fn spawn_settings_panel(parent: &mut ChildSpawnerCommands, typography: &Typography) {
    parent
        .spawn((
            Node {
                width: Val::Px(PANEL_WIDTH),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            BackgroundColor(SURFACE_LOW),
        ))
        .with_children(|parent| {
            // Panel Title
            parent.spawn((
                Text::new("SETTINGS"),
                TextFont {
                    font: typography.space_grotesk.clone(),
                    font_size: 40.0,
                    ..default()
                },
                TextColor(NEUTRAL_TEXT),
                Node {
                    margin: UiRect::bottom(Val::Px(30.0)),
                    ..default()
                },
            ));

            spawn_mode_toggle_button(parent, typography);
            spawn_sequence_control_button(parent, typography);
            spawn_rhythm_section(parent, typography);
            spawn_curriculum_section(parent, typography);
        });
}

/// Spawns the curriculum integration controls, including the visibility toggle
/// and selectors for grades/documents.
fn spawn_curriculum_section(parent: &mut ChildSpawnerCommands, typography: &Typography) {
    parent.spawn((
        Node {
            margin: UiRect::top(Val::Px(30.0)),
            flex_direction: FlexDirection::Column,
            ..default()
        },
    )).with_children(|parent| {
        parent.spawn((
            Text::new("CURRICULUM"),
            TextFont {
                font: typography.space_grotesk.clone(),
                font_size: 24.0,
                ..default()
            },
            TextColor(NEUTRAL_TEXT),
            Node {
                margin: UiRect::bottom(Val::Px(15.0)),
                ..default()
            },
        ));

        // Toggle Curriculum Button
        parent.spawn((
            Button,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(45.0),
                border: UiRect::all(Val::Px(1.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::bottom(Val::Px(15.0)),
                ..default()
            },
            BorderColor::all(GHOST_BORDER),
            BackgroundColor(Color::NONE),
            crate::components::CurriculumToggleButton,
        )).with_children(|parent| {
            parent.spawn((
                Text::new("Open Curriculum"),
                TextFont {
                    font: typography.work_sans.clone(),
                    font_size: 16.0,
                    ..default()
                },
                TextColor(NEUTRAL_TEXT),
            ));
        });

        // Grade Selection Button
        parent.spawn((
            Button,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(45.0),
                border: UiRect::all(Val::Px(1.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::bottom(Val::Px(15.0)),
                ..default()
            },
            BorderColor::all(GHOST_BORDER),
            BackgroundColor(Color::NONE),
            crate::components::CurriculumGradeButton,
        )).with_children(|parent| {
            parent.spawn((
                Text::new("Grade: Niveau 1.1"),
                TextFont {
                    font: typography.work_sans.clone(),
                    font_size: 14.0,
                    ..default()
                },
                TextColor(NEUTRAL_TEXT),
                crate::ui::systems::ParentButton::<crate::components::CurriculumGradeButton>(std::marker::PhantomData),
            ));
        });

        // Document Selection Button
        parent.spawn((
            Button,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(45.0),
                border: UiRect::all(Val::Px(1.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::bottom(Val::Px(15.0)),
                ..default()
            },
            BorderColor::all(GHOST_BORDER),
            BackgroundColor(Color::NONE),
            crate::components::CurriculumDocumentButton,
        )).with_children(|parent| {
            parent.spawn((
                Text::new("Doc: Passage de Grade 1.1"),
                TextFont {
                    font: typography.work_sans.clone(),
                    font_size: 12.0,
                    ..default()
                },
                TextColor(NEUTRAL_TEXT),
                crate::ui::systems::ParentButton::<crate::components::CurriculumDocumentButton>(std::marker::PhantomData),
            ));
        });

        // Page Navigation Row
        parent.spawn(Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            margin: UiRect::bottom(Val::Px(15.0)),
            ..default()
        }).with_children(|parent| {
            // Previous Button
            parent.spawn((
                Button,
                Node {
                    width: Val::Px(45.0),
                    height: Val::Px(45.0),
                    border: UiRect::all(Val::Px(1.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor::all(GHOST_BORDER),
                BackgroundColor(Color::NONE),
                crate::components::CurriculumPrevPageButton,
            )).with_children(|parent| {
                parent.spawn((
                    Text::new("<"),
                    TextFont {
                        font: typography.work_sans.clone(),
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(NEUTRAL_TEXT),
                ));
            });

            // Page Counter
            parent.spawn((
                Text::new("Page: -- / --"),
                TextFont {
                    font: typography.work_sans.clone(),
                    font_size: 14.0,
                    ..default()
                },
                TextColor(NEUTRAL_TEXT),
                crate::components::CurriculumPageText,
            ));

            // Next Button
            parent.spawn((
                Button,
                Node {
                    width: Val::Px(45.0),
                    height: Val::Px(45.0),
                    border: UiRect::all(Val::Px(1.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor::all(GHOST_BORDER),
                BackgroundColor(Color::NONE),
                crate::components::CurriculumNextPageButton,
            )).with_children(|parent| {
                parent.spawn((
                    Text::new(">"),
                    TextFont {
                        font: typography.work_sans.clone(),
                        font_size: 20.0,
                        ..default()
                    },
                    TextColor(NEUTRAL_TEXT),
                ));
            });
        });
    });
}

/// Spawns the button for toggling sequence mode (Random vs Ordered).
fn spawn_mode_toggle_button(parent: &mut ChildSpawnerCommands, typography: &Typography) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(65.0),
                border: UiRect::all(Val::Px(1.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::bottom(Val::Px(15.0)),
                ..default()
            },
            BorderColor::all(GHOST_BORDER),
            BackgroundColor(Color::NONE),
            SequenceModeButton,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Mode: Random"),
                TextFont {
                    font: typography.work_sans.clone(),
                    font_size: 20.0,
                    ..default()
                },
                TextColor(NEUTRAL_TEXT),
            ));
        });
}

/// Spawns the button for starting/stopping the training sequence.
fn spawn_sequence_control_button(parent: &mut ChildSpawnerCommands, typography: &Typography) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(65.0),
                border: UiRect::all(Val::Px(1.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::bottom(Val::Px(15.0)),
                ..default()
            },
            BorderColor::all(GHOST_BORDER),
            BackgroundColor(Color::NONE),
            SequenceControlButton,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Launch Sequence"),
                TextFont {
                    font: typography.work_sans.clone(),
                    font_size: 20.0,
                    ..default()
                },
                TextColor(NEUTRAL_TEXT),
            ));
        });
}

/// Spawns the rhythm controls, including the mode toggle, duration label, and speed slider.
///
/// This section allows the user to switch between constant and accelerating rhythms
/// and adjust the base timing of the sequence.
fn spawn_rhythm_section(parent: &mut ChildSpawnerCommands, typography: &Typography) {
    // Rhythm Mode Toggle Button: Constant vs Accelerate timing.
    parent
        .spawn((
            Button,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(45.0),
                border: UiRect::all(Val::Px(1.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::bottom(Val::Px(15.0)),
                ..default()
            },
            BorderColor::all(GHOST_BORDER),
            BackgroundColor(Color::NONE),
            RhythmModeButton,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Rhythm: Constant"),
                TextFont {
                    font: typography.work_sans.clone(),
                    font_size: 16.0,
                    ..default()
                },
                TextColor(NEUTRAL_TEXT),
            ));
        });

    // Rhythm current value label.
    parent.spawn((
        Text::new("Rhythm: 1.0s"),
        TextFont {
            font: typography.work_sans.clone(),
            font_size: 20.0,
            ..default()
        },
        TextColor(NEUTRAL_TEXT),
        Node {
            margin: UiRect::bottom(Val::Px(5.0)),
            ..default()
        },
        RhythmText,
    ));

    // Rhythm Slider widget (vertical).
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(24.0),
                height: Val::Px(200.0),
                align_self: AlignSelf::Center,
                margin: UiRect::bottom(Val::Px(20.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
            BorderColor::all(GHOST_BORDER),
            Interaction::default(),
            RelativeCursorPosition::default(),
            Slider {
                track_click: TrackClick::Snap,
            },
            SliderValue(1.0),
            SliderRange::new(0.5, 3.0),
            SliderStep(0.1),
            observe(slider_self_update),
        ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(16.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                BackgroundColor(NEUTRAL_TEXT),
                Interaction::default(),
                SliderThumb,
            ));
        });
}
