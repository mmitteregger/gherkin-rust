use std::borrow::Cow;

use cucumber_messages::ast;
use cucumber_messages::id_generator::IdGenerator;

use crate::cuke;

pub struct Compiler<'id_gen> {
    id_generator: &'id_gen mut dyn IdGenerator,
}

/// A session to store data for the compilation of a single `GherkinDocument`.
///
/// `'d` is the lifetime of the `GherkinDocument`.
struct Session<'d> {
    cukes: Vec<cuke::Cuke<'d>>,
    uri: &'d str,
    feature: &'d ast::Feature,
    feature_background: Option<&'d ast::Background>,
    feature_background_steps: Vec<cuke::Step<'d>>,
    rule: Option<&'d ast::Rule>,
    rule_background: Option<&'d ast::Background>,
    rule_background_steps: Vec<cuke::Step<'d>>,
}

impl<'id_gen> Compiler<'id_gen> {
    pub fn new(id_generator: &'id_gen mut dyn IdGenerator) -> Compiler<'id_gen> {
        Compiler { id_generator }
    }

    pub fn compile<'d>(
        &mut self,
        gherkin_document: &'d ast::GherkinDocument,
    ) -> Vec<cuke::Cuke<'d>> {
        let feature = match &gherkin_document.feature {
            Some(feature) => feature,
            None => return Vec::new(),
        };

        let mut session = Session {
            cukes: Vec::with_capacity(feature.children.len()),
            uri: &gherkin_document.uri,
            feature,
            feature_background: None,
            feature_background_steps: Vec::new(),
            rule: None,
            rule_background: None,
            rule_background_steps: Vec::new(),
        };
        self.compile_feature(&mut session);
        session.cukes
    }

    fn compile_feature<'d>(&mut self, session: &mut Session<'d>) {
        session.feature_background = None;
        session.feature_background_steps = Vec::new();

        for feature_child in &session.feature.children {
            if let Some(value) = &feature_child.value {
                match value {
                    ast::FeatureChildValue::Background(background) => {
                        session.feature_background = Some(background);
                        session.feature_background_steps = self.background_cuke_steps(background);
                    }
                    ast::FeatureChildValue::Rule(rule) => {
                        self.compile_rule(session, rule);
                    }
                    ast::FeatureChildValue::Scenario(scenario) => {
                        if scenario.examples.is_empty() {
                            self.compile_scenario(session, scenario);
                        } else {
                            self.compile_scenario_outline(session, scenario);
                        }
                    }
                }
            }
        }
    }

    fn compile_rule<'d>(&mut self, session: &mut Session<'d>, rule: &'d ast::Rule) {
        session.rule_background = None;
        session.rule_background_steps = Vec::new();

        for rule_child in &rule.children {
            if let Some(value) = &rule_child.value {
                match value {
                    ast::RuleChildValue::Background(background) => {
                        session.rule_background = Some(background);
                        session.rule_background_steps = self.background_cuke_steps(background);
                    }
                    ast::RuleChildValue::Scenario(scenario) => {
                        if scenario.examples.is_empty() {
                            self.compile_scenario(session, scenario);
                        } else {
                            self.compile_scenario_outline(session, scenario);
                        }
                    }
                }
            }
        }
    }

    fn compile_scenario<'d>(&mut self, session: &mut Session<'d>, scenario: &'d ast::Scenario) {
        let name = Cow::Borrowed(scenario.name.as_str());
        let language = &session.feature.language;
        let (feature_background_steps, rule_background_steps) =
            self.compile_feature_and_rule_background_steps(session, !scenario.steps.is_empty());
        let scenario_steps = self.compile_scenario_steps(scenario);
        let tags = self.compile_scenario_tags(session, scenario);
        let locations = vec![cuke::Location::from(scenario.location.unwrap())];
        let ast_node_ids = vec![scenario.id.as_str()];
        let cuke = cuke::Cuke {
            id: self.id_generator.new_id(),
            uri: session.uri,
            feature: session.feature,
            feature_background: session.feature_background,
            rule: session.rule,
            rule_background: session.rule_background,
            scenario,
            name,
            language,
            feature_background_steps,
            rule_background_steps,
            scenario_steps,
            tags,
            locations,
            ast_node_ids,
        };

        session.cukes.push(cuke);
    }

    fn compile_scenario_steps<'d>(&mut self, scenario: &'d ast::Scenario) -> Vec<cuke::Step<'d>> {
        let mut scenario_steps = Vec::with_capacity(scenario.steps.len());

        for step in &scenario.steps {
            let cuke_step = self.cuke_step(step);
            scenario_steps.push(cuke_step);
        }

        scenario_steps
    }

    fn compile_scenario_tags<'d>(
        &mut self,
        session: &mut Session<'d>,
        scenario: &'d ast::Scenario,
    ) -> Vec<cuke::Tag<'d>> {
        let feature_tags = &session.feature.tags;
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
        session: &mut Session<'d>,
        scenario: &'d ast::Scenario,
    ) {
        for examples in &scenario.examples {
            let table_header: &ast::TableRow = match &examples.table_header {
                Some(table_header) => table_header,
                None => return,
            };

            let variable_cells = &table_header.cells;
            for values in &examples.table_body {
                let value_cells = &values.cells;

                let name = self.interpolate(&scenario.name, variable_cells, value_cells);
                let language = &session.feature.language;
                let (feature_background_steps, rule_background_steps) = self
                    .compile_feature_and_rule_background_steps(session, !scenario.steps.is_empty());
                let scenario_steps = self.compile_scenario_outline_steps(
                    scenario,
                    variable_cells,
                    value_cells,
                    values,
                );
                let tags = self.compile_scenario_outline_tags(session, scenario, examples);
                let locations = vec![
                    cuke::Location::from(values.location.unwrap()),
                    cuke::Location::from(scenario.location.unwrap()),
                ];
                let ast_node_ids = vec![scenario.id.as_str(), values.id.as_str()];
                let cuke = cuke::Cuke {
                    id: self.id_generator.new_id(),
                    uri: session.uri,
                    feature: session.feature,
                    feature_background: session.feature_background,
                    rule: session.rule,
                    rule_background: session.rule_background,
                    scenario,
                    name,
                    language,
                    feature_background_steps,
                    rule_background_steps,
                    scenario_steps,
                    tags,
                    locations,
                    ast_node_ids,
                };

                session.cukes.push(cuke);
            }
        }
    }

    fn compile_scenario_outline_steps<'d>(
        &mut self,
        scenario: &'d ast::Scenario,
        variable_cells: &'d [ast::TableCell],
        value_cells: &'d [ast::TableCell],
        values: &'d ast::TableRow,
    ) -> Vec<cuke::Step<'d>> {
        if scenario.steps.is_empty() {
            Vec::new()
        } else {
            let mut steps = Vec::with_capacity(scenario.steps.len());

            for step in &scenario.steps {
                let keyword = &step.keyword;
                let text = self.interpolate(&step.text, variable_cells, value_cells);
                let argument =
                    self.create_cuke_argument(step.argument.as_ref(), variable_cells, value_cells);
                let locations = vec![
                    cuke::Location::from(values.location.unwrap()),
                    self.cuke_step_location(step),
                ];
                let ast_node_ids = vec![step.id.as_str(), values.id.as_str()];
                let cuke_step = cuke::Step {
                    id: self.id_generator.new_id(),
                    keyword,
                    text,
                    argument,
                    locations,
                    ast_node_ids,
                };

                steps.push(cuke_step);
            }

            steps
        }
    }

    fn compile_scenario_outline_tags<'d>(
        &mut self,
        session: &mut Session<'d>,
        scenario: &'d ast::Scenario,
        examples: &'d ast::Examples,
    ) -> Vec<cuke::Tag<'d>> {
        let feature_tags = &session.feature.tags;
        let scenario_outline_tags = &scenario.tags;
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
                let location = cuke::Location::from(doc_string.location.unwrap());
                let content = self.interpolate(&doc_string.content, variable_cells, value_cells);
                let media_type =
                    self.interpolate(&doc_string.media_type, variable_cells, value_cells);
                let cuke_string = cuke::String {
                    location,
                    content,
                    media_type,
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
                                let location = cuke::Location::from(cell.location.unwrap());
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
            .map(|step| self.cuke_step_without_id(step))
            .collect()
    }

    fn cuke_step<'d>(&mut self, step: &'d ast::Step) -> cuke::Step<'d> {
        let mut cuke_step = self.cuke_step_without_id(step);
        cuke_step.id = self.id_generator.new_id();
        cuke_step
    }

    fn cuke_step_without_id<'d>(&mut self, step: &'d ast::Step) -> cuke::Step<'d> {
        let keyword = &step.keyword;
        let text = Cow::Borrowed(step.text.as_str());
        let argument = self.create_cuke_argument(step.argument.as_ref(), &[], &[]);
        let locations = vec![self.cuke_step_location(step)];
        let ast_node_ids = vec![step.id.as_str()];

        cuke::Step {
            id: String::new(),
            keyword,
            text,
            argument,
            locations,
            ast_node_ids,
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
        let step_location = step.location.unwrap();
        let line = step_location.line;
        let column = step_location.column + keyword_column;
        cuke::Location { line, column }
    }

    fn compile_feature_and_rule_background_steps<'d>(
        &mut self,
        session: &mut Session<'d>,
        should_compile: bool,
    ) -> (Vec<cuke::Step<'d>>, Vec<cuke::Step<'d>>) {
        if should_compile {
            (
                self.recompile_steps(&session.feature_background_steps),
                self.recompile_steps(&session.rule_background_steps),
            )
        } else {
            (Vec::new(), Vec::new())
        }
    }

    fn recompile_steps<'d>(&mut self, steps: &[cuke::Step<'d>]) -> Vec<cuke::Step<'d>> {
        steps
            .iter()
            .map(|step| {
                let mut step = step.clone();
                step.id = self.id_generator.new_id();
                step
            })
            .collect()
    }
}
