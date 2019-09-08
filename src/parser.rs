/*
 *  parser.rs
 *  
 *  Compartmentalises HTTP request parsing
 */

use httparse;
use json;

use std::iter::FromIterator;

 pub fn get_path(header: &String) -> Option<String> {
 
	let mut headers = [httparse::EMPTY_HEADER; 16];

	let mut request = httparse::Request::new(&mut headers);
 
	let _res = request.parse(header.as_bytes());

	match request.path {
		Some(ref path) => {
			return Some(path.to_string());
		},
		None => {
			return None;
		}
	}
 }

 pub fn get_path_components(path: String) -> Vec<String> {

	let mut split_url = Vec::from_iter(path.split("/").map(String::from));

	split_url.retain(|s| s.len() > 0);

	return split_url;
 }

 pub fn get_body(request: &String) -> Option<String> {
	
	let split_request = Vec::from_iter(request.splitn(2, "\r\n\r\n").map(String::from));

	if split_request.len() > 1 {
		return Some(split_request[1].to_string());
	} else {
		return None;
	}
 }

 pub fn parse_body(body_string: Option<String>) -> json::Result<json::JsonValue> {

	match body_string {
		Some(contents) => {
		
			let trimmed = contents.trim_matches(char::from(0)).trim();
			
			json::parse(&trimmed)
		},
		None => json::parse("")
	}
 }