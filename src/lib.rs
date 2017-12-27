use std::fmt;
use std::error::Error;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn add_remove_entities() {

    }
}

//TODO understand what type i just used here
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
    pub fn new(registry: &mut Registry) -> EntityStream {
        EntityStream {
            count: 0,
            num_entities: registry.get_num_entities(),
            registry: registry,
        }
    }

    pub fn next(&mut self) -> Option<&mut Entity> {
        self.count += 1;

        if self.count < self.num_entities {
            Some(self.registry.get_entity(self.count).unwrap())
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct EntityError {
    index: EntityIndex,
}

impl fmt::Display for EntityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} with index: {}", self.description(),  self.index)
    }
}

impl Error for EntityError {
    fn description(&self) -> &str {
        "Tried to access an entity which does not exist"
    }
}

struct Registry {
    components: Vec<Component>,
    entity_indicies: Vec<usize>,
}

impl Registry {
    pub fn make_entity(&mut self, components_to_add: Vec<Component>) {
        //Add the new entity index
        self.entity_indicies.push(self.components.len());

        //Add each component to the array
        for component in components_to_add {
            self.components.push(component);
        } 
    }
    
    fn get_entity_end(&self, entity_index: EntityIndex) -> usize {
        if self.entity_indicies.len() - 1 > entity_index {
            self.entity_indicies[entity_index+1]
        } else {
            self.components.len()
        }
    }

    pub fn remove_entity(&mut self, entity_index: EntityIndex) {
        //crash if EntityError
        self.try_remove_entity(entity_index).unwrap();
    }

    pub fn try_remove_entity(&mut self, entity_index: EntityIndex) -> Result<(), EntityError> {
        //If the entity in question does not exist, fail. TODO error handling
        if self.entity_indicies.len() <= entity_index {
            Err(EntityError{ index: entity_index })
        } else {
            //Remove each component of the entity
            let entity_end = self.get_entity_end(entity_index);
            let entity_begin = self.entity_indicies[entity_index];
            self.components.drain(entity_begin..entity_end);

            //Remove the entity index
            self.entity_indicies.remove(entity_index);
            Ok(())
        }
    }

    pub fn get_entity(&mut self, entity_index: EntityIndex) -> Result<&mut Entity, EntityError> {
        if self.entity_indicies.len() <= entity_index {
            Err(EntityError{ index: entity_index })
        } else { 
            let entity_end = self.get_entity_end(entity_index);
            let entity_begin = self.entity_indicies[entity_index];
            Ok(&mut self.components[entity_begin..entity_end])
        }
    }

    pub fn get_num_entities(&self) -> EntityIndex {
        self.entity_indicies.len()
    }

    pub fn get_entity_stream<'registry>(&'registry mut self) -> EntityStream<'registry> {
        EntityStream::new(self)
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
