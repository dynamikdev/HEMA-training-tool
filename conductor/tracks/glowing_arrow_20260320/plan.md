# Implementation Plan: Glowing Arrow Visual Guide

## Phase 1: HDR & Bloom Setup [checkpoint: 839d58f]
- [x] Task: Update `src/main.rs` to enable HDR on the primary camera and add the `Bloom` component. fa767b3
- [x] Task: Conductor - User Manual Verification 'HDR & Bloom Setup' (Protocol in workflow.md)

## Phase 2: Arrow Logic & Data Structures [checkpoint: 45d7c06]
- [x] Task: Define `GlowingArrow` component and necessary resources in `src/components.rs` or `src/resources.rs`. b4982e6
- [x] Task: Write tests in `src/logic_test.rs` for calculating the diametrically opposite target index. e12cda2
- [x] Task: Implement a system in `src/logic.rs` that calculates the arrow's start and end points when `CurrentNumber` changes. d5ce6c9
- [x] Task: Conductor - User Manual Verification 'Arrow Logic & Data Structures' (Protocol in workflow.md)

## Phase 3: Visual Rendering & Animation [checkpoint: e78b2ae]
- [x] Task: Write tests for the arrow's "shoot" animation logic (path interpolation). 62b26b4
- [x] Task: Implement the arrow rendering system using Bevy's `Gizmos` (or a custom mesh) with an emissive color to trigger Bloom. ce7a014
- [x] Task: Implement the "Animated Path" logic to make the arrow shoot from origin to destination. ce7a014
- [x] Task: Conductor - User Manual Verification 'Visual Rendering & Animation' (Protocol in workflow.md)

## Phase 4: Refinement & Style Compliance [checkpoint: f4b5af8]
- [x] Task: Fine-tune Bloom parameters (intensity, threshold) to match the "red and glowing" requirement. 46f94d7
- [x] Task: Verify the arrow's visibility and contrast against the existing circle and background. 813e3c9
- [x] Task: Conductor - User Manual Verification 'Refinement & Style Compliance' (Protocol in workflow.md)
