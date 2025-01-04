use include_image::include_animation;
use include_image_structs::QmkImage;

include_animation!("rust_logo");

fn main() {
    println!("{:?}", RUST_LOGO);
}
