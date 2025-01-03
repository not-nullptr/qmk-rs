use include_image::include_image;

include_image!("rust_logo.png");

fn main() {
    println!("{:?}", RUST_LOGO.bytes.len());
}
