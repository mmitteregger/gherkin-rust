use ast;
use pickle;
use std::default::Default;

pub struct Compiler;

impl Default for Compiler {
    fn default() -> Compiler {
        Compiler {}
    }
}

impl Compiler {
    pub fn compile(&mut self, gherkin_document: &ast::GherkinDocument) -> Vec<pickle::Pickle> {
        let feature: &ast::Feature = match &gherkin_document.feature {
            Some(feature) => feature,
            None => return Vec::new(),
        };

        let mut pickles = Vec::with_capacity(feature.scenario_definitions.len());
        let feature_tags = &feature.tags;
        let language = &feature.language;
        let mut background_steps: Vec<pickle::Step> = Vec::new();

        for scenario_definition in &feature.scenario_definitions {
            match scenario_definition {
                ast::ScenarioDefinition::Background(background) => {
                    background_steps = self.background_pickle_steps(background);
                },
                ast::ScenarioDefinition::Scenario(scenario) => {
                    self.compile_scenario(
                        &mut pickles,
                        &background_steps,
                        scenario,
                        feature_tags,
                        language,
                    );
                },
                ast::ScenarioDefinition::ScenarioOutline(scenario_outline) => {
                    self.compile_scenario_outline(
                        &mut pickles,
                        &background_steps,
                        scenario_outline,
                        feature_tags,
                        language,
                    );
                },
            }
        }

        pickles
    }

    fn compile_scenario(
        &mut self,
        pickles: &mut Vec<pickle::Pickle>,
        background_steps: &[pickle::Step],
        scenario: &ast::Scenario,
        feature_tags: &[ast::Tag],
        language: &str,
    ) {
        let name = scenario.name.to_owned();
        let language = language.to_owned();
        let steps = self.compile_scenario_steps(background_steps, scenario);
        let tags = self.compile_scenario_tags(feature_tags, scenario);
        let locations = vec![self.pickle_location(scenario.location)];
        let pickle = pickle::Pickle {
            name,
            language,
            steps,
            tags,
            locations,
        };

        pickles.push(pickle);
    }

    fn compile_scenario_steps(
        &mut self,
        background_steps: &[pickle::Step],
        scenario: &ast::Scenario,
    ) -> Vec<pickle::Step> {
        let scenario_steps = &scenario.steps;
        let steps_capacity = background_steps.len() + scenario_steps.len();
        let mut steps = Vec::with_capacity(steps_capacity);

        if !scenario_steps.is_empty() {
            steps.extend_from_slice(background_steps);
        }

        steps.extend(self.scenario_pickle_steps(scenario));
        steps
    }

    fn compile_scenario_tags(
        &mut self,
        feature_tags: &[ast::Tag],
        scenario: &ast::Scenario,
    ) -> Vec<pickle::Tag> {
        let scenario_tags = &scenario.tags;
        let mut tags = Vec::with_capacity(feature_tags.len() + scenario_tags.len());

        tags.extend_from_slice(feature_tags);
        tags.extend_from_slice(scenario_tags);

        self.pickle_tags(tags)
    }

    fn compile_scenario_outline(
        &mut self,
        pickles: &mut Vec<pickle::Pickle>,
        background_steps: &[pickle::Step],
        scenario_outline: &ast::ScenarioOutline,
        feature_tags: &[ast::Tag],
        language: &str,
    ) {
        for examples in &scenario_outline.examples {
            let table_header: &ast::TableRow = match &examples.table_header {
                Some(table_header) => table_header,
                None => return,
            };
            let table_body = examples.table_body.as_ref().unwrap();

            let variable_cells = &table_header.cells;
            for values in table_body {
                let value_cells = &values.cells;

                let name =
                    self.interpolate(&scenario_outline.name, variable_cells, value_cells);
                let language = language.to_owned();
                let steps = self.compile_scenario_outline_steps(
                    background_steps,
                    scenario_outline,
                    variable_cells,
                    value_cells,
                    values,
                );
                let tags =
                    self.compile_scenario_outline_tags(feature_tags, scenario_outline, examples);
                let locations = vec![
                    self.pickle_location(values.location),
                    self.pickle_location(scenario_outline.location),
                ];
                let pickle = pickle::Pickle {
                    name,
                    language,
                    steps,
                    tags,
                    locations,
                };

                pickles.push(pickle);
            }
        }
    }

    fn compile_scenario_outline_steps(
        &mut self,
        background_steps: &[pickle::Step],
        scenario_outline: &ast::ScenarioOutline,
        variable_cells: &[ast::TableCell],
        value_cells: &[ast::TableCell],
        values: &ast::TableRow,
    ) -> Vec<pickle::Step> {
        let scenario_outline_steps = &scenario_outline.steps;
        let steps_capacity = background_steps.len() + scenario_outline_steps.len();
        let mut steps = Vec::with_capacity(steps_capacity);

        if !scenario_outline_steps.is_empty() {
            steps.extend_from_slice(background_steps);
        }

        for scenario_outline_step in &scenario_outline.steps {
            let text = self.interpolate(
                &scenario_outline_step.text,
                variable_cells,
                value_cells,
            );
            let arguments = self.create_pickle_arguments(
                scenario_outline_step.argument.as_ref(),
                variable_cells,
                value_cells,
            );
            let locations = vec![
                self.pickle_location(values.location),
                self.pickle_step_location(scenario_outline_step),
            ];

            let pickle_step = pickle::Step {
                text,
                arguments,
                locations,
            };
            steps.push(pickle_step);
        }

        steps
    }

    fn compile_scenario_outline_tags(
        &mut self,
        feature_tags: &[ast::Tag],
        scenario_outline: &ast::ScenarioOutline,
        examples: &ast::Examples,
    ) -> Vec<pickle::Tag> {
        let scenario_outline_tags = &scenario_outline.tags;
        let examples_tags = &examples.tags;
        let tags_capacity = feature_tags.len() + scenario_outline_tags.len() + examples_tags.len();

        let mut tags = Vec::with_capacity(tags_capacity);
        tags.extend_from_slice(feature_tags);
        tags.extend_from_slice(scenario_outline_tags);
        tags.extend_from_slice(examples_tags);

        self.pickle_tags(tags)
    }

    fn create_pickle_arguments(
        &mut self,
        argument: Option<&ast::Argument>,
        variable_cells: &[ast::TableCell],
        value_cells: &[ast::TableCell],
    ) -> Vec<pickle::Argument> {
        let argument = match argument {
            Some(argument) => argument,
            None => return Vec::new(),
        };

        match argument {
            ast::Argument::DocString(doc_string) => {
                let location = self.pickle_location(doc_string.location);
                let content = self.interpolate(&doc_string.content, variable_cells, value_cells);
                let content_type = match &doc_string.content_type {
                    Some(content_type) => {
                        Some(self.interpolate(content_type, variable_cells, value_cells))
                    }
                    None => None,
                };
                let pickle_string = pickle::String {
                    location,
                    content,
                    content_type,
                };

                vec![pickle::Argument::String(pickle_string)]
            },
            ast::Argument::DataTable(data_table) => {
                let rows = data_table.rows
                    .iter()
                    .map(|row: &ast::TableRow| {
                        let cells = row.cells
                            .iter()
                            .map(|cell: &ast::TableCell| {
                                let location = self.pickle_location(cell.location);
                                let value =
                                    self.interpolate(&cell.value, variable_cells, value_cells);

                                pickle::Cell { location, value }
                            })
                            .collect::<Vec<pickle::Cell>>();

                        pickle::Row { cells }
                    })
                    .collect::<Vec<pickle::Row>>();
                let pickle_table = pickle::Table { rows };

                vec![pickle::Argument::Table(pickle_table)]
            },
        }
    }

    fn background_pickle_steps(
        &mut self,
        background: &ast::Background,
    ) -> Vec<pickle::Step> {
        background.steps
            .iter()
            .map(|step| self.pickle_step(step))
            .collect()
    }

    fn scenario_pickle_steps(
        &mut self,
        scenario: &ast::Scenario,
    ) -> Vec<pickle::Step> {
        scenario.steps
            .iter()
            .map(|step| self.pickle_step(step))
            .collect()
    }

    fn pickle_step(&mut self, step: &ast::Step) -> pickle::Step {
        let text = step.text.to_owned();
        let arguments = self.create_pickle_arguments(step.argument.as_ref(), &Vec::new(), &Vec::new());
        let locations = vec![self.pickle_step_location(step)];

        pickle::Step {
            text,
            arguments,
            locations,
        }
    }

    fn interpolate(
        &mut self,
        text: &str,
        variable_cells: &[ast::TableCell],
        value_cells: &[ast::TableCell],
    ) -> String {
        let mut interpolated_text = text.to_owned();

        for (column, variable_cell) in variable_cells.iter().enumerate() {
            let value_cell = &value_cells[column];
            let header = &variable_cell.value;
            let value = &value_cell.value;
            interpolated_text = interpolated_text.replace(&format!("<{}>", header), value);
        }

        interpolated_text
    }

    fn pickle_step_location(&mut self, step: &ast::Step) -> pickle::Location {
        let keyword_column = if step.keyword.is_empty() {
            0
        } else {
            step.keyword.chars().count() as u32
        };
        let step_location = step.location;
        let line = step_location.line;
        let column = step_location.column + keyword_column;
        pickle::Location { line, column }
    }

    fn pickle_location(&mut self, location: ast::Location) -> pickle::Location {
        pickle::Location {
            line: location.line,
            column: location.column,
        }
    }

    fn pickle_tags(&mut self, tags: Vec<ast::Tag>) -> Vec<pickle::Tag> {
        tags.into_iter().map(|tag| self.pickle_tag(tag)).collect()
    }

    fn pickle_tag(&mut self, tag: ast::Tag) -> pickle::Tag {
        pickle::Tag {
            location: self.pickle_location(tag.location),
            name: tag.name,
        }
    }
}
