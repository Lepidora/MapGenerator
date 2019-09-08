/*
 *  worlds.rs
 *  
 *  Stores and manages active worlds
 */

extern crate rand;

use rand::Rng;

#[path = "generation/world.rs"]
mod world;

use world::World;

use std::sync::{ Arc, Mutex };
use std::collections::HashMap;

lazy_static! {
	static ref WORLD_STORE: Arc<Mutex<HashMap<u64, World>>> = {
		return Arc::new(Mutex::new(HashMap::new()));
	};
}

pub fn get_world_for_id(id: u64) -> Option<World> {

	println!("Getting world with id {}", id);

	let local_world_store = WORLD_STORE.clone();

	let store = local_world_store.lock().unwrap();

	let world_ref = store.get(&id);

	match world_ref {
		Some(world) => return Some(world.clone()),
		None => return None
	}
}

pub fn store_world(world: World) {

	let local_world_store = WORLD_STORE.clone();

	let mut store = local_world_store.lock().unwrap();

	store.insert(world.id, world);
}

pub fn world_id_exists(id: u64) -> bool {
	
	let local_world_store = WORLD_STORE.clone();

	let store = local_world_store.lock().unwrap();

	return store.contains_key(&id);
}

pub fn new_world() -> World {
	return new_world_with_values(None, None, None, None, None);
}

pub fn new_world_with_values(name: Option<String>, seed: Option<String>, sea_level: Option<f64>, temperature: Option<f64>, humidity: Option<f64>) -> World {
 
	let mut rng = rand::thread_rng();

	//Generate the id for the world to be used in the url and internally
	let mut id: u64 = rng.gen();

	//Make sure the ID is unique and not zero
	while world_id_exists(id) && id != 0 {
		id = rng.gen();
	}

	let mut world = World { ..Default::default() };

	world.id = id;

	//Put the values into our world, generating any that are missing
	world.name = match name {
		Some(name_val) => { println!("{}", &name_val); name_val }
		None => { "".to_string() /* TODO: add random name generation */ }
	};

	world.seed = match seed {
		Some(seed_val) => {
			if seed_val.len() > 0 { seed_val } else { generate_seed() }
		}
		None => { generate_seed() }
	};

	world.sea_level = match sea_level {
		Some(sea_level_val) => { sea_level_val }
		None => { rng.gen_range(0.0, 100.0) }
	};

	world.temperature = match temperature {
		Some(temperature_val) => { temperature_val }
		None => { rng.gen_range(0.0, 100.0) } 
	};

	world.humidity = match humidity {
		Some(humidity_val) => { humidity_val }
		None => { rng.gen_range(0.0, 100.0) }
	};

	//This guarantees any world generated is stored
	//This may be a bad idea, we'll see in retrospect...
	store_world(world.clone());

	return world;
}

fn generate_seed() -> String {
 
	let seed_length = 16;

	return rand::thread_rng().sample_iter(&rand::distributions::Alphanumeric).take(seed_length).collect::<String>();
}