#[macro_use]
pub extern crate log;
pub extern crate simple_logger;

mod project_logger;


fn main() {
    project_logger::init_project_logger();
    println!("Hello, world!");
}

