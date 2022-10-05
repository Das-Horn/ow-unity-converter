#![feature(allocator_api)]
use ow_unity_converter::ImgConv;
use std::env::args;

fn main() {
    let input_args: Vec<String> = args().collect();

    if input_args.len() <= 1 {
        println!("Error no path supplied.");
        return;
    }

    for i in 1..input_args.len(){
        let mut conv = ImgConv::new();

        conv.read_image(&input_args[i]);
        conv.from_owto_unity();
    }
}
