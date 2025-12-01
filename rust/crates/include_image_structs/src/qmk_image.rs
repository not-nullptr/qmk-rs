pub trait QmkImage {
    /// Get the pixel value at (x, y).
    ///
    /// ## Returns
    /// - `Some(true)` if the pixel is set (on).
    /// - `Some(false)` if the pixel is not set (off).
    /// - `None` if (x, y) is out of bounds.
    fn get_pixel(&self, x: usize, y: usize) -> Option<bool>;

    /// Get the alpha value at (x, y).
    ///
    /// ## Returns
    /// - `Some(true)` if the pixel is opaque.
    /// - `Some(false)` if the pixel is transparent.
    /// - `None` if (x, y) is out of bounds.
    fn get_alpha(&self, x: usize, y: usize) -> Option<bool>;

    /// Get the width of the image.
    ///
    //// ## Returns
    /// The width of the image in pixels.
    fn width(&self) -> u8;

    /// Get the height of the image.
    ///
    /// ## Returns
    /// The height of the image in pixels.
    fn height(&self) -> u8;

    /// Get the raw byte data of the image.
    ///
    /// ## Returns
    /// A reference to the byte array representing the image data.
    fn as_bytes(&self) -> &[u8];

    /// Get the raw alpha byte data of the image, if applicable.
    ///
    /// ## Returns
    /// A reference to the byte array representing the alpha data, or `None` if not applicable.
    fn as_bytes_alpha(&self) -> Option<&[u8]>;
}
