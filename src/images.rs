/*
 *  images.rs
 *  
 *  Provides images for the server to send to the client
 */

use std::fs::File;
use std::io::prelude::*;

extern crate image;

use image::png;

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
	
	return color_test_tile(x, y, z);
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