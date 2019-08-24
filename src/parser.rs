/*
 *  parser.rs
 *  
 *  Compartmentalises HTTP request parsing
 */

use httparse;

use std::iter::FromIterator;

 pub fn get_path(header: String) -> Option<String> {
 
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