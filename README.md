# Phosphlux Lite

A simple, unified video synthesizer with a fixed 7-stage signal chain. Captures the aesthetic of vintage analog video synthesis without requiring hardware-accurate emulation.

## Inspiration

- **Paik/Abe Video Synthesizer** - Wobbulation, colorization
- **Sandin Image Processor** - Amplitude classification, keying
- **Rutt/Etra Scan Processor** - Z-axis displacement, 3D terrain effects
- **Jones Colorizer** - Threshold-based colorization bands
- **LZX Cadet** - Modular signal flow concepts
- **Beck Direct Video Synthesizer** - Oscillator-based pattern generation
- **VHS/CRT aesthetics** - Tape degradation, scanlines, phosphor glow
- **Sony PVM monitors** - Broadcast monitor aesthetic with bezel overlay

## Signal Chain

```
[INPUT] → [GEOMETRY] → [AMPLITUDE] → [COLORIZE] → [FEEDBACK] → [OUTPUT]
```

### Stage 1: Input Matrix
Generate and mix signal sources:
- **Sources**: Horizontal/vertical ramps, sine oscillators, white/pink/brown noise, geometric shapes, checkerboard
- **Controls**: Source A/B selection, mix blend, frequency, phase, rotation

### Stage 2: Geometry
Spatial distortions inspired by analog video processors:
- **Wobbulation**: Horizontal/vertical wave distortion (Paik/Abe style)
- **Z-Displacement**: Luminance-based vertical displacement (Rutt/Etra style)
- **Lissajous**: X/Y modulation patterns
- **Transform**: Rotation and scale

### Stage 3: Amplitude
Waveform shaping and signal processing:
- **Folding**: Wave folding with adjustable gain (creates harmonic complexity)
- **Quantization**: Reduce to N levels (Sandin Amplitude Classifier style)
- **Soft Clip**: Gentle saturation
- **Solarize**: Threshold-based inversion
- **Gate**: Hard threshold cutoff
- **Invert**: Signal inversion

### Stage 4: Colorize
Map luminance to color:
- **Spectrum**: Rainbow gradient mapping
- **Threshold**: Quantized color bands (Jones Colorizer style)
- **Gradient**: Custom two-color gradient
- **Monochrome**: Grayscale output
- **Controls**: Hue offset, saturation adjustment

### Stage 5: Feedback
Temporal effects using previous frame with mixing controls:
- **Mix**: Feedback amount, blend modes (Mix, Add, Multiply, Screen, Overlay, Difference)
- **Luma Key**: Key based on luminance with threshold/softness
- **Transform**: Zoom, rotation, X/Y offset
- **Color**: Hue shift, decay, saturation

### Stage 6: Output
Stackable display effects (applied in order: VHS → Cable → CRT):
- **VHS**: Tracking errors, tape wobble, chroma shift, noise
- **Cable**: Bandwidth limiting, RF ghosting, noise
- **CRT**: Scanlines, bloom, vignette

## Features

### PVM Bezel Overlay
Sony PVM-style monitor bezel frames the output for authentic broadcast monitor aesthetic. Configurable in Settings:
- **Show/Hide**: Toggle bezel visibility
- **Zoom**: Scale the display (default 1.8x)
- **Position**: Vertical offset adjustment

### LFO Automation
Per-parameter LFO modulation with BPM sync:
- Click the button next to any slider to cycle: Off → Slow → Medium → Fast → Off
- Right-click to immediately disable
- Expanded controls when active: range (lo/hi), phase offset, tempo subdivision
- Global BPM control in header (60, 90, 120, 140 presets or custom)

### Randomize
One-click randomization of all synthesis parameters for instant inspiration.

## Built-in Presets

| Preset | Description |
|--------|-------------|
| Feedback Spiral | Classic video feedback with zoom and rotation |
| Wobbulator | Paik/Abe style wobbulation distortion |
| Rutt/Etra Terrain | Z-axis displacement creating 3D terrain |
| VHS Memory | Degraded VHS tape aesthetic |
| Colorizer Bands | Jones-style banded colorization |
| Noise Meditation | Slow-moving noise patterns |

## Usage

```bash
# Clone and run
git clone https://github.com/templeoflum/phosphlux-lite.git
cd phosphlux-lite
cargo run --release
```

## Controls

- **Top Panel**: Preset selection, Randomize button, BPM controls, Settings (gear icon)
- **Right Panel**: Stage tabs (INPUT, GEOM, AMP, COLOR, FB, OUT)
- **Center**: Video preview with PVM bezel overlay
- **Settings Window**: Bezel toggle, zoom, position, and alignment adjustments

Click a stage tab to reveal its controls. All parameters update in real-time. Click the LFO button (~/S/M/F) next to sliders to add automation.

## Technical Details

- **Resolution**: 640x480 internal rendering
- **Window**: Launches maximized
- **GPU**: wgpu (Vulkan/Metal/DX12)
- **UI**: egui immediate-mode GUI
- **Feedback**: Ping-pong texture buffers for temporal effects
- **Bezel**: PNG overlay with configurable screen region

## Future Plans

- External video input (webcam, capture card)
- MIDI CC mapping for all parameters
- Audio reactivity (envelope follower)
- Preset export/import
- Recording to video file
