# HEMA Training Tool

A reaction training application for HEMA (Historical European Martial Arts) practitioners, built with the Bevy game engine.

## Overview

The tool displays a set of numbers (1-8) arranged in a circle. It highlights targets in a sequence, allowing practitioners to practice their strike transitions and reaction times.

### Features
- **Two Sequence Modes**:
    - **Random**: Targets are highlighted in a randomized order.
    - **Ordered**: Targets are highlighted in numeric sequence (1 to 8).
- **Two Rhythm Modes**:
    - **Constant**: A steady interval between highlights, adjustable via a slider.
    - **Accelerate**: The sequence starts at the selected speed and automatically speeds up every 8 steps.
- **Dynamic Layout**: The circular arrangement of numbers automatically scales and centers itself based on the window size.

## Getting Started

### Prerequisites
- [Rust and Cargo](https://rustup.rs/)

### Running the Tool
To start the application, run:
```bash
cargo run
```

### Controls
Use the side panel to:
1. **Toggle Mode**: Switch between Random and Ordered sequences.
2. **Start/Stop**: Launch or halt the training sequence.
3. **Toggle Rhythm**: Switch between Constant and Accelerate timing.
4. **Adjust Speed**: Use the vertical slider to set the baseline interval (0.5s to 3.0s).

## Documentation
The codebase is fully documented using Rust doc comments. To generate and view the documentation locally:
```bash
cargo doc --no-deps --open
```

## Related Projects
- [bevy_ui_widgets](https://github.com/vlad-dragosh/bevy_ui_widgets): Used for the slider component.
