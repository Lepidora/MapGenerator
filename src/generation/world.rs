/*
 *  generation/world.rs
 *  
 *  Handles generation of high level world details like continents and climates at world level 0
 */

extern crate rand;

use rand::Rng;

 pub struct World {
	pub name: String,
	pub id: u64,
	pub seed: String,
	pub sea_level: f64,
	pub temperature: f64,
	pub humidity: f64
 }

 impl World {
 
	fn to_string(&self) -> String {
		
		let representation = format!("ID: {} Name: {} Seed: {} Sea Level: {} Temperature: {} Humidity: {}", self.id, self.name, self.seed, self.sea_level, self.temperature, self.humidity);

		return representation;
	}
 }

 pub fn new_world() -> u64 {
	
	let mut rng = rand::thread_rng();

	let id: u64 = rng.gen();

	return id;
 }