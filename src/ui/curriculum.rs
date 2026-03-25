//! UI components and logic for the curriculum document viewer.

use bevy::prelude::*;

use crate::components::*;
use crate::resources::CurriculumState;
use crate::constants::PANEL_WIDTH;

/// Marker component for the central curriculum image viewer.
#[derive(Component)]
pub struct CurriculumViewer;

/// System to toggle visibility between the training circle and curriculum viewer.
pub fn toggle_curriculum_visibility(
    curriculum_state: Res<CurriculumState>,
    mut target_query: Query<&mut Visibility, (With<NumberIndex>, Without<CurriculumViewer>)>,
    mut viewer_query: Query<&mut Visibility, (With<CurriculumViewer>, Without<NumberIndex>)>,
) {
    if curriculum_state.is_changed() {
        let (target_vis, viewer_vis) = if curriculum_state.is_visible {
            (Visibility::Hidden, Visibility::Visible)
        } else {
            (Visibility::Visible, Visibility::Hidden)
        };

        for mut visibility in &mut target_query {
            *visibility = target_vis;
        }

        for mut visibility in &mut viewer_query {
            *visibility = viewer_vis;
        }
    }
}

/// System to dynamically load the asset handles for a selected document's pages.
pub fn load_curriculum_pages(
    asset_server: Res<AssetServer>,
    mut curriculum_state: ResMut<CurriculumState>,
) {
    if !curriculum_state.is_changed() {
        return;
    }

    if let (Some(grade), Some(document)) = (&curriculum_state.selected_grade, &curriculum_state.selected_document) {
        // Here we build a predefined list of page numbers since we can't easily query the
        // file system asynchronously within this generic synchronous system without
        // loading states.
        // As a robust approach for this track, we attempt to load up to an arbitrary maximum
        // (e.g., 20 pages) which exceeds known document lengths. The missing files will
        // gracefully fail to load or be skipped when rendering if handles are invalid.
        // For a more exact approach, we need a directory read logic, which Bevy doesn't do natively
        // out of the box in WASM/cross-platform ways easily without plugin extensions, so
        // pre-generating the paths is safer.
        // For our test documents, max page is 13.

        let mut new_pages = Vec::new();
        // Look up the exact page count from the static manifest to avoid runtime
        // file system checks, which ensures WASM and packaged app compatibility.
        let page_count = crate::constants::CURRICULUM_MANIFEST
            .get(grade.as_str())
            .and_then(|docs| docs.iter().find(|(doc, _)| doc == document))
            .map(|(_, count)| *count)
            .unwrap_or(0); // Fallback to 0 if not found, loading nothing

        for i in 0..page_count {
            let asset_path = format!("Grades Escrime/{}/{}/page-{:02}.jpg", grade, document, i);
            let handle: Handle<Image> = asset_server.load(asset_path);
            new_pages.push(handle);
        }

        // Check if the actual handles changed (e.g. if the document changed but happened to have the same page count)
        if curriculum_state.pages != new_pages {
             curriculum_state.pages = new_pages;
             curriculum_state.current_page = 0;
        }
    } else if !curriculum_state.pages.is_empty() {
        curriculum_state.pages.clear();
        curriculum_state.current_page = 0;
    }
}

/// System to update the image component based on the currently selected page.
pub fn update_curriculum_image(
    curriculum_state: Res<CurriculumState>,
    mut image_query: Query<&mut ImageNode, With<CurriculumDocumentImage>>,
) {
    if !curriculum_state.is_changed() || !curriculum_state.is_visible {
        return;
    }

    if let Some(handle) = curriculum_state.pages.get(curriculum_state.current_page) {
        for mut image_node in &mut image_query {
            image_node.image = handle.clone();
        }
    }
}

/// System to handle page navigation via keyboard inputs (Left/Right arrows).
pub fn curriculum_keyboard_navigation(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut curriculum_state: ResMut<CurriculumState>,
) {
    if !curriculum_state.is_visible || curriculum_state.pages.is_empty() {
        return;
    }

    if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
        if curriculum_state.current_page > 0 {
            curriculum_state.current_page -= 1;
        }
    } else if keyboard_input.just_pressed(KeyCode::ArrowRight) {
        if curriculum_state.current_page < curriculum_state.pages.len().saturating_sub(1) {
            curriculum_state.current_page += 1;
        }
    }
}

/// Spawns the central curriculum document viewer node.
pub fn spawn_curriculum_viewer(parent: &mut ChildSpawnerCommands) {
    parent.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            top: Val::Px(0.0),
            width: Val::Vw(100.0),
            height: Val::Vh(100.0),
            // Ensure the image fits within the area excluding the settings panel
            padding: UiRect::right(Val::Px(PANEL_WIDTH)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        Visibility::Hidden,
        CurriculumViewer,
    )).with_children(|parent| {
        parent.spawn((
            ImageNode {
                // Initial placeholder or clear image; dynamic loading updates this
                ..default()
            },
            Node {
                max_width: Val::Percent(100.0),
                max_height: Val::Percent(100.0),
                // Preserve aspect ratio
                ..default()
            },
            crate::components::CurriculumDocumentImage,
        ));
    });
}
