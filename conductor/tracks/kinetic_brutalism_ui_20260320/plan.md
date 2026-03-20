# Implementation Plan: Kinetic Brutalism UI Overhaul

## Objective
Adapt the HEMA Training Tool's UI to the "Kinetic Brutalism" design system. This involves implementing extreme high-contrast ratios, a zero-radius rounding scale, specific typography (Space Grotesk, Work Sans), and a focused monochromatic palette with emissive active signals.

## Phase 1: Asset Integration & Constants Update
- [x] Task: Create an `assets/fonts/` directory.
- [x] Task: Update `src/constants.rs` to define the Kinetic Brutalism color palette.
- [x] Task: Create `Typography` resource and load custom fonts in `src/resources.rs` and `src/ui/setup.rs`.

## Phase 2: Telemetry Rail (Sidebar) Styling
- [x] Task: Update `src/ui/settings.rs` to use `SURFACE_LOW` background and Work Sans typography.
- [x] Task: Restyle buttons with Ghost Borders and no rounded corners.
- [x] Task: Restyle sliders with high-contrast square thumbs.

## Phase 3: Active Canvas (Combat Zone) Styling
- [x] Task: Update `src/ui/setup.rs` to use the `BACKGROUND_COLOR`.
- [x] Task: Update `src/ui/target.rs` and `src/ui/systems.rs` to use Space Grotesk Bold and `PRIMARY_EMISSIVE`.
- [x] Task: Update the Glowing Arrow guide to use the `PRIMARY_EMISSIVE` color.

## Phase 4: Review & Fixes
- [x] Task: Apply review suggestions (fix `BorderColor`, `into()` ambiguity, and missing `CIRCLE_RADIUS`).

## Phase 5: Verification
- [x] Task: Verify that no UI elements possess rounded corners.
- [x] Task: Verify the bloom effect accurately mimics the "Active Signal" light source.
