/*
 *  generation/world.rs
 *  
 *  Handles generation of high level world details like continents and climates at world level 0
 */

#[derive(Clone)]
pub struct World {
	pub name: String,
	pub id: u64,
	pub seed: String,
	pub sea_level: f64,
	pub temperature: f64,
	pub humidity: f64
	
}

impl World {
 
	pub fn to_string(&self) -> String {
		
		let representation = format!("ID: {} Name: {} Seed: {} Sea Level: {} Temperature: {} Humidity: {}", self.id, self.name, self.seed, self.sea_level, self.temperature, self.humidity);

		return representation;
	}

	pub fn to_json(&self) -> String {
		
		let representation = format!("{{\"name\":\"{}\",\"id\":\"{}\",\"seed\":\"{}\",\"sea_level\":\"{}\",\"temperature\":\"{}\",\"humidity\":\"{}\"}}", self.name, self.id, self.seed, self.sea_level, self.temperature, self.humidity);

		return representation;
	}
}

impl Default for World {
	
	fn default() -> World {
		World {
			name: "".to_string(),
			id: 0,
			seed: "".to_string(),
			sea_level: 0.0,
			temperature: 0.0,
			humidity: 0.0
		}
	}
}