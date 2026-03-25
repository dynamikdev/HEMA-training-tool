# Track: Meyer's Square Workflow Implementation

## Overview
This track implements a second training workflow based on the 16th-century **Meyer's Square** (Carre Meyer) diagram. It replaces the circular 8-point system with a four-quadrant concentric grid. The interaction is **purely visual**, where a "Ghost of Meyer" guide indicates the path and technique (cut, thrust, parry) the user should execute.

## Functional Requirements
### 1. Geometric Grid (Meyer's Square)
- Render a four-quadrant system with a stark bounding square and a thick central cross.
- Implement 16 target nodes arranged in four concentric square sequences (Outer, Outer-Mid, Inner-Mid, Inner).
- Each sequence consists of a 4-strike order crossing through the central intersection.

### 2. "Ghost of Meyer" Visual Guidance
- Implement a **Temporal Kinetic Guide** (the "Ghost") that provides visual-only cues:
    - **Slash (Cut)**: A high-intensity "Kinetic Slash" traveling from start to end with a decaying "blade-width" trail.
    - **Beam (Thrust)**: A sharp, narrow "Focus Beam" telescoping outward from the center/previous node to the next.
    - **Shield (Parry)**: A "Static Shield" or "Block" that appears on a line, strobing/blinking to signal a defensive hold.
- Distinguish the four sequences using **Glow Intensity** and pulse frequency rather than color.

### 3. Training Logic (Technique Mode)
- The system cycles through the 16 nodes in their defined sequences.
- The "Ghost" changes shape (Slash, Beam, Shield) to signal the required technique for the next strike.
- No keyboard interaction required; the session is a continuous visual drill.

### 4. UI/UX Transitions
- Add a "Workflow" toggle in the Settings panel to switch between `Circular` and `Meyer Square`.
- The UI should adapt the "Active Canvas" to display either the circular targets or the geometric Meyer grid.

## Non-Functional Requirements
- **Performance**: Maintain smooth rendering of the "Ghost" paths and trails in Bevy.
- **Aesthetic**: Adhere to the "Kinetic Brutalism" style (high contrast, emissive red glow, sharp lines).

## Acceptance Criteria
- [ ] User can switch between Circular and Meyer Square workflows in Settings.
- [ ] Meyer Square grid renders correctly with 16 nodes and central cross.
- [ ] "Ghost of Meyer" correctly visualizes Slash, Beam, and Shield techniques.
- [ ] The sequences follow the historically accurate 4x4 strike patterns.
- [ ] Training session progresses purely visually without keyboard input.

## Out of Scope
- Keyboard input for Meyer's Square (visual-only MVP).
- Custom sequence creation (using historical sequences only).
