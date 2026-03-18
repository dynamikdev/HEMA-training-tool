use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;
use bevy_ui_widgets::{
    observe, slider_self_update, Slider, SliderRange, SliderStep, SliderThumb, SliderValue,
    TrackClick,
};

use crate::components::*;
use crate::constants::PANEL_WIDTH;

/// Spawns the settings configuration panel.
pub fn spawn_settings_panel(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            Node {
                width: Val::Px(PANEL_WIDTH),
                height: Val::Percent(100.0),
                border: UiRect::left(Val::Px(2.0)),
                flex_direction: FlexDirection::Column,
                padding: UiRect::all(Val::Px(10.0)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
            BorderColor::all(Color::WHITE),
        ))
        .with_children(|parent| {
            // Panel Title
            parent.spawn((
                Text::new("Settings"),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::bottom(Val::Px(20.0)),
                    ..default()
                },
            ));

            spawn_mode_toggle_button(parent);
            spawn_sequence_control_button(parent);
            spawn_rhythm_section(parent);
        });
}

/// Spawns the button for toggling sequence mode.
fn spawn_mode_toggle_button(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(65.0),
                border: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::bottom(Val::Px(10.0)),
                ..default()
            },
            BorderColor::all(Color::WHITE),
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            SequenceModeButton,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Mode: Random"),
                TextFont {
                    font_size: 25.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));
        });
}

/// Spawns the button for starting/stopping the sequence.
fn spawn_sequence_control_button(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(65.0),
                border: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::bottom(Val::Px(10.0)),
                ..default()
            },
            BorderColor::all(Color::WHITE),
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            SequenceControlButton,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Launch Sequence"),
                TextFont {
                    font_size: 30.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));
        });
}

/// Spawns the rhythm controls (mode toggle, label, and slider).
fn spawn_rhythm_section(parent: &mut ChildSpawnerCommands) {
    // Rhythm Mode Toggle Button: Constant vs Accelerate timing.
    parent
        .spawn((
            Button,
            Node {
                width: Val::Percent(100.0),
                height: Val::Px(45.0),
                border: UiRect::all(Val::Px(2.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                margin: UiRect::bottom(Val::Px(10.0)),
                ..default()
            },
            BorderColor::all(Color::WHITE),
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            RhythmModeButton,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Rhythm: Constant"),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));
        });

    // Rhythm current value label.
    parent.spawn((
        Text::new("Rhythm: 1.0s"),
        TextFont {
            font_size: 25.0,
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.9, 0.9)),
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
            BorderColor::all(Color::WHITE),
            Interaction::default(),
            RelativeCursorPosition::default(),
            Slider {
                track_click: TrackClick::Snap,
                ..default()
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
                BackgroundColor(Color::srgb(0.8, 0.8, 0.8)),
                Interaction::default(),
                SliderThumb,
            ));
        });
}
