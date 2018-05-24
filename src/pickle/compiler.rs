use std::default::Default;

use pickle::*;
use ast::*;

pub struct Compiler;

impl Default for Compiler {
    fn default() -> Compiler {
        Compiler {}
    }
}

impl Compiler {
    pub fn compile(&mut self, gherkin_document: &GherkinDocument) -> Vec<Pickle> {
        let feature: &Feature = match gherkin_document.get_feature() {
            Some(feature) => feature,
            None => return Vec::new(),
        };

        let mut pickles = Vec::with_capacity(feature.get_children().len());
        let feature_tags = feature.get_tags();
        let language = feature.get_language();
        let mut background_steps: Vec<PickleStep> = Vec::new();

        for scenario_definition in feature.get_children() {
            if let Some(background) = scenario_definition.downcast_ref::<Background>() {
                background_steps = self.pickle_steps(background);
            } else if let Some(scenario) = scenario_definition.downcast_ref::<Scenario>() {
                self.compile_scenario(&mut pickles, &background_steps, scenario, feature_tags, language);
            } else if let Some(scenario_outline) = scenario_definition.downcast_ref::<ScenarioOutline>() {
                self.compile_scenario_outline(&mut pickles, &background_steps, scenario_outline, feature_tags, language);
            } else {
                panic!("Unexpected scenario definition: {:?}", scenario_definition);
            }
        }

        pickles
    }

    fn compile_scenario(&mut self, pickles: &mut Vec<Pickle>, background_steps: &Vec<PickleStep>,
        scenario: &Scenario, feature_tags: &Vec<Tag>, language: &String) {

        let name = scenario.get_name().to_owned();
        let language = language.to_owned();
        let steps = self.compile_scenario_steps(background_steps, scenario);
        let tags = self.compile_scenario_tags(feature_tags, scenario);
        let locations = vec![self.pickle_location(scenario.get_location())];
        let pickle = Pickle::new(name, language, steps, tags, locations);

        pickles.push(pickle);
    }

    fn compile_scenario_steps(&mut self, background_steps: &Vec<PickleStep>,
            scenario: &Scenario) -> Vec<PickleStep> {

        let scenario_steps = scenario.get_steps();
        let steps_capacity = background_steps.len() + scenario_steps.len();
        let mut steps = Vec::with_capacity(steps_capacity);

        if !scenario_steps.is_empty() {
            steps.extend_from_slice(background_steps);
        }

        steps.extend(self.pickle_steps(scenario));
        steps
    }

    fn compile_scenario_tags(&mut self, feature_tags: &Vec<Tag>, scenario: &Scenario) -> Vec<PickleTag> {
        let scenario_tags = scenario.get_tags();
        let mut tags = Vec::with_capacity(feature_tags.len() + scenario_tags.len());

        tags.extend_from_slice(feature_tags);
        tags.extend_from_slice(scenario_tags);

        self.pickle_tags(tags)
    }

    fn compile_scenario_outline(&mut self, pickles: &mut Vec<Pickle>, background_steps: &Vec<PickleStep>,
        scenario_outline: &ScenarioOutline, feature_tags: &Vec<Tag>, language: &String) {

        for examples in scenario_outline.get_examples() {
            let table_header: &TableRow = match examples.get_table_header() {
                Some(table_header) => table_header,
                None => return,
            };
            let table_body = examples.get_table_body().as_ref().unwrap();

            let variable_cells = table_header.get_cells();
            for values in table_body {
                let value_cells = values.get_cells();

                let name = self.interpolate(scenario_outline.get_name(), variable_cells, value_cells);
                let language = language.to_owned();
                let steps = self.compile_scenario_outline_steps(background_steps, scenario_outline, variable_cells, value_cells, values);
                let tags = self.compile_scenario_outline_tags(feature_tags, scenario_outline, examples);
                let locations = vec![
                    self.pickle_location(values.get_location()),
                    self.pickle_location(scenario_outline.get_location()),
                ];
                let pickle = Pickle::new(name, language, steps, tags, locations);

                pickles.push(pickle);
            }
        }
    }

    fn compile_scenario_outline_steps(&mut self, background_steps: &Vec<PickleStep>,
            scenario_outline: &ScenarioOutline, variable_cells: &Vec<TableCell>,
            value_cells: &Vec<TableCell>, values: &TableRow) -> Vec<PickleStep> {

        let scenario_outline_steps = scenario_outline.get_steps();
        let steps_capacity = background_steps.len() + scenario_outline_steps.len();
        let mut steps = Vec::with_capacity(steps_capacity);

        if !scenario_outline_steps.is_empty() {
            steps.extend_from_slice(background_steps);
        }

        for scenario_outline_step in scenario_outline.get_steps() {
            let step_text = self.interpolate(scenario_outline_step.get_text(), variable_cells, value_cells);
            let arguments = self.create_pickle_arguments(scenario_outline_step.get_argument(), variable_cells, value_cells);
            let locations = vec![
                self.pickle_location(values.get_location()),
                self.pickle_step_location(scenario_outline_step),
            ];

            let pickle_step = PickleStep::new(step_text, arguments, locations);
            steps.push(pickle_step);
        }

        steps
    }

    fn compile_scenario_outline_tags(&mut self, feature_tags: &Vec<Tag>,
            scenario_outline: &ScenarioOutline, examples: &Examples) -> Vec<PickleTag> {

        let scenario_outline_tags = scenario_outline.get_tags();
        let examples_tags = examples.get_tags();
        let tags_capacity = feature_tags.len()
            + scenario_outline_tags.len()
            + examples_tags.len();

        let mut tags = Vec::with_capacity(tags_capacity);
        tags.extend_from_slice(feature_tags);
        tags.extend_from_slice(scenario_outline_tags);
        tags.extend_from_slice(examples_tags);

        self.pickle_tags(tags)
    }

    fn create_pickle_arguments(&mut self, argument: &Option<Box<Node>>,
        variable_cells: &Vec<TableCell>, value_cells: &Vec<TableCell>) -> Vec<Box<Argument>> {

        let argument: &Box<Node> = match argument {
            Some(argument) => argument,
            None => return Vec::new(),
        };

        if let Some(data_table) = argument.downcast_ref::<DataTable>() {
            let rows = data_table.get_rows().iter()
                .map(|row: &TableRow| {
                    let cells = row.get_cells().iter()
                        .map(|cell: &TableCell| {
                            let location = self.pickle_location(cell.get_location());
                            let value = self.interpolate(cell.get_value(), variable_cells, value_cells);

                            PickleCell::new(location, value)
                        })
                        .collect::<Vec<PickleCell>>();

                    PickleRow::new(cells)
                })
                .collect::<Vec<PickleRow>>();
            let pickle_table = PickleTable::new(rows);

            vec![Box::new(pickle_table)]
        } else if let Some(doc_string) = argument.downcast_ref::<DocString>() {
            let location = self.pickle_location(doc_string.get_location());
            let content = self.interpolate(doc_string.get_content(), variable_cells, value_cells);
            let content_type = match doc_string.get_content_type() {
                Some(content_type) => Some(self.interpolate(content_type, variable_cells, value_cells)),
                None => None,
            };
            let pickle_string = PickleString::new(location, content, content_type);

            vec![Box::new(pickle_string)]
        } else {
            panic!("Unexpected argument: {:?}", argument);
        }
    }

    fn pickle_steps<SD: ScenarioDefinition>(&mut self, scenario_definition: &SD) -> Vec<PickleStep> {
        scenario_definition.get_steps().into_iter()
            .map(|step| self.pickle_step(step))
            .collect()
    }

    fn pickle_step(&mut self, step: &Step) -> PickleStep {
        let text = step.get_text().to_owned();
        let arguments = self.create_pickle_arguments(step.get_argument(), &Vec::new(), &Vec::new());
        let locations = vec![self.pickle_step_location(step)];

        PickleStep::new(text, arguments, locations)
    }

    fn interpolate(&mut self, text: &String, variable_cells: &Vec<TableCell>,
        value_cells: &Vec<TableCell>) -> String {

        let mut interpolated_text = text.to_owned();

        for (column, variable_cell) in variable_cells.iter().enumerate() {
            let value_cell = &value_cells[column];
            let header = variable_cell.get_value().as_str();
            let value = value_cell.get_value().as_str();
            interpolated_text = interpolated_text.replace(&format!("<{}>", header), value);
        }

        interpolated_text
    }

    fn pickle_step_location(&mut self, step: &Step) -> PickleLocation {
        let keyword_column = if step.get_keyword().is_empty() {
            0
        } else {
            step.get_keyword().chars().count()
        };
        let step_location = step.get_location();
        let line = step_location.get_line();
        let column = step_location.get_column() + keyword_column;
        PickleLocation::new(line, column)
    }

    fn pickle_location(&mut self, location: Location) -> PickleLocation {
        PickleLocation::new(location.get_line(), location.get_column())
    }

    fn pickle_tags(&mut self, tags: Vec<Tag>) -> Vec<PickleTag> {
        tags.into_iter()
            .map(|tag| self.pickle_tag(tag))
            .collect()
    }

    fn pickle_tag(&mut self, tag: Tag) -> PickleTag {
        PickleTag::new(self.pickle_location(tag.get_location()), tag.take_name())
    }
}
