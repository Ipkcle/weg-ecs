use ::Registry;
use ::ExampleComponent;

#[test]
fn add_remove_crash_test() {
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
