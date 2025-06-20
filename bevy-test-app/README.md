# Bevy Test App

A headless Bevy application that renders a cube and saves a screenshot.

## Features

- Headless rendering (invisible window)
- Renders a single cube in the center with proper lighting
- Captures the rendered frame to an off-screen buffer
- Saves the result as `screenshot.png`
- Uses GPU acceleration (Vulkan/OpenGL)

## Requirements

- Rust 1.70+
- GPU with Vulkan or OpenGL support
- Linux with X11 support

## Running

```bash
cargo run --release
```

The application will:
1. Set up a 3D scene with a red cube and directional lighting
2. Render the scene to an off-screen buffer (1920x1080)
3. Save the rendered image as `screenshot.png`
4. Exit automatically

## Output

The application creates a `screenshot.png` file in the project directory showing a red cube from an angled perspective with proper 3D lighting and shadows.

## Dependencies

- Bevy 0.14 with minimal features for headless rendering
- image crate for PNG export
- Uses invisible window for GPU context without desktop interaction
