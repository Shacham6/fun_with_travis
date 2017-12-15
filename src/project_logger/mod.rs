use simple_logger;


pub fn init_project_logger() {
    simple_logger::init().unwrap_or_else(|error| {
        error!("{}", error);
    });
}
