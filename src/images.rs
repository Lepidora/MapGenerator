/*
 *  images.rs
 *  
 *  Provides images for the server to send to the client
 */

use std::fs::File;
use std::io::prelude::*;

extern crate image;

use image::png;
use noise::{NoiseFn, OpenSimplex};

struct Bounds {
	pub left: f64,
	pub right: f64,
	pub top: f64,
	pub bottom: f64,
	pub level: u32
}

pub fn get_image_header() -> Vec<u8> {

	let header = "HTTP/1.1 200 OK\r\nContent-type: image/png\r\nTransfer-Encoding: identity\r\n\r\n";

	return header.to_string().into_bytes();
}

pub fn get_default_image() -> Vec<u8> {
	
	let file_path = "default.png";

	let mut buffer = Vec::new();
	let mut file = File::open(&file_path).unwrap();

	file.read_to_end(&mut buffer).unwrap();

	let header = "HTTP/1.1 200 OK\r\nContent-type: image/png\r\nTransfer-Encoding: identity\r\n\r\n";

	let mut response = header.to_string().into_bytes();
	response.extend(buffer);

	return response;
}

pub fn image_for_coordinates(x: f64, y: f64, z: u32) -> Vec<u8> {
	
	return simplex_noise_tile(x, y, z);
}

fn simplex_noise_tile(x: f64, y: f64, z: u32) -> Vec<u8> {

	println!("Requested tile at x: {} y: {} z: {}", x, y, z);

	let width = 256;
	let height = 256;

	let base: i64 = 2;

	let scale_factor: f64 = base.pow(z) as f64;

	let tile_size = 1.0 / scale_factor;

	let scaled_x = tile_size * x;
	let scaled_y = tile_size * y;

	let noise = OpenSimplex::new();

	let mut image_buffer = image::ImageBuffer::new(width, height);

	for (x_pos, y_pos, pixel) in image_buffer.enumerate_pixels_mut() {
		
		let scaled_x_pos = x_pos as f64 / width as f64;
		let scaled_y_pos = y_pos as f64 / height as f64;

		let positioned_x = (scaled_x_pos * tile_size) + scaled_x;
		let positioned_y = (scaled_y_pos * tile_size) + scaled_y;

		let detail_scale = 50.0;

		let mut noise_val = 0.0;

		let mut intensity_scale = 0.0;

		let base_zoom = 1;

		for i in 0..z-base_zoom {
			
			/*let intensity = if i == 0 {
				1.0
			} else {
				1.0 / i as f64
			};*/

			let intensity = 1.0 / 2.0f64.powi(i as i32);

			noise_val += intensity * noise.get([positioned_x * (detail_scale / intensity), positioned_y * (detail_scale / intensity), 1 as f64]);

			intensity_scale += intensity;
		}
			
		//noise_val /= intensity_scale;

		/*let r = (256.0 * positioned_x) as u8;
		let b = ((z as f64 / 5.0) * 256.0) as u8;
		let g = (256.0 * positioned_y) as u8;*/

		let normalised_noise = (noise_val + 1.0) / 2.0;

		let (r, g, b) = color_for_height(normalised_noise);

		*pixel = image::Rgb([r, g, b]);
	}

	let mut png_data = Vec::new();

	let encoder = png::PNGEncoder::new(&mut png_data);
	
	let _result = encoder.encode(&image_buffer, image_buffer.width(), image_buffer.height(), image::RGB(8));

	return png_data;
}

fn color_for_height(height: f64) -> (u8, u8, u8) {

	if height < 0.5 {
		return (59 as u8, 131 as u8, 255 as u8);
	}

	if height > 0.9 {
		return (241 as u8, 241 as u8, 241 as u8);
	}

	let green_scale = (height - 0.5) / 0.4;

	let green_base = 0.7852;

	let color_scale = green_scale;

	((color_scale * 256.0) as u8, (green_base * 256.0) as u8, (color_scale * 256.0) as u8)
}

fn color_test_tile(x: f64, y: f64, z: u32) -> Vec<u8> {
	
	let width = 256;
	let height = 256;

	let base: i64 = 2;

	let scale_factor: f64 = base.pow(z) as f64;

	let tile_size = 1.0 / scale_factor;

	let scaled_x = tile_size * x;
	let scaled_y = tile_size * y;

	let mut image_buffer = image::ImageBuffer::new(width, height);

	for (x_pos, y_pos, pixel) in image_buffer.enumerate_pixels_mut() {
		
		let scaled_x_pos = x_pos as f64 / width as f64;
		let scaled_y_pos = y_pos as f64 / height as f64;

		let positioned_x = (scaled_x_pos * tile_size) + scaled_x;
		let positioned_y = (scaled_y_pos * tile_size) + scaled_y;

		let r = (256.0 * positioned_x) as u8;
		let b = ((z as f64 / 5.0) * 256.0) as u8;
		let g = (256.0 * positioned_y) as u8;

		*pixel = image::Rgb([r, g, b]);
	}

	let mut png_data = Vec::new();

	let encoder = png::PNGEncoder::new(&mut png_data);
	
	let _result = encoder.encode(&image_buffer, image_buffer.width(), image_buffer.height(), image::RGB(8));

	return png_data;
}

/*fn bounds_for_tile(x: u64, y: u64, z: u32) -> Bounds {
	
	let width = 256;
	let height = 256;

	let base: i64 = 2;

	let scale_factor: f64 = base.pow(z) as f64;

	let tile_size = 1.0 / scale_factor;


}*/