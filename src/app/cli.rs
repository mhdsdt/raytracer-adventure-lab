use clap::Parser;

/// Phase 1 Ray Tracer — Progressive Preview App
#[derive(Parser, Debug)]
#[command(name = "raytracer-adventure-lab")]
#[command(about = "Phase 1 progressive ray tracing preview")]
pub struct CliArgs {
    /// Scene to render (default: materials_lighting)
    #[arg(short, long, default_value = "materials_lighting")]
    pub scene: String,

    /// Enable deterministic mode for repeatable results
    #[arg(short, long, default_value_t = false)]
    pub deterministic: bool,

    /// Initial quality profile: draft, preview, quality
    #[arg(short, long, default_value = "preview")]
    pub quality: String,

    /// Use development resolution (960x540) instead of 1280x720
    #[arg(long, default_value_t = false)]
    pub dev_resolution: bool,
}

/// Validated launch configuration derived from CLI args.
pub struct LaunchConfig {
    pub scene_id: String,
    pub deterministic: bool,
    pub initial_quality: crate::presets::quality_profiles::QualityPreset,
    pub width: u32,
    pub height: u32,
}

impl LaunchConfig {
    pub fn from_args(args: CliArgs) -> anyhow::Result<Self> {
        use crate::presets::quality_profiles::QualityPreset;
        use crate::scenes::catalog;

        // Validate scene
        let scene_ids = catalog::scene_ids();
        if !scene_ids.contains(&args.scene.as_str()) {
            anyhow::bail!(
                "Unknown scene '{}'. Available scenes: {}",
                args.scene,
                scene_ids.join(", ")
            );
        }

        // Parse quality preset
        let initial_quality = match args.quality.to_lowercase().as_str() {
            "draft" => QualityPreset::Draft,
            "preview" => QualityPreset::Preview,
            "quality" => QualityPreset::Quality,
            other => anyhow::bail!(
                "Unknown quality '{}'. Options: draft, preview, quality",
                other
            ),
        };

        let (width, height) = if args.dev_resolution {
            (960, 540)
        } else {
            (1280, 720)
        };

        Ok(LaunchConfig {
            scene_id: args.scene,
            deterministic: args.deterministic,
            initial_quality,
            width,
            height,
        })
    }

    pub fn print_startup_summary(&self) {
        println!("=== Ray Tracer Phase 1 ===");
        println!("Scene:         {}", self.scene_id);
        println!("Resolution:    {}x{}", self.width, self.height);
        println!("Quality:       {}", self.initial_quality.name());
        println!("Deterministic: {}", if self.deterministic { "ON" } else { "OFF" });
        println!("========================");
    }
}
