use registry::Registry;
use component::Component;

#[macro_use]
pub mod macros;

#[cfg(test)]
mod tests;

pub mod component;

pub mod registry;

pub trait System<T: Component> {
    fn run(&mut self, registry: &mut Registry<T>);
}

