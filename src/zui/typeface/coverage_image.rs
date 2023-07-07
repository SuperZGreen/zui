/// A reimplementation of a greyscale, u8 coverage image to allow for faster copying than image-rs
/// allows
pub struct CoverageImage {
    /// The raw bytes of the image in along-row-first order
    pub coverage: Box<[u8]>,

    /// The width of the image in pixels
    pub width: u32,

    /// The height of the image in pixels
    pub height: u32,
}

impl CoverageImage {
    /// Creates a new image with 0 coverage for the provided dimensions
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            coverage: vec![0; width as usize * height as usize].into_boxed_slice(),
            width,
            height,
        }
    }

    /// Is true if width or height is zero
    fn is_zero(&self) -> bool {
        self.width == 0 || self.height == 0
    }

    // Copies from the target image onto the self image, using y-down coordinates. The destination
    // is represented by x, y, which gives the top-left coordinate of the destination
    pub fn copy_from(&mut self, other: &Self, x: u32, y: u32) -> Result<(), ()> {
        // TODO: bounds checking if out of bounds

        if other.is_zero() {
            return Ok(());
        }

        for (other_row_index, other_row) in other.coverage.chunks(other.width as usize).enumerate()
        {
            let self_y_index = other_row_index + y as usize;
            let self_x_index = x as usize;
            let start_index = self_y_index * self.width as usize + self_x_index;
            let end_index = start_index + other.width as usize;

            self.coverage[start_index..end_index].clone_from_slice(other_row);
        }

        Ok(())
    }
}
