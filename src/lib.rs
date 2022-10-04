#![feature(allocator_api)]

use std::{alloc::Global};

use image::{DynamicImage, GenericImageView, ImageBuffer};

use progress::Bar;

pub struct ImgConv {
    input_image: Option<DynamicImage>,
    // convType: String,
    _image_read: bool,
    path: String,
    bar: Bar,
}

 impl ImgConv { 

    pub fn new() -> ImgConv {
        let tmp_type = ImgConv {
            input_image: None,
            _image_read: false,
            path: String::from(""),
            bar: Bar::new()
        };
        return tmp_type;
    }

    pub fn read_image(&mut self, path: &str) {
        self.bar.set_job_title("Reading Image...");

        self.input_image = Some(image::open(&path)
                                .expect("Error Loading the image."));

        self.path = String::from(path);
        self._image_read = true;

        self.bar.reach_percent(10);
    }

    pub fn from_owto_unity(&mut self){
        self.bar.set_job_title("Converting...");

        if !self._image_read {
            println!("Please read an image first.");
        }

        let mut tmp_colour;
        let mut proc_colour: image::Rgba<u8> = image::Rgba([0,0,0,0]);
        let tmp_image = self.input_image.as_ref()
                                                    .expect("Error loading image to memory");
        let (w, h) = tmp_image.dimensions();
        let mut scale: f64 = 0.;
        let mut image_process = ImageBuffer::new(w, h);
        // let total = w * h;
        // let mut counter = 0;

        for x in 0..w {
            for y in 0..h {
                tmp_colour = tmp_image.get_pixel(x, y);
                // Roughness Channel : Green -> Alpha
                proc_colour[3] = tmp_colour[1];
                // AO Default Value 255
                proc_colour[1] = 255;
                // Metallic Calc
                scale = (proc_colour[0] / 255) as f64;

                if scale < 0.5 {
                    scale = 0.;
                }

                proc_colour[0] = num::clamp(((((scale - 0.5) * scale) * 2.) * 255.) as u8, 0, 1);

                image_process.put_pixel(x, y, proc_colour);
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

        process_string.insert(process_string.len() - 4, '-');

        img.save(process_string)
            .expect("Error Saving the image");
    }
 }
