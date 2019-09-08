#[macro_use]
extern crate lazy_static;

mod server;
mod worlds;

fn main() {
	server::start();
}

