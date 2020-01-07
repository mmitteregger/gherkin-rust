use std::borrow::Cow;

use crate::ast;
use crate::cuke;

#[derive(Default)]
pub struct Compiler;

impl Compiler {
    pub fn compile<'d>(
        &mut self,
        gherkin_document: &'d ast::GherkinDocument,
    ) -> Vec<cuke::Cuke<'d>> {
        let feature: &ast::Feature = match &gherkin_document.feature {
            Some(feature) => feature,
            None => return Vec::new(),
        };

        let mut cukes = Vec::with_capacity(feature.scenario_definitions.len());
        let mut background: Option<&ast::Background> = None;
        let mut background_steps: Vec<cuke::Step<'_>> = Vec::new();

        for scenario_definition in &feature.scenario_definitions {
            match scenario_definition {
                ast::ScenarioDefinition::Background(ref bg) => {
                    background = Some(bg);
                    background_steps = self.background_cuke_steps(bg);
                }
                ast::ScenarioDefinition::Scenario(scenario) => {
                    self.compile_scenario(
                        &mut cukes,
                        feature,
                        background,
                        &background_steps,
                        scenario,
                    );
                }
                ast::ScenarioDefinition::ScenarioOutline(scenario_outline) => {
                    self.compile_scenario_outline(
                        &mut cukes,
                        feature,
                        background,
                        &background_steps,
                        scenario_outline,
                    );
                }
            }
        }

        cukes
    }

    fn compile_scenario<'d>(
        &mut self,
        cukes: &mut Vec<cuke::Cuke<'d>>,
        feature: &'d ast::Feature,
        background: Option<&'d ast::Background>,
        background_steps: &[cuke::Step<'d>],
        scenario: &'d ast::Scenario,
    ) {
        let scenario_definition = cuke::ScenarioDefinition::from(scenario);
        let name = Cow::Borrowed(scenario.name.as_str());
        let language = &feature.language;
        let (background_steps, scenario_steps) =
            self.compile_scenario_steps(background_steps, scenario);
        let tags = self.compile_scenario_tags(feature, scenario);
        let locations = vec![cuke::Location::from(scenario.location)];
        let cuke = cuke::Cuke {
            feature,
            background,
            scenario_definition,
            name,
            language,
            background_steps,
            scenario_steps,
            tags,
            locations,
        };

        cukes.push(cuke);
    }

    fn compile_scenario_steps<'d>(
        &mut self,
        background_steps: &[cuke::Step<'d>],
        scenario: &'d ast::Scenario,
    ) -> (Vec<cuke::Step<'d>>, Vec<cuke::Step<'d>>) {
        if scenario.steps.is_empty() {
            (Vec::new(), Vec::new())
        } else {
            let mut scenario_steps = Vec::with_capacity(scenario.steps.len());

            for step in &scenario.steps {
                let cuke_step = self.cuke_step(step);
                scenario_steps.push(cuke_step);
            }

            (background_steps.to_vec(), scenario_steps)
        }
    }

    fn compile_scenario_tags<'d>(
        &mut self,
        feature: &'d ast::Feature,
        scenario: &'d ast::Scenario,
    ) -> Vec<cuke::Tag<'d>> {
        let feature_tags = &feature.tags;
        let scenario_tags = &scenario.tags;
        let tags_capacity = feature_tags.len() + scenario_tags.len();

        let mut tags = Vec::with_capacity(tags_capacity);

        feature_tags
            .iter()
            .map(cuke::Tag::from)
            .for_each(|tag| tags.push(tag));
        scenario_tags
            .iter()
            .map(cuke::Tag::from)
            .for_each(|tag| tags.push(tag));

        tags
    }

    fn compile_scenario_outline<'d>(
        &mut self,
        cukes: &mut Vec<cuke::Cuke<'d>>,
        feature: &'d ast::Feature,
        background: Option<&'d ast::Background>,
        background_steps: &[cuke::Step<'d>],
        scenario_outline: &'d ast::ScenarioOutline,
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

                let scenario_definition = cuke::ScenarioDefinition::from(scenario_outline);
                let name = self.interpolate(&scenario_outline.name, variable_cells, value_cells);
                let language = &feature.language;
                let (background_steps, scenario_steps) = self.compile_scenario_outline_steps(
                    background_steps,
                    scenario_outline,
                    variable_cells,
                    value_cells,
                    values,
                );
                let tags = self.compile_scenario_outline_tags(&feature, scenario_outline, examples);
                let locations = vec![
                    cuke::Location::from(values.location),
                    cuke::Location::from(scenario_outline.location),
                ];
                let cuke = cuke::Cuke {
                    feature,
                    background,
                    scenario_definition,
                    name,
                    language,
                    background_steps,
                    scenario_steps,
                    tags,
                    locations,
                };

                cukes.push(cuke);
            }
        }
    }

    fn compile_scenario_outline_steps<'d>(
        &mut self,
        background_steps: &[cuke::Step<'d>],
        scenario_outline: &'d ast::ScenarioOutline,
        variable_cells: &'d [ast::TableCell],
        value_cells: &'d [ast::TableCell],
        values: &'d ast::TableRow,
    ) -> (Vec<cuke::Step<'d>>, Vec<cuke::Step<'d>>) {
        if scenario_outline.steps.is_empty() {
            (Vec::new(), Vec::new())
        } else {
            let mut scenario_outline_steps = Vec::with_capacity(scenario_outline.steps.len());

            for step in &scenario_outline.steps {
                let keyword = &step.keyword;
                let text = self.interpolate(&step.text, variable_cells, value_cells);
                let argument =
                    self.create_cuke_argument(step.argument.as_ref(), variable_cells, value_cells);
                let locations = vec![
                    cuke::Location::from(values.location),
                    self.cuke_step_location(step),
                ];
                let cuke_step = cuke::Step {
                    keyword,
                    text,
                    argument,
                    locations,
                };

                scenario_outline_steps.push(cuke_step);
            }

            (background_steps.to_vec(), scenario_outline_steps)
        }
    }

    fn compile_scenario_outline_tags<'d>(
        &mut self,
        feature: &'d ast::Feature,
        scenario_outline: &'d ast::ScenarioOutline,
        examples: &'d ast::Examples,
    ) -> Vec<cuke::Tag<'d>> {
        let feature_tags = &feature.tags;
        let scenario_outline_tags = &scenario_outline.tags;
        let examples_tags = &examples.tags;
        let tags_capacity = feature_tags.len() + scenario_outline_tags.len() + examples_tags.len();

        let mut tags = Vec::with_capacity(tags_capacity);

        feature_tags
            .iter()
            .map(cuke::Tag::from)
            .for_each(|tag| tags.push(tag));
        scenario_outline_tags
            .iter()
            .map(cuke::Tag::from)
            .for_each(|tag| tags.push(tag));
        examples_tags
            .iter()
            .map(cuke::Tag::from)
            .for_each(|tag| tags.push(tag));

        tags
    }

    fn create_cuke_argument<'d>(
        &mut self,
        argument: Option<&'d ast::Argument>,
        variable_cells: &'d [ast::TableCell],
        value_cells: &'d [ast::TableCell],
    ) -> Option<cuke::Argument<'d>> {
        let argument = match argument {
            Some(argument) => argument,
            None => return None,
        };

        match argument {
            ast::Argument::DocString(doc_string) => {
                let location = cuke::Location::from(doc_string.location);
                let content = self.interpolate(&doc_string.content, variable_cells, value_cells);
                let content_type = doc_string.content_type.as_ref().map(|content_type| {
                    self.interpolate(content_type, variable_cells, value_cells)
                });
                let cuke_string = cuke::String {
                    location,
                    content,
                    content_type,
                };

                Some(cuke::Argument::String(cuke_string))
            }
            ast::Argument::DataTable(data_table) => {
                let rows = data_table
                    .rows
                    .iter()
                    .map(|row: &ast::TableRow| {
                        let cells = row
                            .cells
                            .iter()
                            .map(|cell: &ast::TableCell| {
                                let location = cuke::Location::from(cell.location);
                                let value =
                                    self.interpolate(&cell.value, variable_cells, value_cells);

                                cuke::Cell { location, value }
                            })
                            .collect::<Vec<cuke::Cell<'_>>>();

                        cuke::Row { cells }
                    })
                    .collect::<Vec<cuke::Row<'_>>>();
                let cuke_table = cuke::Table { rows };

                Some(cuke::Argument::Table(cuke_table))
            }
        }
    }

    fn background_cuke_steps<'d>(
        &mut self,
        background: &'d ast::Background,
    ) -> Vec<cuke::Step<'d>> {
        background
            .steps
            .iter()
            .map(|step| self.cuke_step(step))
            .collect()
    }

    fn cuke_step<'d>(&mut self, step: &'d ast::Step) -> cuke::Step<'d> {
        let keyword = &step.keyword;
        let text = Cow::Borrowed(step.text.as_str());
        let argument = self.create_cuke_argument(step.argument.as_ref(), &[], &[]);
        let locations = vec![self.cuke_step_location(step)];

        cuke::Step {
            keyword,
            text,
            argument,
            locations,
        }
    }

    fn interpolate<'d>(
        &mut self,
        text: &'d str,
        variable_cells: &'d [ast::TableCell],
        value_cells: &'d [ast::TableCell],
    ) -> Cow<'d, str> {
        if variable_cells.is_empty() {
            Cow::Borrowed(text)
        } else {
            let mut interpolated_text = text.to_owned();

            assert_eq!(variable_cells.len(), value_cells.len());
            for (column, variable_cell) in variable_cells.iter().enumerate() {
                let value_cell = &value_cells[column];
                let header = &variable_cell.value;
                let value = &value_cell.value;
                interpolated_text = interpolated_text.replace(&format!("<{}>", header), value);
            }

            Cow::Owned(interpolated_text)
        }
    }

    fn cuke_step_location(&mut self, step: &ast::Step) -> cuke::Location {
        let keyword_column = if step.keyword.is_empty() {
            0
        } else {
            step.keyword.chars().count() as u32
        };
        let step_location = step.location;
        let line = step_location.line;
        let column = step_location.column + keyword_column;
        cuke::Location { line, column }
    }
}
