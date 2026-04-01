# Track: Curriculum Integration

## Overview
This feature integrates the HEMA curriculum documents (extracted as images from PDFs) into the application. It allows users to browse and read the "Passage de Grade" requirements directly within the tool, replacing the main training area when active.

## Functional Requirements
- **Sidebar Navigation:** Add a new "Curriculum" tab or section to the settings sidebar.
- **Grade & Document Selection:** Users can select the Grade Level (e.g., Niveau 1.1) and specific documents from the sidebar.
- **Document Visualisation:** When a document is selected, the central training circle is replaced by the document viewer (displaying the extracted JPEG pages).
- **Page Navigation:**
    - **Visual Buttons:** "Previous" and "Next" buttons in the UI.
    - **Keyboard Shortcuts:** Use Left and Right arrow keys to flip pages.
- **Training Interruption:** Opening the curriculum view automatically pauses any active training sequence (`SequenceState.running = false`).

## UI Design
- **Main View Replacement:** The document page (JPEG) should scale to fill the primary training area (the left side of the screen) while maintaining its aspect ratio.
- **Sidebar Integration:** Navigation controls for grades, documents, and page numbers integrated into the right-hand settings panel.

## Acceptance Criteria
- [ ] Users can browse all grade levels found in `assets/Grades Escrime`.
- [ ] Selecting a document successfully loads and displays the first page.
- [ ] Page navigation (Prev/Next) works via UI buttons and keyboard arrows.
- [ ] Opening the curriculum pauses the trainer.
- [ ] Transitioning back to "Training" restores the training circle.

## Out of Scope
- Native PDF/DOCX rendering (using extracted images instead).
- Video playback for curriculum videos (only documents are integrated in this track).
- Full-screen modal overlay (using main area replacement instead).
