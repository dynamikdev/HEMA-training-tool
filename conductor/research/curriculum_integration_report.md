# Technical Evaluation: Curriculum Document Integration

## Objective
Evaluate the feasibility of integrating HEMA curriculum documents (PDF, DOCX, MOV) from `assets/Grades Escrime` into the HEMA Training Tool.

## Technical Feasibility Analysis

| Integration Method | Feasibility | Effort | UX Impact |
| :--- | :--- | :--- | :--- |
| **Native Bevy PDF/DOCX Reader** | **Very Low** | **Very High** | High (Seamless but potentially buggy) |
| **External System Viewer** | **High** | **Very Low** | Medium (Context switch to OS) |
| **Static Image Extraction** | **High** | **Low** | High (Consistent with Kinetic Brutalism) |
| **Markdown Rendering** | **Medium** | **Medium** | Medium (Lacks complex diagrams) |

### 1. Native Document Rendering
As of Bevy 0.18.0, there are no stable, cross-platform plugins for PDF or DOCX rendering. Implementation would require integrating heavy C-libraries (e.g., Pdfium) and manually mapping rasterized buffers to Bevy `Image` assets. This contradicts the "Speed First" development principle.

### 2. Video Playback
The `.MOV` files in the curriculum are high-definition video drills. Bevy does not natively support video decoding. External players (VLC, QuickTime, Windows Media Player) provide a far superior experience with hardware acceleration and standard playback controls.

### 3. Kinetic Brutalism Alignment
The "Kinetic Brutalism" aesthetic (high contrast, monochromatic) is best complemented by high-resolution, stark PNG/JPG exports of the curriculum diagrams rather than a generic document viewer UI.

## Recommendations

### Phase 1: OS-Native Integration (Immediate)
- **Action**: Add the `opener` crate to `Cargo.toml`.
- **UI**: Add a "Curriculum" section to the settings sidebar.
- **Behavior**: Clicking a grade level (e.g., "Niveau 1.1") opens the folder or primary PDF using the system's default application.
- **Benefit**: Zero implementation risk; 100% compatibility with all file formats.

### Phase 2: Tactical Cheat Sheets (In-App)
- **Action**: Manually extract the primary "Drill Diagram" from each PDF as a high-contrast PNG.
- **UI**: Implement a "Quick Reference" toggle that displays these images as a semi-transparent overlay on the training canvas.
- **Benefit**: Allows the practitioner to check the footwork/geometry without leaving the app during a training loop.

### Phase 3: Markdown Curriculum (Future)
- **Action**: Re-author curriculum summaries in Markdown.
- **Benefit**: Enables interactive documents where clicking a "Drill 1" link automatically configures the app's target sequence and rhythm.

## Conclusion
We should proceed with **Phase 1** immediately to provide value to the user, while preparing **Phase 2** assets for a more polished, integrated feel that respects the app's unique visual style.
