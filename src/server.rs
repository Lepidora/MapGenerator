/*
 *  server.rs
 *  
 *  Handles the operation of the tile HTTP server
 */

use json;

#[path = "parser.rs"]
mod parser;
#[path = "images.rs"]
mod images;
#[path = "generation/world.rs"]
mod world;
#[path = "worlds.rs"]
mod worlds;

use std::borrow::Cow::Owned;
use std::borrow::Cow::Borrowed;

use std::net::{ TcpStream, TcpListener }; 
use std::io::{ Read, Write };
use std::fs::File;
use std::iter::FromIterator;
use std::thread;

struct MapPos {
	pub x: f64,
	pub y: f64,
	pub z: u32,
}

//Starts the webserver and begins listening for connections
pub fn start() {

	let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

	println!("Listening for connections on port {}", 8080);

	for stream in listener.incoming() {
		
		match stream {
			
			Ok(stream) => {
				thread::spawn(|| {
					handle_connection(stream);
				});
			}
			Err(error) => {
				println!("Error in connection: {}", error);
			}
		}
	}
}

fn handle_connection(mut stream: TcpStream) {
	
	let mut buffer = [0u8;4096];

	match &stream.read(&mut buffer) {
		
		Ok(_) => {
		
			let request_string = String::from_utf8_lossy(&buffer);

			match request_string {

				Borrowed(rq) => {
					handle_out(stream, rq.to_string());
				},
				Owned(_) => {
					write_default(stream);
				},
			}
		},
		Err(error) => println!("Unable to read stream: {}", error)
	}
}

//Writes a default error message to the given stream
fn write_default(stream: TcpStream) {
	
	let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Unable to complete request</body></html>\r\n";

	write_stream(stream, response);
}

//Takes the given HTTP request and decides what to do with it
//If the path can be parsed into ZXY coordinates, it sends an image, otherwise it sends nothing
fn handle_out(stream: TcpStream, request: String) {
	
	let parsed_path = parser::get_path(&request);

	match parsed_path {
		
		Some(path) => {
			
			let path_components = parser::get_path_components(path);

			let body = parser::get_body(&request);

			if path_components.len() > 0 {
				
				match path_components[0].as_ref() {
				
					//These are matched against the first item after the / in the request url
					"new" => handle_new_world(stream, path_components, body),
					"tiles" => handle_tiles(stream, path_components),
					"client" => handle_static(stream, path_components),
					"get" => handle_get_world(stream, path_components),

					page => {

						if path_components.len() > 1 {
							write_default(stream);
						} else {

							let test = &page.parse::<u64>();

							match test {
								Ok(_) => handle_root(stream),
								Err(_) => write_default(stream),
							}
						}
					}
				}

			} else {
				handle_root(stream);
			}
		},

		None => write_default(stream)
	}
}

//Handles when a client asks for a new world to be generated
fn handle_new_world(stream: TcpStream, path_components: Vec<String>, body_string: Option<String>) {

	println!("{}", body_string.clone().unwrap());

	let parsed_body = parser::parse_body(body_string);

	let world;

	match parsed_body {

		Ok(body) => {
			
			let name = if !body["name"].is_null() { Some(body["name"].to_string()) } else { None };
			let seed = if !body["seed"].is_null() { Some(body["seed"].to_string()) } else { None };
			let sea_level = if !body["sea_level"].is_null() { body["sea_level"].as_f64() } else { println!("tessssst"); None };
			let temperature = if !body["temperature"].is_null() { body["temperature"].as_f64() } else { None };
			let humidity = if !body["humidity"].is_null() { body["humidity"].as_f64() } else { None };
			
			world = worlds::new_world_with_values(name, seed, sea_level, temperature, humidity)
		},
		Err(error) => { println!("{}", error); world = worlds::new_world() }
	};

	let mut header = default_header();

	header.extend(world.to_json().into_bytes());
	write_stream(stream, &header);
}

//Handles when a client requests an existing world
//This will eventually need refactoring to respond with a proper error to the client in the case a world doesn't exist
fn handle_get_world(stream: TcpStream, path_components: Vec<String>) {

	if path_components.len() >= 2 {
			
		let raw_id = &path_components[1];

		match raw_id.parse::<u64>() {
			Ok(id) => {

				match worlds::get_world_for_id(id) {

					Some(world) => {

						let mut header = default_header();

						header.extend(world.to_json().into_bytes());

						write_stream(stream, &header);
					},
					None => {
						write_error(stream, "Could not find world");
					}
				}				
			},
			Err(_) => write_error(stream, "Could not parse ID")
		};

	} else {
		write_error(stream, "No ID found in URL");
	}
}

//Handles when a client asks for a map tile
fn handle_tiles(stream: TcpStream, path_components: Vec<String>) {
	
	let coords = get_position(path_components);

	//println!("Responding to z: {}, x: {} y: {}", coords.z, coords.x, coords.y);

	let mut header = images::get_image_header();

	let image = images::image_for_coordinates(coords.x, coords.y, coords.z);

	header.extend(image);

	write_stream(stream, &header);
}

//Handles when a client visits the root page of the site. It reads and sends the client html page
fn handle_root(stream: TcpStream) {
	
	let components = vec!["client".to_string(), "client.html".to_string()];

	handle_static(stream, components);
}

//Handles delivery of static files. It fetches them from the 'client' folder
fn handle_static(stream: TcpStream, path_components: Vec<String>) {
	
	let file_name = &path_components[1];

	let mut buffer = Vec::new();
	
	let file_open = File::open(format!("client/{}", file_name));

	match file_open {

		Ok(mut file) => {

			match file.read_to_end(&mut buffer) {
				
				Ok(_) => {
					
					let file_components = Vec::from_iter(file_name.splitn(2, ".").map(String::from));

					let mut content_type = "text/plain";

					if file_components.len() >= 2 {
						
						let file_extension = &file_components[1];

						content_type = get_content_type(file_extension.to_string())
					}

					let mut header = content_header(content_type.to_string());

					header.extend(buffer);

					write_stream(stream, &header);
				},
				Err(error) => {
					write_string(stream, error.to_string());
				}
			}
		},
		Err(error) => {
			write_string(stream, error.to_string());
		}
	}
}

//Takes an error message, converts it into JSON and sends it to the client
fn write_error<S: Into<String>>(stream: TcpStream, error_message: S) {
	
	let mut header = content_header(get_content_type("json"));

	let data = format!("{{\"error\":{}}}", json::stringify(error_message.into()));

	header.extend(data.into_bytes());

	write_stream(stream, &header);
}

//Writes the given string to the given stream by appending it to a text header
fn write_string(stream: TcpStream, message: String) {

	let mut header = default_header();

	header.extend(message.into_bytes());

	write_stream(stream, &header);
} 

//Takes a file extension and returns a given content type
fn get_content_type<S: Into<String>>(extension: S) -> &'static str {
	
	match extension.into().as_ref() {
		"html" => return "text/html",
		"js" => return "text/javascript",
		"css" => return "text/css",
		"png" => return "image/png",
		"jpg" => return "image/jpeg",
		"json" => return "text/json",
		&_ => "text/plain"
	}
}

//Writes the given byte buffer to the given stream
fn write_stream(mut stream: TcpStream, content: &[u8]) {

	match stream.write(content) {
		
		Ok(_) => {},
		Err(error) => println!("Error sending response: {}", error),
	}
}

fn content_header<S: Into<String>>(content_type: S) -> Vec<u8> {
	
	let header = format!("HTTP/1.1 200 OK\r\nAccess-Control-Allow-Origin: *\r\nContent-Type: {}; charset=UTF-8\r\n\r\n", content_type.into());

	return header.to_string().into_bytes();
}

fn default_header() -> Vec<u8> {
	return content_header("text/plain".to_string());
}

// Takes the request parts and attempts to split it into a XYZ coordinate
fn get_position(split_url: Vec<String>) -> MapPos {
	
	//let split_url = parser::get_path_components(url);

	if split_url.len() < 3 {
		return MapPos{ x:0.0, y:0.0, z:0 };
	}

	let ref raw_y = split_url[split_url.len() - 1];
	let ref raw_x = split_url[split_url.len() - 2];
	let ref raw_z = split_url[split_url.len() - 3];

	let x = raw_x.parse::<f64>();
	let y = raw_y.parse::<f64>();
	let z = raw_z.parse::<u32>();

	if x.is_err() || y.is_err() || z.is_err() {
		return MapPos{ x:0.0, y:0.0, z:0 };
	} else {
		return MapPos{ x:x.unwrap(), y:y.unwrap(), z:z.unwrap() };
	}
}