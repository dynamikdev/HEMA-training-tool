# Implementation Plan: Glowing Arrow Visual Guide

## Phase 1: HDR & Bloom Setup [checkpoint: 839d58f]
- [x] Task: Update `src/main.rs` to enable HDR on the primary camera and add the `Bloom` component. fa767b3
- [x] Task: Conductor - User Manual Verification 'HDR & Bloom Setup' (Protocol in workflow.md)

## Phase 2: Arrow Logic & Data Structures
- [ ] Task: Define `GlowingArrow` component and necessary resources in `src/components.rs` or `src/resources.rs`.
- [ ] Task: Write tests in `src/logic_test.rs` for calculating the diametrically opposite target index.
- [ ] Task: Implement a system in `src/logic.rs` that calculates the arrow's start and end points when `CurrentNumber` changes.
- [ ] Task: Conductor - User Manual Verification 'Arrow Logic & Data Structures' (Protocol in workflow.md)

## Phase 3: Visual Rendering & Animation
- [ ] Task: Write tests for the arrow's "shoot" animation logic (path interpolation).
- [ ] Task: Implement the arrow rendering system using Bevy's `Gizmos` (or a custom mesh) with an emissive color to trigger Bloom.
- [ ] Task: Implement the "Animated Path" logic to make the arrow shoot from origin to destination.
- [ ] Task: Conductor - User Manual Verification 'Visual Rendering & Animation' (Protocol in workflow.md)

## Phase 4: Refinement & Style Compliance
- [ ] Task: Fine-tune Bloom parameters (intensity, threshold) to match the "red and glowing" requirement.
- [ ] Task: Verify the arrow's visibility and contrast against the existing circle and background.
- [ ] Task: Conductor - User Manual Verification 'Refinement & Style Compliance' (Protocol in workflow.md)
