# Implementation Plan: Implement core reaction training loop

## Phase 1: Core Logic Setup
- [ ] Task: Set up basic Bevy app structure.
    - [ ] Task: Initialize Bevy app and window.
    - [ ] Task: Add necessary Bevy plugins.
- [ ] Task: Define core data structures for targets and sequences.
    - [ ] Task: Create `Target` struct.
    - [ ] Task: Create `Sequence` struct (handling modes).
- [ ] Task: Implement target display system.
    - [ ] Task: Render numerical targets on screen.
    - [ ] Task: Logic for highlighting active target.
- [ ] Task: Implement sequence generation logic.
    - [ ] Task: Implement random sequence generation.
    - [ ] Task: Implement ordered sequence generation.
- [ ] Task: Implement rhythm modes logic.
    - [ ] Task: Implement constant rhythm timer.
    - [ ] Task: Implement accelerating rhythm timer.
- [ ] Task: Conductor - User Manual Verification 'Phase 1: Core Logic Setup' (Protocol in workflow.md)

## Phase 2: User Interaction and Feedback
- [ ] Task: Implement UI controls for mode selection.
    - [ ] Task: Add buttons for Random/Ordered and Constant/Accelerate.
    - [ ] Task: Connect UI actions to game state.
- [ ] Task: Implement user input handling for target interaction.
    - [ ] Task: Detect user input (e.g., key press, click).
    - [ ] Task: Validate user input against active target.
- [ ] Task: Provide visual feedback on user interaction.
    - [ ] Task: Indicate correct vs. incorrect input.
- [ ] Task: Conductor - User Manual Verification 'Phase 2: User Interaction and Feedback' (Protocol in workflow.md)

## Phase 3: Polish and Refinement
- [ ] Task: Adjust UI for responsiveness and aesthetics.
    - [ ] Task: Ensure targets scale correctly.
    - [ ] Task: Apply high-contrast styling.
- [ ] Task: Add basic performance tracking (session only).
    - [ ] Task: Track correct/incorrect counts.
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Polish and Refinement' (Protocol in workflow.md)
