#![feature(allocator_api)]

use std::{alloc::Global};

use image::{DynamicImage, GenericImageView, ImageBuffer};

use progress::Bar;

pub struct ImgConv {
    finalImage: Option<DynamicImage>,
    inputImage: Option<DynamicImage>,
    // convType: String,
    _imageRead: bool,
    path: String,
    bar: Bar,
}

 impl ImgConv { 

    pub fn new() -> ImgConv {
        let tmp_type = ImgConv {
            finalImage: None,
            inputImage: None,
            _imageRead: false,
            path: String::from(""),
            bar: Bar::new()
        };
        return tmp_type;
    }

    pub fn read_image(&mut self, path: &str) {
        self.bar.set_job_title("Reading Image...");

        self.inputImage = Some(image::open(&path)
                                .expect("Error Loading the image."));

        self.path = String::from(path);
        self._imageRead = true;

        self.bar.reach_percent(10);
    }

    pub fn from_owto_unity(&mut self){
        self.bar.set_job_title("Converting...");

        if !self._imageRead {
            println!("Please read an image first.");
        }

        let mut tmp_colour;
        let tmp_image = self.inputImage.as_ref()
                                                    .expect("Error loading image to memory");
        let (w, h) = tmp_image.dimensions();
        let mut image_process = ImageBuffer::new(w, h);
        let total = w * h;
        let mut counter = 0;

        for x in 0..w {
            for y in 0..h {
                tmp_colour = tmp_image.get_pixel(x, y);
                image_process.put_pixel(x, y, tmp_colour);
                // println!("adding percent : {}", counter as i32);
                counter += 1;
            }
        }
        self.bar.reach_percent(90);
        self.bar.set_job_title("Saving Image ...");
        self.save_image(&image_process);
        self.bar.reach_percent(100);
        self.bar.jobs_done();

    }

    fn save_image(&self,img: &ImageBuffer<image::Rgba<u8>, Vec<u8, Global>>) {
        let mut process_string = String::from(&self.path);

        process_string.insert((
            process_string.len() - 4
        ), '-');

        img.save(process_string)
            .expect("Error Saving the image");
    }
 }
