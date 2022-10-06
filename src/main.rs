#![feature(allocator_api)]
use ow_unity_converter::ImgConv;
use startup::CDStartup;
use std::env::args;

mod startup;

fn main() {
    let input_args: Vec<String> = args().collect();

    let cli_text = CDStartup::new(
        String::from("Overwatch to Unity converter"),
        String::from("A simple program to convert Overwatch textures"),
    );

    cli_text.print_opening_message();

    if input_args.len() <= 1 {
        println!("Error no path supplied.");
        return;
    }

    for i in 1..input_args.len() {
        let mut conv = ImgConv::new(Some(String::from("conv")));

        conv.read_image(&input_args[i]);
        conv.from_owto_unity();
    }
}
