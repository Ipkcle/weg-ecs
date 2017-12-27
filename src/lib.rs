#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

type Entity = [Component];
type EntityIndex = usize;

enum Component {
    Count(i32),
    Name(String),
}

struct EntityStream<'registry> {
    count: EntityIndex,
    num_entities: EntityIndex,
    registry: &'registry mut Registry,
}

//streaming iterator for entities in registry
impl<'registry> EntityStream<'registry> {
    pub fn new(num_entities: EntityIndex, registry: &mut Registry) -> EntityStream {
        EntityStream {
            count: 0,
            num_entities: num_entities,
            registry: registry,
        }
    }

    pub fn next(&mut self) -> Option<&mut Entity> {
        self.count += 1;

        if self.count < self.registry.get_num_entities() {
            Some(self.registry.get_entity(self.count))
        } else {
            None
        }
    }
}

struct Registry {
    components: Vec<Component>,
    entity_indicies: Vec<usize>,
}

impl Registry {
    pub fn add_entity(&mut self, entity: &Entity) {
        //pass
    }
    
    pub fn remove_entity(&mut self, entity_index: EntityIndex) {
        //pass
    }

    pub fn get_entity(&mut self, entity_index: EntityIndex) -> &mut Entity {
        &mut self.components[0..1]
    }

    pub fn get_num_entities(&self) -> EntityIndex {
        self.entity_indicies.len()
    }

    pub fn get_entity_stream<'registry>(&'registry mut self) -> EntityStream<'registry> {
        EntityStream::new(self.get_num_entities(), self)
    }
}

fn render_system(registry: &mut Registry) {
    let mut entity_stream = registry.get_entity_stream();
    loop {
        match entity_stream.next() {
            Some(entity) => {
                //pass
            },
            None => break,
        }
    }
}
