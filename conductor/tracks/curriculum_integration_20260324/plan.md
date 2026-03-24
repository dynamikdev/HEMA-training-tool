# Implementation Plan: Curriculum Integration

## Phase 1: Data Structures and State
- [ ] Task: Define `CurriculumState` resource in `src/resources.rs` to track visibility, selected grade/document, and current page.
- [ ] Task: Create a static `CurriculumManifest` in `src/constants.rs` mapping the `assets/Grades Escrime` folder structure.
- [ ] Task: Implement TDD for `CurriculumState` initialization and basic state transitions in `src/resources_test.rs`.
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Data Structures and State' (Protocol in workflow.md)

## Phase 2: Sidebar UI Integration
- [ ] Task: Add a "Curriculum" tab/button to the settings sidebar in `src/ui/settings.rs`.
- [ ] Task: Implement logic to toggle `CurriculumState.is_visible` and pause training when opened.
- [ ] Task: Add dropdowns/buttons for selecting Grade and Document within the sidebar.
- [ ] Task: Write tests for sidebar interaction and state synchronization in `src/ui/setup_test.rs` or a new `src/ui/settings_test.rs`.
- [ ] Task: Conductor - User Manual Verification 'Phase 2: Sidebar UI Integration' (Protocol in workflow.md)

## Phase 3: Main View Replacement & Navigation
- [ ] Task: Create `src/ui/curriculum.rs` to handle the document viewer UI logic.
- [ ] Task: Implement a system to swap the Training Circle with the Document Image when curriculum is active.
- [ ] Task: Add "Previous" and "Next" page navigation buttons to the sidebar or a bottom overlay.
- [ ] Task: Map Left/Right arrow keys to page navigation in `src/logic.rs` or `src/ui/systems.rs`.
- [ ] Task: Write tests for page navigation logic and image loading in a new `src/ui/curriculum_test.rs`.
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Main View Replacement & Navigation' (Protocol in workflow.md)

## Phase 4: Dynamic Asset Loading
- [ ] Task: Implement a system to dynamically load all `page-XX.jpg` handles for a selected document into `CurriculumState.pages`.
- [ ] Task: Ensure smooth transitions between pages with proper asset handle management.
- [ ] Task: Verify that switching back to "Training" correctly restores the training circle and sequence logic.
- [ ] Task: Conductor - User Manual Verification 'Phase 4: Dynamic Asset Loading' (Protocol in workflow.md)
