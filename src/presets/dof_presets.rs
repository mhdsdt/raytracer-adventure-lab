#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DofPreset {
    Off,
    Subtle,
    Strong,
}

impl DofPreset {
    pub fn name(self) -> &'static str {
        match self {
            DofPreset::Off => "Off",
            DofPreset::Subtle => "Subtle",
            DofPreset::Strong => "Strong",
        }
    }

    /// Returns the aperture radius for this DOF preset.
    pub fn aperture_radius(self) -> f32 {
        match self {
            DofPreset::Off => 0.0,
            DofPreset::Subtle => 0.05,
            DofPreset::Strong => 0.2,
        }
    }

    pub fn next(self) -> DofPreset {
        match self {
            DofPreset::Off => DofPreset::Subtle,
            DofPreset::Subtle => DofPreset::Strong,
            DofPreset::Strong => DofPreset::Off,
        }
    }
}
