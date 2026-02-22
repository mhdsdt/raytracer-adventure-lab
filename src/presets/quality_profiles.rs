use crate::core::renderer::RenderProfile;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QualityPreset {
    Draft,
    Preview,
    Quality,
}

impl QualityPreset {
    pub fn name(self) -> &'static str {
        match self {
            QualityPreset::Draft => "Draft",
            QualityPreset::Preview => "Preview",
            QualityPreset::Quality => "Quality",
        }
    }

    pub fn to_render_profile(self) -> RenderProfile {
        match self {
            QualityPreset::Draft => RenderProfile { max_bounces: 4 },
            QualityPreset::Preview => RenderProfile { max_bounces: 8 },
            QualityPreset::Quality => RenderProfile { max_bounces: 16 },
        }
    }

    #[allow(dead_code)]
    pub fn all() -> &'static [QualityPreset] {
        &[QualityPreset::Draft, QualityPreset::Preview, QualityPreset::Quality]
    }

    pub fn next(self) -> QualityPreset {
        match self {
            QualityPreset::Draft => QualityPreset::Preview,
            QualityPreset::Preview => QualityPreset::Quality,
            QualityPreset::Quality => QualityPreset::Draft,
        }
    }
}
