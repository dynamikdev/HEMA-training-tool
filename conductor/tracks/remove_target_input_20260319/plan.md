# Implementation Plan: Remove Target Keystroke Interaction

## Phase 1: Remove Target Input & Ensure Auto-Timer
- [x] Task: Update `src/logic_test.rs` to remove tests related to `handle_target_input` and keyboard keys 1-8. Ensure tests for `update_sequence_logic` verify auto-transitioning without input. 864dc5d
- [ ] Task: Update `src/logic.rs` to remove the `handle_target_input` system, `TargetInputEvent`, and modify `update_sequence_logic` to smoothly auto-transition targets.
- [ ] Task: Update `src/main.rs` to remove the `handle_target_input` system and `TargetInputEvent` registration.
- [ ] Task: Conductor - User Manual Verification 'Remove Target Input & Ensure Auto-Timer' (Protocol in workflow.md)

## Phase 2: Implement Spacebar Play/Pause Toggle
- [ ] Task: Add a test in `src/logic_test.rs` for toggling the session state (Play/Pause) using the Spacebar.
- [ ] Task: Update `src/logic.rs` to handle Spacebar input and toggle the `SequenceState` between `Playing` and `Paused`.
- [ ] Task: Conductor - User Manual Verification 'Implement Spacebar Play/Pause Toggle' (Protocol in workflow.md)

## Phase 3: Remove Visual Feedback and Highlights
- [ ] Task: Update `src/components_test.rs` and `src/logic_test.rs` to remove tests related to feedback colors, timers, and active highlights.
- [ ] Task: Update `src/components.rs` to remove `FeedbackTimer` and related visual feedback components.
- [ ] Task: Update `src/constants.rs` to remove success, failure, and highlight color definitions (leaving just standard text/target appearance).
- [ ] Task: Update `src/logic.rs` to remove `handle_feedback`, `update_feedback_visuals`, and modify `sync_target_visuals` to prevent color changes.
- [ ] Task: Update `src/main.rs` to remove the feedback systems from the application build.
- [ ] Task: Conductor - User Manual Verification 'Remove Visual Feedback and Highlights' (Protocol in workflow.md)