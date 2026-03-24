# Proposal: Meyer's Square Training Workflow

## Overview
This proposal outlines the implementation of a second main training workflow based on the traditional **Meyer's Square** (Carre Meyer) diagram. This will complement the existing circular target system by providing a structured geometric grid layout used extensively in Historical European Martial Arts (HEMA), specifically focusing on the exact geometry of the 16th-century reference diagram.

## Research Synthesis

### 1. Current State Analysis
- **Logic**: The tool currently uses an 8-point circular system.
- **UI/UX**: The layout is dynamic and circular. Interaction is via numbers 1-8.
- **Design System**: "Kinetic Brutalism" featuring high-contrast monochromatic surfaces (`#131313`, `#1e1e1e`, `#e2e2e2`) and high-intensity emissive signals (glowing red).

### 2. Meyer's Square Exact Geometry
Based on the `assets/carre_meyer.jpg` reference, the structure is a four-quadrant system with concentric sequences, rather than a generic 9-point grid.
- **Lines and Intersections**: 
    - A stark bounding square.
    - A thick central cross (one vertical line, one horizontal line) dividing the canvas into four equal quadrants.
    - Four concentric square bands forming the target zones.
- **Numbering & Target Coordinates**: 16 specific target points arranged in four 4-strike sequences. Each sequence has one node per quadrant, crossing the center:
    - **Sequence 1 (Red / Outer)**: 1 (Top-Right) &rarr; 2 (Bottom-Left) &rarr; 3 (Bottom-Right) &rarr; 4 (Top-Left).
    - **Sequence 2 (Green / Inner-Mid)**: 1 (Bottom-Right) &rarr; 2 (Top-Left) &rarr; 3 (Top-Right) &rarr; 4 (Bottom-Left).
    - **Sequence 3 (Blue / Outer-Mid)**: 1 (Top-Left) &rarr; 2 (Bottom-Right) &rarr; 3 (Bottom-Left) &rarr; 4 (Top-Right).
    - **Sequence 4 (Yellow / Inner)**: 1 (Bottom-Left) &rarr; 2 (Top-Right) &rarr; 3 (Top-Left) &rarr; 4 (Bottom-Right).

## Proposed Implementation Roadmap

### Phase 1: Strict Geometric State Foundations
- **Target Coordinates**: Define exact (x, y) coordinates for the 16 nodes within the 4 quadrants, scaled proportionately from the center to match the concentric squares of the diagram.
- **Data Structure**: Create a `MeyerSequence` struct to handle the 4 distinct geometric patterns and their internal 4-strike order.

### Phase 2: Specific Training Patterns
Define training logic based on the exact lines and paths of the diagram:
- **Diagonal Patterns (Numbered Sequences)**: Implement the sequences exactly as numbered (1 to 2, 3 to 4) to train descending (*Oberhau* / *Zornhau*) and ascending (*Unterhau*) diagonal cuts that pass precisely through the central intersection.
- **Horizontal & Vertical Patterns (The Cross)**: Utilize the central black cross for specific structural strikes:
    - **Horizontal Path**: Left-to-right and right-to-left strikes along the horizontal axis (e.g., *Zwerchhau*, *Mittelhau*).
    - **Vertical Path**: Top-to-bottom along the vertical axis (e.g., *Scheitelhau*).

### Phase 3: Visual Overlay (Kinetic Brutalism)
Render the diagram as a stark vector overlay in the 'Active Canvas':
- **Base Geometry**: Draw the bounding box and central cross using razor-sharp, zero-radius lines in `GHOST_BORDER` (20% opacity white) to maintain the brutalist aesthetic.
- **Concentric Paths**: Render the four concentric squares as sharp, high-contrast monochrome vector outlines. 
- **Emissive Signals**: Use the `PRIMARY_EMISSIVE` red glow to highlight the specific nodes or the active concentric square currently being trained.
- **Kinetic Feedback**: When a sequence is active (e.g., 1 &rarr; 2), a sharp, glowing vector arrow pulses along the diagonal or orthogonal path, leaving a decaying trail to emphasize the motion of the blade through the cross.

### Phase 4: UI/UX Transitions
- **Workflow Toggle**: Add a "Workflow" button in Settings to switch between `Circular` and `Meyer Square`.

### Phase 5: Verification & Testing
- Ensure the geometric layout matches the reference diagram exactly on screen.
- Verify that the specific patterns accurately follow the horizontal, vertical, and diagonal paths outlined in the plan.