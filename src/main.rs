#![feature(allocator_api)]
use ow_unity_converter::ImgConv;

mod lib;

fn main() {
    let mut conv = ImgConv::new();

    conv.read_image("/home/cd/Pictures/Wallpapers/Clearnight.jpg");
    conv.from_owto_unity();
}
