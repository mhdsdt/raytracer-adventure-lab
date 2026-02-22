/// Display buffer holding u32 pixels in 0xRRGGBB format for the window.
pub struct ImageBuffer {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u32>,
}

impl ImageBuffer {
    pub fn new(width: u32, height: u32) -> Self {
        ImageBuffer {
            width,
            height,
            pixels: vec![0; (width * height) as usize],
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, pixel: u32) {
        let idx = (y * self.width + x) as usize;
        self.pixels[idx] = pixel;
    }
}
