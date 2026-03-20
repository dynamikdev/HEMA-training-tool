# Track Specification: Remove Target Keystroke Interaction

## Overview
Remove the keystroke (1-8) interaction with the target circle. The primary goal of the project is to display sword movements for the user to physically train against, so requiring keystrokes to follow the display is counterproductive. The application will instead rely on an auto-timer to transition between targets, and use the Spacebar to toggle play/pause states.

## Functional Requirements
1.  **Auto-Timer Transition**: The training loop must automatically advance to the next target after a specific time (defined by the selected rhythm: constant or accelerating), without requiring user input.
2.  **Remove Target Input**: Remove all code and logic that maps numeric keys (1-8) to specific targets.
3.  **Spacebar Control**: Implement/retain the ability to toggle between Play and Pause states for the training session using the Spacebar.
4.  **Remove Visual Feedback & Highlights**: Remove the success (green) and failure (red) color feedback, as well as any active target highlight color. Targets should just display their number/symbol with no color changes during their active state.

## Non-Functional Requirements
1.  The codebase should be cleaned of unnecessary input handling, feedback timers, and color-switching structures to improve maintainability.

## Acceptance Criteria
1.  **Given** the user starts a training session, **when** the timer for a target expires, **then** the next target should automatically be displayed without requiring a keystroke.
2.  **Given** the training session is active, **when** the user presses keys 1-8, **then** the inputs are ignored.
3.  **Given** the training session is active, **when** the user presses the Spacebar, **then** the session pauses. Pressing it again resumes the session.
4.  **Given** a target becomes active, **when** it is displayed, **then** no color changes occur (no active highlight, no success/fail feedback).

## Out of Scope
-   Changing the structural layout of the targets (they remain in a circle).
-   Adding new guard positions or rhythms.