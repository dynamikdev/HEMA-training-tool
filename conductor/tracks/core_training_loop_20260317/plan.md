# Implementation Plan: Implement core reaction training loop

## Phase 1: Core Logic Setup [checkpoint: b89a47d]
- [x] Task: Set up basic Bevy app structure. 2a53e4f
    - [x] Task: Initialize Bevy app and window. 2a53e4f
    - [x] Task: Add necessary Bevy plugins. 2a53e4f
- [x] Task: Define core data structures for targets and sequences. 2a53e4f
    - [x] Task: Create `Target` struct. 2a53e4f
    - [x] Task: Create `Sequence` struct (handling modes). 2a53e4f
- [x] Task: Implement target display system. 2a53e4f
    - [x] Task: Render numerical targets on screen. 2a53e4f
    - [x] Task: Logic for highlighting active target. 2a53e4f
- [x] Task: Implement sequence generation logic. 2a53e4f
    - [x] Task: Implement random sequence generation. 2a53e4f
    - [x] Task: Implement ordered sequence generation. 2a53e4f
- [x] Task: Implement rhythm modes logic. a13354c
    - [x] Task: Implement constant rhythm timer. a13354c
    - [x] Task: Implement accelerating rhythm timer. a13354c
- [x] Task: Conductor - User Manual Verification 'Phase 1: Core Logic Setup' (Protocol in workflow.md) b89a47d

## Phase 2: User Interaction and Feedback
- [x] Task: Implement UI controls for mode selection. 2a53e4f
    - [x] Task: Add buttons for Random/Ordered and Constant/Accelerate. 2a53e4f
    - [x] Task: Connect UI actions to game state. 2a53e4f
- [x] Task: Implement user input handling for target interaction. 76c9a4e
    - [x] Task: Detect user input (e.g., key press, click). 76c9a4e
    - [x] Task: Validate user input against active target. 76c9a4e
- [x] Task: Provide visual feedback on user interaction. 76c9a4e
    - [x] Task: Indicate correct vs. incorrect input. 76c9a4e
- [ ] Task: Conductor - User Manual Verification 'Phase 2: User Interaction and Feedback' (Protocol in workflow.md)

## Phase 3: Polish and Refinement
- [ ] Task: Adjust UI for responsiveness and aesthetics.
    - [ ] Task: Ensure targets scale correctly.
    - [ ] Task: Apply high-contrast styling.
- [ ] Task: Add basic performance tracking (session only).
    - [ ] Task: Track correct/incorrect counts.
- [ ] Task: Conductor - User Manual Verification 'Phase 3: Polish and Refinement' (Protocol in workflow.md)
