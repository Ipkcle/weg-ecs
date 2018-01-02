use std::fmt;
use std::error::Error;
use component::{Component, ComponentMask};

pub struct EntityStream<'registry, T: 'registry + Component> {
    count: EntityIndex,
    num_entities: EntityIndex,
    registry: &'registry mut Registry<T>,
}

///Streaming iterator for entities in registry
impl<'registry, T: Component> EntityStream<'registry, T> {
    pub fn new(registry: &mut Registry<T>) -> EntityStream<T> {
        EntityStream {
            count: 0,
            num_entities: registry.get_num_entities(),
            registry,
        }
    }

    pub fn next(&mut self) -> Option<&mut [T]> {
        self.count += 1;
        if self.count <= self.num_entities {
            Some(self.registry.try_get_entity(self.count - 1).unwrap())
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct EntityError {
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

#[derive(Debug)]
pub struct LinkError;

impl fmt::Display for LinkError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, either above the maximum of {} links, or to an entity which does not currently exist.", self.description(),  MAX_LINKS)
    }
}

impl Error for LinkError {
    fn description(&self) -> &str {
        "Attempted to make an invalid new link"
    }
}


type ComponentIndex = usize;
type EntityIndex = usize;
pub type Link = usize;

const MAX_LINKS: usize = 200;
pub struct Registry<T: Component> {
    components: Vec<T>,
    entity_indicies: Vec<ComponentIndex>,
    links: [Option<EntityIndex>; MAX_LINKS],
}

impl<T: Component> Registry<T> {
    pub fn make_entity(&mut self, components_to_add: Vec<T>) {
        //Add the new entity index
        self.entity_indicies.push(self.components.len());

        //Add each component to the array
        for component in components_to_add {
            self.components.push(component);
        } 
    }

    pub fn link_make_entity(&mut self, components: Vec<T>) -> Link {
        let entity_index = self.entity_indicies.len();
        self.make_entity(components);
        self.make_link(entity_index)
    }

    pub fn make_link(&mut self, index: EntityIndex) -> Link {
        self.try_make_link(index).unwrap()
    }

    pub fn try_make_link(&mut self, index: EntityIndex) -> Result<Link, LinkError> {
        let mut link: Result<Link, LinkError> = Err(LinkError);
        if index < self.entity_indicies.len() {
            for i in 0..MAX_LINKS {
                if self.links[i] == None{
                    self.links[i] = Some(index);
                    link = Ok(i);
                    break
                }
            }
        }
        link
    }
    
    pub fn free_link(&mut self, link: Link) {
        self.links[link] = None;
    }

    pub fn get_entity_by_link(&mut self, link: Link) -> &mut [T] {
        self.try_get_entity_by_link(link).unwrap()
    }

    pub fn try_get_entity_by_link(&mut self, link: Link) -> Result<&mut [T], LinkError> {
        match self.links[link] {
            Some(entity_index) => { Ok(self.get_entity(entity_index)) },
            None => { Err(LinkError) },
        }
    }

        
    fn update_links(&mut self, removed_entity_index: EntityIndex) {
        let mut to_remove: Option<usize> = None;
        for i in 0..MAX_LINKS {
            if let Some(ref mut index) = self.links[i] {
                if *index > removed_entity_index { *index = *index -1;
                } else if *index == removed_entity_index {
                    to_remove = Some(i);
                }
            }
        }
        if let Some(i) = to_remove {
            self.links[i] = None;
        }
    }

    fn get_entity_end(&self, entity_index: EntityIndex) -> usize {
        if self.entity_indicies.len() > entity_index + 1 {
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
        //If the entity in question does not exist, raise an EntityError.
        if self.entity_indicies.len() <= entity_index {
            Err(EntityError{ index: entity_index })
        } else {
            //Find first and last index of entity in the component array
            let entity_end = self.get_entity_end(entity_index);
            let entity_begin = self.entity_indicies[entity_index];

            //Remove each component of the entity
            self.components.drain(entity_begin..entity_end);

            //If the removed entity is not the most recent entity, 
            //update the entity beginnings of all more recent entities.
            if self.entity_indicies.len() > entity_index + 1 {
                let num_components_removed = entity_end - entity_begin;
                for entity_begin_index in &mut self.entity_indicies[entity_index + 1..] {
                    *entity_begin_index -= num_components_removed;
                }
            }

            //Remove the entity index
            self.entity_indicies.remove(entity_index);

            //Update links
            self.update_links(entity_index);

            Ok(())
        }
    }

    pub fn get_entity(&mut self, entity_index: EntityIndex) -> &mut [T] {
        //crash if EntityError
        self.try_get_entity(entity_index).unwrap()
    }

    pub fn try_get_entity(&mut self, entity_index: EntityIndex) -> Result<&mut [T], EntityError> {
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

    pub fn get_num_components(&self) -> usize {
        self.components.len()
    }
    
    pub fn mask_of(entity: &[T]) -> ComponentMask {
        let mut mask: ComponentMask = 0;
        for component in entity {
            mask &= component.get_mask();
        }
        mask
    }

    pub fn stream<'registry>(&'registry mut self) -> EntityStream<'registry, T> {
        EntityStream::new(self)
    }

    pub fn with_capacity(component_capacity: usize, entity_capacity: usize) {
        Registry {
            components: Vec::with_capacity(component_capacity),
            entity_indicies: Vec::with_capacity(entity_capacity),
            links: [None; MAX_LINKS],
        }
    }

    pub fn new() -> Registry<T> {
        Registry {
            components: Vec::new(),
            entity_indicies: Vec::new(),
            links: [None; MAX_LINKS],
        }
    }
}
