# Track Specification: Glowing Arrow Visual Guide

## Overview
Add a large, red, and glowing arrow that points from the currently active target to its diametrically opposite target on the Meyer's Square (Circle). This visual guide helps the practitioner visualize the line of the cut during training.

## Functional Requirements
1.  **Arrow Visualization**: A solid red arrow must be rendered connecting the active target to the opposite target.
2.  **Bloom Glow**: The arrow should use Bevy's Bloom post-processing effect to create a vibrant, glowing appearance (requires HDR and Bloom configuration).
3.  **Path Animation**: When a target changes, the arrow should animate its path (e.g., "shooting" out) from the origin target toward the destination target.
4.  **Dynamic Update**: The arrow must automatically update its position and direction whenever the `CurrentNumber` resource changes.

## Non-Functional Requirements
1.  **High Visibility**: The arrow must be easily visible from a distance (high contrast/red glow).
2.  **Performance**: The Bloom effect and arrow rendering should maintain the application's target frame rate.

## Acceptance Criteria
1.  **Given** the training session is running, **when** the auto-timer switches the active target, **then** a red glowing arrow appears pointing to the opposite target.
2.  **Given** a new target becomes active, **when** the arrow appears, **then** it animates along the cut line from start to finish.
3.  **Given** the arrow is displayed, **when** viewed on screen, **then** it exhibits a distinct Bloom glow effect.

## Out of Scope
-   Adding audio cues for the arrow.
-   Adding multiple arrows for simultaneous targets.
-   Changing the target circle layout.
