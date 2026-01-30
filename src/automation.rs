//! Automation system with LFO modulation
//!
//! Ported from main Phosphlux, simplified for Lite's fixed pipeline.

use crate::synth::SynthState;
use instant::Instant;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// LFO state for a single parameter
#[derive(Clone, Serialize, Deserialize)]
pub struct LfoState {
    /// Speed multiplier (0.1 = slow, 0.25 = medium, 0.5 = fast)
    pub speed: f32,
    /// Lower bound of oscillation
    pub lo: f32,
    /// Upper bound of oscillation
    pub hi: f32,
    /// Phase offset (0.0-1.0)
    pub offset: f32,
    /// Tempo subdivision (0.25, 0.5, 1.0, 2.0, 4.0)
    pub subdivide: f32,
}

impl Default for LfoState {
    fn default() -> Self {
        Self {
            speed: 0.25,
            lo: 0.0,
            hi: 1.0,
            offset: 0.0,
            subdivide: 1.0,
        }
    }
}

impl LfoState {
    /// Create slow LFO (0.1x BPM)
    pub fn slow(min: f32, max: f32) -> Self {
        Self {
            speed: 0.1,
            lo: min,
            hi: max,
            offset: 0.0,
            subdivide: 1.0,
        }
    }

    /// Create medium LFO (0.25x BPM)
    pub fn medium(min: f32, max: f32) -> Self {
        Self {
            speed: 0.25,
            lo: min,
            hi: max,
            offset: 0.0,
            subdivide: 1.0,
        }
    }

    /// Create fast LFO (0.5x BPM)
    pub fn fast(min: f32, max: f32) -> Self {
        Self {
            speed: 0.5,
            lo: min,
            hi: max,
            offset: 0.0,
            subdivide: 1.0,
        }
    }

    /// Compute LFO value at given time
    pub fn compute(&self, time_secs: f32, bpm_hz: f32) -> f32 {
        let center = (self.lo + self.hi) / 2.0;
        let range = (self.hi - self.lo) / 2.0;
        let effective_hz = bpm_hz * self.speed * self.subdivide;
        let phase = time_secs * effective_hz + self.offset;
        let val = center + range * (phase * std::f32::consts::TAU).sin();
        val.clamp(self.lo, self.hi)
    }
}

/// Global automation state
pub struct AutomationState {
    /// Active LFOs keyed by "stage.param" (e.g., "geometry.wobbulate_h")
    pub lfos: HashMap<String, LfoState>,
    /// Global tempo in BPM
    pub global_bpm: f32,
    /// Start time for LFO phase calculation
    start_time: Instant,
}

impl Default for AutomationState {
    fn default() -> Self {
        Self {
            lfos: HashMap::new(),
            global_bpm: 120.0,
            start_time: Instant::now(),
        }
    }
}

impl AutomationState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Get elapsed time since automation started
    pub fn lfo_time(&self) -> f32 {
        self.start_time.elapsed().as_secs_f32()
    }

    /// Apply all active LFOs to synth state
    /// Returns true if any parameters were modified
    pub fn apply(&self, synth: &mut SynthState) -> bool {
        if self.lfos.is_empty() {
            return false;
        }

        let bpm_hz = self.global_bpm / 60.0;
        let time = self.lfo_time();
        let mut modified = false;

        for (key, lfo) in &self.lfos {
            let val = lfo.compute(time, bpm_hz);
            if self.set_param(synth, key, val) {
                modified = true;
            }
        }

        modified
    }

    /// Set a parameter value by key
    fn set_param(&self, synth: &mut SynthState, key: &str, val: f32) -> bool {
        let parts: Vec<&str> = key.split('.').collect();
        if parts.len() != 2 {
            return false;
        }

        let (stage, param) = (parts[0], parts[1]);

        match stage {
            "input" => match param {
                "mix" => synth.input.mix = val,
                "frequency" => synth.input.frequency = val,
                "phase" => synth.input.phase = val,
                "rotation" => synth.input.rotation = val,
                _ => return false,
            },
            "geometry" => match param {
                "wobbulate_h" => synth.geometry.wobbulate_h = val,
                "wobbulate_v" => synth.geometry.wobbulate_v = val,
                "wobble_freq" => synth.geometry.wobble_freq = val,
                "z_displacement" => synth.geometry.z_displacement = val,
                "lissajous_x" => synth.geometry.lissajous_x = val,
                "lissajous_y" => synth.geometry.lissajous_y = val,
                "rotation" => synth.geometry.rotation = val,
                "scale" => synth.geometry.scale = val,
                _ => return false,
            },
            "amplitude" => match param {
                "fold_gain" => synth.amplitude.fold_gain = val,
                "fold_mix" => synth.amplitude.fold_mix = val,
                "quantize_levels" => synth.amplitude.quantize_levels = val,
                "quantize_mix" => synth.amplitude.quantize_mix = val,
                "soft_clip" => synth.amplitude.soft_clip = val,
                "solarize" => synth.amplitude.solarize = val,
                "gate_threshold" => synth.amplitude.gate_threshold = val,
                _ => return false,
            },
            "colorize" => match param {
                "hue_offset" => synth.colorize.hue_offset = val,
                "saturation" => synth.colorize.saturation = val,
                "levels" => synth.colorize.levels = val,
                _ => return false,
            },
            "mixer" => match param {
                "feedback_mix" => synth.mixer.feedback_mix = val,
                "key_threshold" => synth.mixer.key_threshold = val,
                "key_softness" => synth.mixer.key_softness = val,
                "layer_opacity" => synth.mixer.layer_opacity = val,
                _ => return false,
            },
            "feedback" => match param {
                "zoom" => synth.feedback.zoom = val,
                "rotation" => synth.feedback.rotation = val,
                "hue_shift" => synth.feedback.hue_shift = val,
                "decay" => synth.feedback.decay = val,
                "offset_x" => synth.feedback.offset_x = val,
                "offset_y" => synth.feedback.offset_y = val,
                "saturation" => synth.feedback.saturation = val,
                _ => return false,
            },
            "output" => match param {
                "scanlines" => synth.output.scanlines = val,
                "bloom" => synth.output.bloom = val,
                "vignette" => synth.output.vignette = val,
                "tracking" => synth.output.tracking = val,
                "chroma_shift" => synth.output.chroma_shift = val,
                "tape_wobble" => synth.output.tape_wobble = val,
                "vhs_noise" => synth.output.vhs_noise = val,
                "bandwidth" => synth.output.bandwidth = val,
                "ghosting" => synth.output.ghosting = val,
                "cable_noise" => synth.output.cable_noise = val,
                _ => return false,
            },
            _ => return false,
        }
        true
    }

    /// Cycle LFO state: Off -> Slow -> Medium -> Fast -> Off
    pub fn cycle_lfo(&mut self, key: &str, min: f32, max: f32) {
        if let Some(lfo) = self.lfos.get(key) {
            let next_state = if lfo.speed < 0.15 {
                // Currently slow -> medium
                Some(LfoState::medium(min, max))
            } else if lfo.speed < 0.4 {
                // Currently medium -> fast
                Some(LfoState::fast(min, max))
            } else {
                // Currently fast -> off
                None
            };

            if let Some(new_lfo) = next_state {
                self.lfos.insert(key.to_string(), new_lfo);
            } else {
                self.lfos.remove(key);
            }
        } else {
            // Currently off -> slow
            self.lfos.insert(key.to_string(), LfoState::slow(min, max));
        }
    }

    /// Remove LFO from a parameter
    pub fn remove_lfo(&mut self, key: &str) {
        self.lfos.remove(key);
    }

    /// Check if LFO is active for a parameter
    pub fn has_lfo(&self, key: &str) -> bool {
        self.lfos.contains_key(key)
    }

    /// Get LFO state for a parameter
    pub fn get_lfo(&self, key: &str) -> Option<&LfoState> {
        self.lfos.get(key)
    }

    /// Get mutable LFO state for a parameter
    pub fn get_lfo_mut(&mut self, key: &str) -> Option<&mut LfoState> {
        self.lfos.get_mut(key)
    }
}
