# Design System: HEMA Training Tool
**Project ID:** HEMA-TRAIN-001

## 1. Visual Theme & Atmosphere
**Stark, Focused, and Aggressive.** The "Combat Dashboard" aesthetic prioritizes extreme legibility and zero distractions. The interface is designed for high-visibility from a distance (at least 2 meters), ensuring practitioners can see cues while moving.

*   **Vibe:** Industrial Minimalist / Combat Ready.
*   **Aesthetic Philosophy:** Function over form. Every element must serve a training purpose.

## 2. Color Palette & Roles
*   **Base Background:** `#000000` (Deep Black)
    *   *Role:* Primary canvas. Eliminates visual noise.
*   **Active Signal:** `#ff0000` (Emissive Red)
    *   *Role:* Active targets, guide arrows, and immediate visual feedback. Requires HDR/Bloom to trigger "glow."
*   **Static Information:** `#ffffff` (Pure White)
    *   *Role:* Inactive targets, labels, borders, and primary text.
*   **UI Surface:** `#1a1a1a` (Dark Gray)
    *   *Role:* Side panel backgrounds and inactive control elements.

## 3. Typography Rules
*   **Family:** Sans-serif (System default / Robust Inter-style).
*   **Weights:** Bold (700+) for targets; Medium (500) for labels.
*   **Dynamic Scaling:** Font size scales linearly with window/radius size to maintain visual hierarchy.

## 4. Component Stylings
*   **Targets (Numerals):** Large, centered numerals. High contrast against black. White (Inactive) vs. Glowing Red (Active).
*   **Guide Arrow:** Thick, compound line with emissive red glow. Indicates the vector of the required cut.
*   **Controls (Sliders/Buttons):** Stark white borders (1-2px) on dark gray backgrounds. No rounded corners (Strictly square/orthogonal).

## 5. Layout Principles
*   **Panel Width:** Fixed 300px for controls.
*   **Training Area:** Flexible circular layout utilizing 80% of available screen radius.
*   **Whitespace:** Generous padding around targets to prevent visual crowding during fast sequences.
