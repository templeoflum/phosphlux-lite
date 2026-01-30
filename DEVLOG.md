# Phosphlux Lite Development Log

## v0.2.0 - UI Overhaul & Automation (2026-01-30)

### New Features

**PVM Bezel Overlay**
- Sony PVM-14 style monitor bezel frames the output
- PNG overlay composited over synth output in egui
- Configurable screen region alignment (left/top/right/bottom)
- Zoom control (0.5x - 2.0x, default 1.8x)
- Vertical position offset for centering
- Toggle to show/hide bezel

**LFO Automation System**
- Per-parameter LFO modulation ported from main Phosphlux
- Button next to each slider cycles: Off → Slow (0.1x) → Medium (0.25x) → Fast (0.5x) → Off
- Right-click to immediately disable
- Expanded controls when active: range (lo/hi), phase offset, tempo subdivision
- Global BPM control with presets (60, 90, 120, 140) and custom input
- BPM synced oscillation for all active LFOs

**Output Stage Refactored**
- Changed from mode-based (Clean/CRT/VHS/Cable) to stackable toggleable effects
- Effects applied in fixed order: VHS → Cable → CRT
- Each effect independently toggleable
- VHS/Cable effects use feedback texture sampling when feedback is active, simple math when off

**UI Restructured**
- Controls moved from bottom panel to right side panel
- Stage tabs in horizontal row at top of side panel
- Single-column layout for better fit in narrow panel
- Settings window (gear icon) for bezel configuration
- Window launches maximized by default

### Technical Changes

- Added `automation.rs` with `LfoState` and `AutomationState`
- Added `BezelSettings` struct with zoom, offset_y, enabled fields
- Bezel PNG loaded via `include_bytes!` and stored as `TextureHandle`
- VHS/cable shader functions now take `use_feedback` parameter
- Feedback active check: `fb_enabled > 0.5 && mixer_feedback_mix > 0.01`

---

## v0.1.0 - Initial Release (2026-01-30)

### Overview

First release of Phosphlux Lite - a simplified, fixed-pipeline video synthesizer designed for immediate playability while capturing the aesthetic of vintage analog video synthesis.

### Architecture Decisions

**Why a fixed pipeline?**
- Predictable signal flow: INPUT → GEOMETRY → AMPLITUDE → COLORIZE → MIXER → FEEDBACK → OUTPUT
- Every stage has a clear purpose inspired by classic hardware
- No routing decisions - just adjust parameters and explore
- Single shader file (`lite.wgsl`) is easier to understand and modify

**Stage design rationale:**
1. **Input**: Multiple generator types cover the basics (ramps, oscillators, noise, shapes)
2. **Geometry**: Combines wobbulation + z-displacement + lissajous - the spatial transformations
3. **Amplitude**: Wave shaping (fold, quantize, clip) - the signal processors
4. **Colorize**: Luminance-to-color mapping - the colorizers
5. **Mixer**: Feedback integration with blend modes - the compositing
6. **Feedback**: Temporal recursion - the time domain
7. **Output**: Display emulation - the final presentation

### Implementation Notes

**Uniform buffer packing:**
- 256-byte uniform struct covering all 7 stages
- Carefully aligned to 16-byte boundaries for GPU compatibility
- Timing (time, frame) included for animation

**Feedback system:**
- Ping-pong texture pair for temporal effects
- Reads from texture A, writes to texture B, then swaps
- Transforms (zoom, rotation, offset) applied during sampling
- Hue shift and decay for evolving colors

**Preset system:**
- Presets stored as `SynthState` structs with all 7 stages
- JSON serialization ready for save/load (not yet exposed in UI)
- 6 built-in presets demonstrating different synthesis styles

### File Structure

```
phosphlux-lite/
├── Cargo.toml          # Package manifest
├── README.md           # User documentation
├── DEVLOG.md           # This file
├── assets/
│   └── cutout/         # PVM bezel images
├── src/
│   ├── main.rs         # Entry point, window, event loop, bezel rendering
│   ├── app.rs          # Application state, presets, randomize, bezel settings
│   ├── automation.rs   # LFO automation system
│   ├── synth.rs        # Stage parameter structs, GPU uniforms
│   ├── renderer.rs     # wgpu renderer, feedback textures
│   ├── ui.rs           # egui UI, stage panels, settings window
│   └── presets.rs      # Built-in presets, save/load
└── shaders/
    └── lite.wgsl       # Fixed-pipeline synthesis shader
```

### Shader Organization

The `lite.wgsl` shader is organized by stage:

1. **Uniforms**: Single `Synth` struct with all parameters
2. **Utilities**: Hash, noise, hue rotation
3. **Stage 1 - Input**: `generate_source()`, `stage_input()`
4. **Stage 2 - Geometry**: `stage_geometry()` with wobbulation, z-displacement
5. **Stage 3 - Amplitude**: `fold()`, `quantize()`, `soft_clip()`, `solarize()`, `stage_amplitude()`
6. **Stage 4 - Colorize**: `colorize_spectrum()`, `colorize_threshold()`, `stage_colorize()`
7. **Stage 5 - Mixer**: Blend modes, luma key, `stage_mixer()`
8. **Stage 6 - Feedback**: Transform sampling, `stage_feedback()`
9. **Stage 7 - Output**: VHS/Cable/CRT effects with conditional feedback sampling
10. **Fragment main**: Orchestrates all stages in order

### Performance

- Release build runs at 60fps easily on integrated graphics
- Single draw call per frame (fullscreen quad)
- Feedback texture copy adds minimal overhead
- No graph compilation or dynamic dispatch

---

## Development History

### 2026-01-30: v0.2.0 Release

- Added LFO automation system with BPM sync
- Added PVM bezel overlay with zoom/position controls
- Refactored output stage to stackable toggleable effects
- Moved controls to right side panel
- Added settings window for bezel configuration
- Fixed VHS/cable effects feedback interaction (only when feedback active)
- Window now launches maximized

### 2026-01-30: v0.1.0 - Project Creation

- Created `phosphlux-lite/` directory structure
- Implemented all 7 stage parameter structs in `synth.rs`
- Built fixed-pipeline shader `lite.wgsl` with ~700 lines
- Created egui UI with stage tabs and sliders
- Added 6 built-in presets covering different synthesis styles
- Both `phosphlux` and `phosphlux-lite` build and run successfully
