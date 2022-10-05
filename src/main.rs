#![feature(allocator_api)]
use ow_unity_converter::ImgConv;
use std::env::args;

fn main() {
    let input_args: Vec<String> = args().collect();

    if input_args.is_empty() {
        println!("Error no path supplied.");
        return;
    }

    let mut conv = ImgConv::new();

    conv.read_image(&input_args[1]);
    conv.from_owto_unity();
}
