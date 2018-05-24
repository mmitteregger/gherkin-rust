use ast::*;

pub trait ScenarioDefinition: Node {
    fn get_keyword(&self) -> &String;

    fn get_name(&self) -> &String;

    fn get_description(&self) -> &Option<String>;

    fn get_steps(&self) -> &Vec<Step>;
}

serialize_trait_object!(ScenarioDefinition);
impl_downcast!(ScenarioDefinition);
