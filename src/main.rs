extern crate ui;
extern crate env_logger;

fn main() {
    env_logger::init().unwrap();
    ui::start();
}
