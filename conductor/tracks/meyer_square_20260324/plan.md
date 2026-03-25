# Implementation Plan: Meyer's Square Workflow

## Phase 1: Data Structures & Core Logic
### Task: Implement Meyer's Square Data Structures
- [ ] Task: Define `MeyerSequence` and `MeyerNode` structs for 16-target geometry
    - [ ] Create `MeyerNode` struct with (x, y) coordinates and technique type
    - [ ] Define the 4 concentric 4-strike sequences using historical data
- [ ] Task: Implement `MeyerTrainingResource` to manage visual-only flow
    - [ ] Add state to track current sequence, node, and technique
    - [ ] Implement logic to cycle through sequences without keyboard input
- [ ] Task: Conductor - User Manual Verification 'Data Structures & Core Logic' (Protocol in workflow.md)

## Phase 2: UI & Render Foundations
### Task: Add Workflow Toggle in Settings
- [ ] Task: Update `Settings` component to include a `TrainingWorkflow` enum (Circular, MeyerSquare)
- [ ] Task: Implement UI button in the side panel to toggle between workflows
- [ ] Task: Refactor `Target` rendering system to switch based on the active workflow
- [ ] Task: Conductor - User Manual Verification 'UI & Render Foundations' (Protocol in workflow.md)

## Phase 3: "Ghost of Meyer" Visual Signatures
### Task: Implement Geometric Grid Rendering
- [ ] Task: Render the bounding square and thick central cross in "GHOST_BORDER" style
- [ ] Task: Render the 16 nodes as sharp, low-opacity markers in their concentric squares
- [ ] Task: Implement "Glow Intensity" logic to highlight the active sequence
### Task: Implement Technique Visual Signatures
- [ ] Task: Implement "Slash (Cut)" shader/system with decaying trail
- [ ] Task: Implement "Beam (Thrust)" telescoping line system
- [ ] Task: Implement "Shield (Parry)" strobing/blinking visual marker
- [ ] Task: Conductor - User Manual Verification 'Visual Signatures' (Protocol in workflow.md)

## Phase 4: Integration & Training Flow
### Task: Finalize Training Loop for Meyer's Square
- [ ] Task: Connect the `MeyerTrainingResource` logic to the "Ghost" rendering systems
- [ ] Task: Ensure smooth transitions between sequences and nodes
- [ ] Task: Final polish of "Kinetic Brutalism" aesthetic for the Meyer workflow
- [ ] Task: Conductor - User Manual Verification 'Integration & Training Flow' (Protocol in workflow.md)
