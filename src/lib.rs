use registry::Registry;
use component::Component;

#[macro_use]
mod macros;

#[cfg(test)]
mod tests;

mod component;

mod registry;

trait System<T: Component> {
    fn run(&mut self, registry: &mut Registry<T>);
    fn mask() -> component::ComponentMask;
}

