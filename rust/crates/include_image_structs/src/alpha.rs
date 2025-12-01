use crate::QmkImage;

#[derive(Debug, Copy, Clone)]
pub struct QmkImageAlpha<const N: usize> {
    pub width: u8,
    pub height: u8,
    pub bytes: [u8; N],
    pub alpha: [u8; N],
}

impl<const N: usize> QmkImage for QmkImageAlpha<N> {
    fn get_pixel(&self, x: usize, y: usize) -> Option<bool> {
        if x >= self.width as usize || y >= self.height as usize {
            return None;
        }

        let byte_index = (y / 8) * self.width as usize + x;
        let bit_index = y % 8;
        Some((self.bytes[byte_index] >> bit_index) & 1 == 1)
    }

    fn get_alpha(&self, x: usize, y: usize) -> Option<bool> {
        if x >= self.width as usize || y >= self.height as usize {
            return None;
        }

        let byte_index = (y / 8) * self.width as usize + x;
        let bit_index = y % 8;
        Some((self.alpha[byte_index] >> bit_index) & 1 == 1)
    }

    fn width(&self) -> u8 {
        self.width
    }

    fn height(&self) -> u8 {
        self.height
    }

    fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    fn as_bytes_alpha(&self) -> Option<&[u8]> {
        Some(&self.alpha)
    }
}
