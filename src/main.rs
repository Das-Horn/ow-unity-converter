#![feature(allocator_api)]
use ow_unity_converter::ImgConv;
use startup::CDStartup;
use std::env::args;
use nfd::*;

mod startup;

fn main() {
    let input_args: Vec<String> = args().collect();

    let cli_text = CDStartup::new(
        String::from("Overwatch to Unity converter"),
        String::from("A simple program to convert Overwatch textures"),
    );

    cli_text.print_opening_message();

    if input_args[1] == "-s" {
        from_dialog();
    } else {
        from_path(input_args);
    }
    return;
}

fn from_path(input_args: Vec<String>) {
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

fn from_dialog() {
    let file_dialog = DialogBuilder::new(DialogType::MultipleFiles);
    let result = file_dialog.open().expect("Erorr collecting Data.");
    let mut result_string: Vec<String> = Vec::new();
    //processing

    match result {
        Response::Okay(result) => result_string.insert(0, result),
        Response::OkayMultiple(mut result) => result_string.append(&mut result),
        Response::Cancel => return,
    }

    for i in 0..result_string.len() {
        let mut conv = ImgConv::new(Some(String::from("conv")));

        conv.read_image(&result_string[i]);
        conv.from_owto_unity();
    }
}