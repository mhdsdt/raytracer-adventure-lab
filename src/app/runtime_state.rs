use crate::presets::dof_presets::DofPreset;
use crate::presets::quality_profiles::QualityPreset;

/// Mutable runtime preset state. Changes to any field require accumulation reset.
#[derive(Debug, Clone)]
pub struct RuntimeState {
    pub quality: QualityPreset,
    pub camera_preset_index: usize,
    pub dof: DofPreset,
}

impl RuntimeState {
    pub fn new(quality: QualityPreset) -> Self {
        RuntimeState {
            quality,
            camera_preset_index: 0,
            dof: DofPreset::Off,
        }
    }
}
