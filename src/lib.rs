use component::Component;

#[cfg(test)]
mod tests;

mod component;

mod registry;

use registry::Registry;

trait System<T: Component> {
    fn run(&mut self, registry: &mut Registry<T>) {
        let mut entity_stream = registry.get_entity_stream();
        loop {
            match entity_stream.next() {
                Some(entity) => {
                    // maybe call a closure here to capture the environment
                },
                None => break,
            }
        }
    }
}
