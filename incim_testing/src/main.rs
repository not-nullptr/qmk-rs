use include_image_structs::Chunk;

fn main() {
    let input_path = "rust_logo.png";
    let img = match image::open(&input_path) {
        Ok(img) => img.to_luma8(),
        Err(e) => panic!("failed to open image {}: {}", input_path, e),
    };

    let width = img.width() as usize;
    let height = img.height() as usize;

    let mut chunks = vec![];
    let chunks_x = (width as f64 / 8.0).ceil() as u8;
    let chunks_y = (height as f64 / 8.0).ceil() as u8;

    for chunk_x in 0..chunks_x {
        for chunk_y in 0..chunks_y {
            let mut bytes: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
            let start_x = (chunk_x as u32) * 8;
            let end_x = ((chunk_x as u32) + 1) * 8;
            let end_x = end_x.min(width as u32);
            let start_y = (chunk_y as u32) * 8;
            let end_y = ((chunk_y as u32) + 1) * 8;
            let end_y = end_y.min(height as u32);
            for x in start_x..end_x {
                let rel_x = (x - start_x) as usize;
                let mut byte: u8 = 0;
                for y in start_y..end_y {
                    let rel_y = (y - start_y) as usize;
                    if rel_y < 8 {
                        if let Some(pixel_val) = img.get_pixel_checked(x, y) {
                            if pixel_val.0[0] > 127 {
                                byte |= 1 << rel_y;
                            }
                        }
                    }
                }
                bytes[rel_x] = byte.reverse_bits();
            }
            chunks.push(Chunk {
                x: chunk_x,
                y: chunk_y,
                bytes,
            })
        }
    }

    println!("{:?}", chunks);
}
