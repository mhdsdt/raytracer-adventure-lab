use super::math::Color;

/// Progressive accumulation buffer. Accumulates linear color values
/// and tracks sample count per pixel.
pub struct AccumulationBuffer {
    pub width: u32,
    pub height: u32,
    /// Sum of all color samples per pixel (linear color space).
    data: Vec<Color>,
    /// Number of samples accumulated (global — same for all pixels in our approach).
    pub sample_count: u32,
}

impl AccumulationBuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        AccumulationBuffer {
            width,
            height,
            data: vec![Color::BLACK; size],
            sample_count: 0,
        }
    }

    /// Reset all accumulated data.
    pub fn clear(&mut self) {
        for pixel in &mut self.data {
            *pixel = Color::BLACK;
        }
        self.sample_count = 0;
    }

    /// Add a color sample to a specific pixel.
    pub fn add_sample(&mut self, x: u32, y: u32, color: Color) {
        let idx = (y * self.width + x) as usize;
        self.data[idx] += color;
    }

    /// Mark that one full sample pass has been accumulated.
    pub fn increment_sample_count(&mut self) {
        self.sample_count += 1;
    }

    /// Get the averaged color for a pixel, suitable for display conversion.
    pub fn get_averaged_color(&self, x: u32, y: u32) -> Color {
        if self.sample_count == 0 {
            return Color::BLACK;
        }
        let idx = (y * self.width + x) as usize;
        self.data[idx] / self.sample_count as f32
    }
}
