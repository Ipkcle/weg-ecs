use registry::{ Registry, Link };

fn mask_of(entity: &[ExampleComponent]) -> u32 {
    let mut mask: u32 = 0;
    for component in entity {
        mask &= component.get_mask();
    }
    mask
}

struct ExampleRenderSystem;

impl ExampleRenderSystem {
    fn run_on_entity(entity: &[ExampleComponent]) {
        let mut n: Option<&String> = None;
        for component in entity {
            match *component {
                ExampleComponent::Name(ref name) => { n = Some(name) },
                _ => (),
            }
        }
        match n {
            Some(name) => println!("name of this entity is: {}", name),
            None => println!("this entity has no name"),
        }
    }

    pub fn run(&mut self, registry: &mut Registry<ExampleComponent>) {
        let mut entity_stream = registry.stream();
        For!(entity in entity_stream => {
            ExampleRenderSystem::run_on_entity(entity);
        })
    }
}

mod print_system {
    use registry::Registry;
    use super::ExampleComponent;
    use super::ExampleComponent::{Count, Name, Velocity};

    pub struct Print;

    impl Print {
        pub fn print_entity(entity: &[ExampleComponent]) {
            println!("BEGIN ENTITY -------");
            for component in entity {
                match *component {
                    Name(ref n) => { println!("Name: {}", n) }, 
                    Count(ref c) => { println!("Count: {}", c) },
                    Velocity(ref v) => { println!("Velocity: {}", v) },
                }
            }
            //println!("END ENTITY");
        }

        pub fn run(&mut self, registry: &mut Registry<ExampleComponent>) {
            let mut stream = registry.stream();
            For!(entity in stream => {
                Print::print_entity(entity);
            })
        }
    }
}


mod physics_system {
    use registry::Registry;
    use super::ExampleComponent;
    use super::ExampleComponent::{Count, Velocity};
    use super::mask_of;

    static MASK: u32 = (1<<0 & 1<<2);

    pub struct Physics;

    impl Physics {
        fn move_entity(entity: &mut [ExampleComponent]) {
            let mut velocity: &i32 = &0;
            let mut count: &mut i32 = &mut 0;
            for component in entity {
                match *component {
                   Count(ref mut c) => { count = c },
                   Velocity(ref v) => { velocity = v },
                   _ => (),
                }
            }
            *count += *velocity;
        }

        pub fn run(&mut self, registry: &mut Registry<ExampleComponent>) {
            let mut stream = registry.stream();
            For!(entity in stream => {
                let entity_mask = mask_of(entity);
                if MASK & entity_mask == entity_mask {
                    Physics::move_entity(entity);
                }
            })
        }
    }
}

pub enum ExampleComponent {
    Count(i32),
    Name(String),
    Velocity(i32),
}

impl ExampleComponent {
    fn get_mask(&self) -> u32 {
        use self::ExampleComponent::*;
        match self {
            &Count(..) => 1 << 0,
            &Name(..) => 1 << 1,
            &Velocity(..) => 1 << 2,
        }
    }
}

#[test]
fn links() {
    use self::ExampleComponent::*;
    use self::print_system::Print;

    let mut registry: Registry<ExampleComponent> = Registry::new();

    let first_link: Link = registry.link_make_entity(vec![Count(0), Velocity(2), Name(String::from("first"))]);
    let second_link = registry.link_make_entity(vec![Count(0), Velocity(1), Name(String::from("second"))]);
    let third_link = registry.link_make_entity(vec![Velocity(20), Name(String::from("third"))]);
    let fourth_link = registry.link_make_entity(vec![Count(0), Name(String::from("fourth"))]);
    let fifth_link = registry.link_make_entity(vec![Count(0), Velocity(100)]);

    registry.remove_entity(2);
    Print::print_entity(registry.get_entity_by_link(fifth_link));
    Print::print_entity(registry.get_entity_by_link(fourth_link));
    registry.remove_entity(0);
    registry.remove_entity(0);
    registry.remove_entity(0);
    registry.remove_entity(0);
}

#[test]
fn physics_system() {
    use self::ExampleComponent::*;

    let mut registry: Registry<ExampleComponent> = Registry::new();
    registry.make_entity(vec![Count(0), Velocity(2), Name(String::from("first"))]);
    registry.make_entity(vec![Count(0), Velocity(1), Name(String::from("second"))]);
    registry.make_entity(vec![Velocity(20), Name(String::from("third"))]);
    registry.make_entity(vec![Count(0), Name(String::from("fourth"))]);
    registry.make_entity(vec![Count(0), Velocity(100)]);
    
    let mut phys = physics_system::Physics{};
    let mut print = print_system::Print{};
    
    print.run(&mut registry);
    println!("Running Physics system!");
    phys.run(&mut registry);
    //
    print.run(&mut registry);
    println!("Running Physics system!");
    phys.run(&mut registry);
    //
    print.run(&mut registry);
    println!("Running Physics system!");
    phys.run(&mut registry);
    //
    print.run(&mut registry);
    println!("Running Physics system!");
    phys.run(&mut registry);
    print.run(&mut registry);
}

#[test]
fn system_crash() {
    let mut registry: Registry<ExampleComponent> = Registry::new();
    registry.make_entity(vec![ExampleComponent::Count(0), ExampleComponent::Name(String::from("first"))]);
    registry.make_entity(vec![ExampleComponent::Count(0), ExampleComponent::Name(String::from("second"))]);
    registry.make_entity(vec![ExampleComponent::Name(String::from("third"))]);
    registry.make_entity(vec![ExampleComponent::Count(0), ExampleComponent::Name(String::from("fourth"))]);
    registry.make_entity(vec![ExampleComponent::Count(0)]);
    
    let mut system = ExampleRenderSystem{};

    system.run(&mut registry);
}

#[test]
fn add_remove_entities_crash() {
    let mut registry: Registry<ExampleComponent> = Registry::new();
    registry.make_entity(vec![ExampleComponent::Count(0), ExampleComponent::Name(String::from("first"))]);
    registry.make_entity(vec![ExampleComponent::Count(0), ExampleComponent::Name(String::from("second"))]);
    registry.make_entity(vec![ExampleComponent::Name(String::from("third"))]);
    registry.make_entity(vec![ExampleComponent::Count(0), ExampleComponent::Name(String::from("fourth"))]);
    registry.remove_entity(0);
    registry.remove_entity(0);
    registry.remove_entity(0);
    registry.remove_entity(0);
}

#[test]
fn add_remove_entities() {
    let mut registry: Registry<ExampleComponent> = Registry::new();
    assert_eq!(registry.get_num_entities(), 0);
    assert_eq!(registry.get_num_components(), 0);

    registry.make_entity(vec![ExampleComponent::Count(0), ExampleComponent::Name(String::from("first"))]);
    assert_eq!(registry.get_num_entities(), 1);
    assert_eq!(registry.get_num_components(), 2);

    registry.make_entity(vec![ExampleComponent::Count(0), ExampleComponent::Name(String::from("second"))]);
    assert_eq!(registry.get_num_entities(), 2);
    assert_eq!(registry.get_num_components(), 4);

    registry.make_entity(vec![ExampleComponent::Name(String::from("third"))]);
    assert_eq!(registry.get_num_entities(), 3);
    assert_eq!(registry.get_num_components(), 5);
    
    registry.make_entity(vec![ExampleComponent::Count(0), ExampleComponent::Name(String::from("fourth"))]);
    assert_eq!(registry.get_entity(3).len(), 2);
    assert_eq!(registry.get_num_entities(), 4);
    assert_eq!(registry.get_num_components(), 7);

    assert_eq!(registry.get_entity(0).len(), 2);
    registry.remove_entity(0);
    assert_eq!(registry.get_num_entities(), 3);
    assert_eq!(registry.get_num_components(), 5);

    assert_eq!(registry.get_entity(2).len(), 2);
    registry.remove_entity(2);
    assert_eq!(registry.get_num_entities(), 2);
    assert_eq!(registry.get_num_components(), 3);

    assert_eq!(registry.get_entity(1).len(), 1);
    registry.remove_entity(1);
    assert_eq!(registry.get_num_entities(), 1);
    assert_eq!(registry.get_num_components(), 2);

    assert_eq!(registry.get_entity(0).len(), 2);
    registry.remove_entity(0);
    assert_eq!(registry.get_num_entities(), 0);
    assert_eq!(registry.get_num_components(), 0);
}
