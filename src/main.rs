use electron_injector::injector::Injector;

fn main() {
    // Setup logging
    pretty_env_logger::init();

    // Run the injector
    let injector = Injector::new();
    injector.run().unwrap();
}
