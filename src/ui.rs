//! User interface using egui

use crate::app::{App, SelectedStage};
use crate::automation::AutomationState;
use crate::synth::*;
use egui::{Color32, RichText, Ui};

/// Draw the complete UI
pub fn draw_ui(ctx: &egui::Context, app: &mut App) {
    // Top panel with title, presets, and BPM
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.heading("PHOSPHLUX LITE");
            ui.separator();

            // Preset selector
            let preset_name = app
                .current_preset
                .and_then(|i| app.presets.get(i))
                .map(|p| p.name.clone())
                .unwrap_or_else(|| "(modified)".to_string());

            let preset_names: Vec<String> = app.presets.iter().map(|p| p.name.clone()).collect();
            let current = app.current_preset;

            let mut selected_preset: Option<usize> = None;
            egui::ComboBox::from_label("")
                .selected_text(&preset_name)
                .width(150.0)
                .show_ui(ui, |ui| {
                    for (i, name) in preset_names.iter().enumerate() {
                        if ui.selectable_label(current == Some(i), name).clicked() {
                            selected_preset = Some(i);
                        }
                    }
                });

            if let Some(idx) = selected_preset {
                app.load_preset(idx);
            }

            if ui.button("Randomize").clicked() {
                app.randomize();
            }

            ui.separator();

            // Global BPM control
            ui.label("BPM:");
            let bpm_presets = [60.0, 90.0, 120.0, 140.0];
            for bpm in bpm_presets {
                let selected = (app.automation.global_bpm - bpm).abs() < 1.0;
                if ui
                    .selectable_label(selected, format!("{}", bpm as u32))
                    .clicked()
                {
                    app.automation.global_bpm = bpm;
                }
            }

            // Custom BPM slider (compact)
            ui.add(
                egui::DragValue::new(&mut app.automation.global_bpm)
                    .speed(1.0)
                    .range(30.0..=240.0)
                    .suffix(" BPM"),
            );

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Settings button on the right
                if ui.button("⚙").clicked() {
                    app.show_settings = !app.show_settings;
                }
            });
        });
    });

    // Right side panel with stage tabs and controls
    egui::SidePanel::right("stage_panel")
        .min_width(320.0)
        .default_width(350.0)
        .show(ctx, |ui| {
            // Stage tab bar (vertical)
            ui.horizontal_wrapped(|ui| {
                let stages = [
                    (SelectedStage::Input, "INPUT"),
                    (SelectedStage::Geometry, "GEOM"),
                    (SelectedStage::Amplitude, "AMP"),
                    (SelectedStage::Colorize, "COLOR"),
                    (SelectedStage::Feedback, "FB"),
                    (SelectedStage::Output, "OUT"),
                ];

                for (stage, label) in stages {
                    let selected = app.selected_stage == stage;
                    let text = if selected {
                        RichText::new(label).strong().color(Color32::WHITE)
                    } else {
                        RichText::new(label).color(Color32::GRAY)
                    };

                    if ui.selectable_label(selected, text).clicked() {
                        app.selected_stage = stage;
                    }
                }
            });

            ui.separator();

            // Stage-specific controls in a scroll area
            egui::ScrollArea::vertical().show(ui, |ui| {
                let modified = match app.selected_stage {
                    SelectedStage::Input => draw_input_stage(ui, &mut app.synth.input, &mut app.automation),
                    SelectedStage::Geometry => draw_geometry_stage(ui, &mut app.synth.geometry, &mut app.automation),
                    SelectedStage::Amplitude => draw_amplitude_stage(ui, &mut app.synth.amplitude, &mut app.automation),
                    SelectedStage::Colorize => draw_colorize_stage(ui, &mut app.synth.colorize, &mut app.automation),
                    SelectedStage::Mixer => draw_feedback_stage(ui, &mut app.synth.feedback, &mut app.synth.mixer, &mut app.automation),
                    SelectedStage::Feedback => draw_feedback_stage(ui, &mut app.synth.feedback, &mut app.synth.mixer, &mut app.automation),
                    SelectedStage::Output => draw_output_stage(ui, &mut app.synth.output, &mut app.automation),
                };

                if modified {
                    app.mark_modified();
                }
            });
        });

    // Settings window (floating)
    if app.show_settings {
        egui::Window::new("Settings")
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.heading("Display");

                ui.checkbox(&mut app.bezel.enabled, "Show Bezel");

                ui.horizontal(|ui| {
                    ui.label("Zoom:");
                    ui.add(egui::Slider::new(&mut app.bezel.zoom, 0.5..=2.0).show_value(true));
                });

                ui.horizontal(|ui| {
                    ui.label("Position:");
                    ui.add(egui::Slider::new(&mut app.bezel.offset_y, -0.5..=0.5).show_value(true));
                });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(5.0);

                ui.heading("Bezel Position");
                ui.label("Adjust screen area within bezel:");

                ui.horizontal(|ui| {
                    ui.label("Left:");
                    ui.add(egui::DragValue::new(&mut app.bezel.left)
                        .speed(0.001)
                        .range(0.0..=0.5)
                        .fixed_decimals(3));
                });

                ui.horizontal(|ui| {
                    ui.label("Top:");
                    ui.add(egui::DragValue::new(&mut app.bezel.top)
                        .speed(0.001)
                        .range(0.0..=0.5)
                        .fixed_decimals(3));
                });

                ui.horizontal(|ui| {
                    ui.label("Right:");
                    ui.add(egui::DragValue::new(&mut app.bezel.right)
                        .speed(0.001)
                        .range(0.5..=1.0)
                        .fixed_decimals(3));
                });

                ui.horizontal(|ui| {
                    ui.label("Bottom:");
                    ui.add(egui::DragValue::new(&mut app.bezel.bottom)
                        .speed(0.001)
                        .range(0.5..=1.0)
                        .fixed_decimals(3));
                });

                ui.add_space(10.0);

                if ui.button("Reset to Default").clicked() {
                    app.bezel = crate::app::BezelSettings::default();
                }

                ui.add_space(10.0);

                if ui.button("Close").clicked() {
                    app.show_settings = false;
                }
            });
    }
}

/// Draw a slider with LFO toggle button
/// Returns true if the value was manually modified (which should disable LFO)
fn param_slider_with_lfo(
    ui: &mut Ui,
    label: &str,
    param_key: &str,
    value: &mut f32,
    range: std::ops::RangeInclusive<f32>,
    automation: &mut AutomationState,
) -> bool {
    let mut modified = false;

    ui.horizontal(|ui| {
        // LFO toggle button
        let lfo_active = automation.has_lfo(param_key);
        let button_text = if let Some(lfo) = automation.get_lfo(param_key) {
            match lfo.speed {
                s if s < 0.15 => "S",
                s if s < 0.4 => "M",
                _ => "F",
            }
        } else {
            "~"
        };

        let button_color = if lfo_active {
            match automation.get_lfo(param_key).map(|l| l.speed) {
                Some(s) if s < 0.15 => Color32::from_rgb(100, 200, 100), // Green - Slow
                Some(s) if s < 0.4 => Color32::from_rgb(200, 200, 100),  // Yellow - Medium
                _ => Color32::from_rgb(100, 200, 200),                    // Cyan - Fast
            }
        } else {
            Color32::from_rgb(80, 80, 80) // Gray - Off
        };

        let response = ui.add(
            egui::Button::new(RichText::new(button_text).monospace())
                .fill(button_color)
                .min_size(egui::vec2(22.0, 18.0)),
        );

        // Left click: cycle states
        if response.clicked() {
            automation.cycle_lfo(param_key, *range.start(), *range.end());
        }

        // Right click: disable
        if response.secondary_clicked() {
            automation.remove_lfo(param_key);
        }

        // Show tooltip
        response.on_hover_text("Left-click: cycle S/M/F/Off\nRight-click: disable");

        // Label
        ui.label(label);

        // Slider
        let slider_response = ui.add(egui::Slider::new(value, range.clone()).show_value(true));
        if slider_response.changed() {
            // Manual adjustment disables LFO
            automation.remove_lfo(param_key);
            modified = true;
        }
    });

    // Show expanded LFO controls if active
    if let Some(lfo) = automation.get_lfo_mut(param_key) {
        ui.indent(param_key, |ui| {
            ui.horizontal(|ui| {
                ui.label("Range:");
                ui.add(
                    egui::DragValue::new(&mut lfo.lo)
                        .speed(0.01)
                        .range(*range.start()..=lfo.hi)
                        .prefix("lo: "),
                );
                ui.add(
                    egui::DragValue::new(&mut lfo.hi)
                        .speed(0.01)
                        .range(lfo.lo..=*range.end())
                        .prefix("hi: "),
                );
            });
            ui.horizontal(|ui| {
                ui.label("Phase:");
                ui.add(egui::Slider::new(&mut lfo.offset, 0.0..=1.0).show_value(false));

                ui.label("Div:");
                egui::ComboBox::from_id_salt(format!("{}_subdiv", param_key))
                    .selected_text(format_subdivide(lfo.subdivide))
                    .width(50.0)
                    .show_ui(ui, |ui| {
                        for &sub in &[0.25, 0.5, 1.0, 2.0, 4.0] {
                            if ui
                                .selectable_label(
                                    (lfo.subdivide - sub).abs() < 0.01,
                                    format_subdivide(sub),
                                )
                                .clicked()
                            {
                                lfo.subdivide = sub;
                            }
                        }
                    });
            });
        });
    }

    modified
}

/// Format subdivide value for display
fn format_subdivide(val: f32) -> &'static str {
    if (val - 0.25).abs() < 0.01 {
        "1/4"
    } else if (val - 0.5).abs() < 0.01 {
        "1/2"
    } else if (val - 1.0).abs() < 0.01 {
        "1"
    } else if (val - 2.0).abs() < 0.01 {
        "2"
    } else if (val - 4.0).abs() < 0.01 {
        "4"
    } else {
        "1"
    }
}

fn draw_input_stage(ui: &mut Ui, input: &mut InputStage, automation: &mut AutomationState) -> bool {
    let mut modified = false;

    ui.label("Source A:");
    modified |= source_combo(ui, "source_a", &mut input.source_a);

    ui.label("Source B:");
    modified |= source_combo(ui, "source_b", &mut input.source_b);

    ui.add_space(8.0);

    modified |= param_slider_with_lfo(ui, "Mix A/B:", "input.mix", &mut input.mix, 0.0..=1.0, automation);
    modified |= param_slider_with_lfo(ui, "Frequency:", "input.frequency", &mut input.frequency, 0.5..=20.0, automation);
    modified |= param_slider_with_lfo(ui, "Phase:", "input.phase", &mut input.phase, 0.0..=1.0, automation);
    modified |= param_slider_with_lfo(ui, "Rotation:", "input.rotation", &mut input.rotation, 0.0..=1.0, automation);

    modified
}

fn source_combo(ui: &mut Ui, id: &str, source: &mut InputSource) -> bool {
    let mut changed = false;
    egui::ComboBox::from_id_salt(id)
        .selected_text(format!("{:?}", source))
        .show_ui(ui, |ui| {
            let sources = [
                InputSource::RampH,
                InputSource::RampV,
                InputSource::OscH,
                InputSource::OscV,
                InputSource::NoiseWhite,
                InputSource::NoisePink,
                InputSource::NoiseBrown,
                InputSource::ShapeCircle,
                InputSource::ShapeRect,
                InputSource::ShapeDiamond,
                InputSource::Checkerboard,
            ];
            for s in sources {
                if ui.selectable_label(*source == s, format!("{:?}", s)).clicked() {
                    *source = s;
                    changed = true;
                }
            }
        });
    changed
}

fn draw_geometry_stage(ui: &mut Ui, geo: &mut GeometryStage, automation: &mut AutomationState) -> bool {
    let mut modified = false;

    modified |= param_slider_with_lfo(ui, "Wobble H:", "geometry.wobbulate_h", &mut geo.wobbulate_h, 0.0..=1.0, automation);
    modified |= param_slider_with_lfo(ui, "Wobble V:", "geometry.wobbulate_v", &mut geo.wobbulate_v, 0.0..=1.0, automation);
    modified |= param_slider_with_lfo(ui, "Wobble Freq:", "geometry.wobble_freq", &mut geo.wobble_freq, 1.0..=20.0, automation);
    modified |= param_slider_with_lfo(ui, "Z Displace:", "geometry.z_displacement", &mut geo.z_displacement, 0.0..=0.5, automation);

    ui.add_space(4.0);

    modified |= param_slider_with_lfo(ui, "Lissajous X:", "geometry.lissajous_x", &mut geo.lissajous_x, 0.0..=1.0, automation);
    modified |= param_slider_with_lfo(ui, "Lissajous Y:", "geometry.lissajous_y", &mut geo.lissajous_y, 0.0..=1.0, automation);
    modified |= param_slider_with_lfo(ui, "Rotation:", "geometry.rotation", &mut geo.rotation, 0.0..=1.0, automation);
    modified |= param_slider_with_lfo(ui, "Scale:", "geometry.scale", &mut geo.scale, 0.5..=2.0, automation);

    modified
}

fn draw_amplitude_stage(ui: &mut Ui, amp: &mut AmplitudeStage, automation: &mut AutomationState) -> bool {
    let mut modified = false;

    modified |= param_slider_with_lfo(ui, "Fold Gain:", "amplitude.fold_gain", &mut amp.fold_gain, 1.0..=8.0, automation);
    modified |= param_slider_with_lfo(ui, "Fold Mix:", "amplitude.fold_mix", &mut amp.fold_mix, 0.0..=1.0, automation);

    ui.add_space(4.0);

    modified |= param_slider_with_lfo(ui, "Quantize:", "amplitude.quantize_levels", &mut amp.quantize_levels, 2.0..=32.0, automation);
    modified |= param_slider_with_lfo(ui, "Quant Mix:", "amplitude.quantize_mix", &mut amp.quantize_mix, 0.0..=1.0, automation);

    ui.add_space(4.0);

    modified |= param_slider_with_lfo(ui, "Soft Clip:", "amplitude.soft_clip", &mut amp.soft_clip, 0.0..=1.0, automation);
    modified |= param_slider_with_lfo(ui, "Solarize:", "amplitude.solarize", &mut amp.solarize, 0.0..=1.0, automation);
    modified |= param_slider_with_lfo(ui, "Gate:", "amplitude.gate_threshold", &mut amp.gate_threshold, 0.0..=1.0, automation);

    let mut invert_bool = amp.invert > 0.5;
    if ui.checkbox(&mut invert_bool, "Invert").changed() {
        amp.invert = if invert_bool { 1.0 } else { 0.0 };
        modified = true;
    }

    modified
}

fn draw_colorize_stage(ui: &mut Ui, color: &mut ColorizeStage, automation: &mut AutomationState) -> bool {
    let mut modified = false;

    ui.label("Mode:");
    egui::ComboBox::from_id_salt("color_mode")
        .selected_text(format!("{:?}", color.mode))
        .show_ui(ui, |ui| {
            for mode in [ColorMode::Spectrum, ColorMode::Threshold, ColorMode::Gradient, ColorMode::Monochrome] {
                if ui.selectable_label(color.mode == mode, format!("{:?}", mode)).clicked() {
                    color.mode = mode;
                    modified = true;
                }
            }
        });

    ui.add_space(4.0);

    modified |= param_slider_with_lfo(ui, "Hue Offset:", "colorize.hue_offset", &mut color.hue_offset, 0.0..=1.0, automation);
    modified |= param_slider_with_lfo(ui, "Saturation:", "colorize.saturation", &mut color.saturation, 0.0..=2.0, automation);
    modified |= param_slider_with_lfo(ui, "Levels:", "colorize.levels", &mut color.levels, 2.0..=32.0, automation);

    // Gradient colors (only show when gradient mode)
    if color.mode == ColorMode::Gradient {
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.label("Start:");
            let mut start_color = Color32::from_rgb(
                (color.gradient_start[0] * 255.0) as u8,
                (color.gradient_start[1] * 255.0) as u8,
                (color.gradient_start[2] * 255.0) as u8,
            );
            if ui.color_edit_button_srgba(&mut start_color).changed() {
                color.gradient_start = [
                    start_color.r() as f32 / 255.0,
                    start_color.g() as f32 / 255.0,
                    start_color.b() as f32 / 255.0,
                ];
                modified = true;
            }

            ui.label("End:");
            let mut end_color = Color32::from_rgb(
                (color.gradient_end[0] * 255.0) as u8,
                (color.gradient_end[1] * 255.0) as u8,
                (color.gradient_end[2] * 255.0) as u8,
            );
            if ui.color_edit_button_srgba(&mut end_color).changed() {
                color.gradient_end = [
                    end_color.r() as f32 / 255.0,
                    end_color.g() as f32 / 255.0,
                    end_color.b() as f32 / 255.0,
                ];
                modified = true;
            }
        });
    }

    modified
}

fn draw_feedback_stage(ui: &mut Ui, fb: &mut FeedbackStage, mixer: &mut MixerStage, automation: &mut AutomationState) -> bool {
    let mut modified = false;

    modified |= ui.checkbox(&mut fb.enabled, "Enable Feedback").changed();

    if fb.enabled {
        ui.add_space(4.0);

        // Mix controls (from mixer stage)
        modified |= param_slider_with_lfo(ui, "FB Mix:", "mixer.feedback_mix", &mut mixer.feedback_mix, 0.0..=1.0, automation);

        ui.label("Blend Mode:");
        egui::ComboBox::from_id_salt("blend_mode")
            .selected_text(format!("{:?}", mixer.blend_mode))
            .show_ui(ui, |ui| {
                for mode in [BlendMode::Mix, BlendMode::Add, BlendMode::Multiply, BlendMode::Screen, BlendMode::Overlay, BlendMode::Difference, BlendMode::LumaKeyA, BlendMode::LumaKeyB] {
                    if ui.selectable_label(mixer.blend_mode == mode, format!("{:?}", mode)).clicked() {
                        mixer.blend_mode = mode;
                        modified = true;
                    }
                }
            });

        modified |= param_slider_with_lfo(ui, "Opacity:", "mixer.layer_opacity", &mut mixer.layer_opacity, 0.0..=1.0, automation);

        ui.add_space(4.0);
        ui.separator();
        ui.label("Luma Key:");

        modified |= param_slider_with_lfo(ui, "Threshold:", "mixer.key_threshold", &mut mixer.key_threshold, 0.0..=1.0, automation);
        modified |= param_slider_with_lfo(ui, "Softness:", "mixer.key_softness", &mut mixer.key_softness, 0.0..=0.5, automation);
        modified |= ui.checkbox(&mut mixer.key_invert, "Invert Key").changed();

        ui.add_space(4.0);
        ui.separator();
        ui.label("Transform:");

        modified |= param_slider_with_lfo(ui, "Zoom:", "feedback.zoom", &mut fb.zoom, 0.9..=1.1, automation);
        modified |= param_slider_with_lfo(ui, "Rotation:", "feedback.rotation", &mut fb.rotation, -0.1..=0.1, automation);
        modified |= param_slider_with_lfo(ui, "Offset X:", "feedback.offset_x", &mut fb.offset_x, -0.1..=0.1, automation);
        modified |= param_slider_with_lfo(ui, "Offset Y:", "feedback.offset_y", &mut fb.offset_y, -0.1..=0.1, automation);

        ui.add_space(4.0);
        ui.separator();
        ui.label("Color:");

        modified |= param_slider_with_lfo(ui, "Hue Shift:", "feedback.hue_shift", &mut fb.hue_shift, 0.0..=0.1, automation);
        modified |= param_slider_with_lfo(ui, "Decay:", "feedback.decay", &mut fb.decay, 0.8..=1.0, automation);
        modified |= param_slider_with_lfo(ui, "Saturation:", "feedback.saturation", &mut fb.saturation, 0.0..=2.0, automation);
    }

    modified
}

fn draw_output_stage(ui: &mut Ui, out: &mut OutputStage, automation: &mut AutomationState) -> bool {
    let mut modified = false;

    // Effect chain: VHS -> Cable -> CRT (toggleable)
    ui.horizontal(|ui| {
        modified |= ui.checkbox(&mut out.vhs_enabled, "VHS").changed();
        modified |= ui.checkbox(&mut out.cable_enabled, "Cable").changed();
        modified |= ui.checkbox(&mut out.crt_enabled, "CRT").changed();
    });

    ui.separator();

    // VHS controls
    if out.vhs_enabled {
        ui.collapsing("VHS", |ui| {
            modified |= param_slider_with_lfo(ui, "Tracking:", "output.tracking", &mut out.tracking, 0.0..=1.0, automation);
            modified |= param_slider_with_lfo(ui, "Chroma:", "output.chroma_shift", &mut out.chroma_shift, 0.0..=0.02, automation);
            modified |= param_slider_with_lfo(ui, "Wobble:", "output.tape_wobble", &mut out.tape_wobble, 0.0..=1.0, automation);
            modified |= param_slider_with_lfo(ui, "Noise:", "output.vhs_noise", &mut out.vhs_noise, 0.0..=0.5, automation);
        });
    }

    // Cable controls
    if out.cable_enabled {
        ui.collapsing("Cable", |ui| {
            modified |= param_slider_with_lfo(ui, "Bandwidth:", "output.bandwidth", &mut out.bandwidth, 0.5..=1.0, automation);
            modified |= param_slider_with_lfo(ui, "Ghosting:", "output.ghosting", &mut out.ghosting, 0.0..=0.3, automation);
            modified |= param_slider_with_lfo(ui, "Noise:", "output.cable_noise", &mut out.cable_noise, 0.0..=0.2, automation);
        });
    }

    // CRT controls
    if out.crt_enabled {
        ui.collapsing("CRT", |ui| {
            modified |= param_slider_with_lfo(ui, "Scanlines:", "output.scanlines", &mut out.scanlines, 0.0..=0.5, automation);
            modified |= param_slider_with_lfo(ui, "Bloom:", "output.bloom", &mut out.bloom, 0.0..=1.0, automation);
            modified |= param_slider_with_lfo(ui, "Vignette:", "output.vignette", &mut out.vignette, 0.0..=1.0, automation);
        });
    }

    modified
}
