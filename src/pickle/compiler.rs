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
        unimplemented!();
    //    Vec<Pickle> pickles = Vec::new();
    //    Feature feature = gherkin_document.getFeature();
    //    if (feature == null) {
    //        return pickles;
    //    }
    //
    //    String language = feature.getLanguage();
    //    Vec<Tag> featureTags = feature.getTags();
    //    Vec<PickleStep> backgroundSteps = Vec::new();
    //
    //    for (ScenarioDefinition scenarioDefinition : feature.getChildren()) {
    //        if (scenarioDefinition instanceof Background) {
    //            backgroundSteps = pickleSteps(scenarioDefinition);
    //        } else if (scenarioDefinition instanceof Scenario) {
    //            compileScenario(pickles, backgroundSteps, (Scenario) scenarioDefinition, featureTags, language);
    //        } else {
    //            compileScenarioOutline(pickles, backgroundSteps, (ScenarioOutline) scenarioDefinition, featureTags, language);
    //        }
    //    }
    //    return pickles;
    }

    fn compile_scenario(&mut self, pickles: Vec<Pickle>, background_steps: Vec<PickleStep>,
        scenario: Scenario, feature_tags: Vec<Tag>, language: String) {
        unimplemented!();
    //    Vec<PickleStep> steps = Vec::new();
    //    if (!scenario.getSteps().isEmpty())
    //        steps.addAll(backgroundSteps);
    //
    //    Vec<Tag> scenarioTags = Vec::new();
    //    scenarioTags.addAll(featureTags);
    //    scenarioTags.addAll(scenario.getTags());
    //
    //    steps.addAll(pickleSteps(scenario));
    //
    //    Pickle pickle = Pickle::new(
    //            scenario.getName(),
    //            language,
    //            steps,
    //            pickleTags(scenarioTags),
    //            singletonVec(pickleLocation(scenario.getLocation()))
    //    );
    //    pickles.add(pickle);
    }

    fn compile_scenario_outline(&mut self, pickles: Vec<Pickle>, background_steps: Vec<PickleStep>,
        scenario_outline: ScenarioOutline, feature_tags: Vec<Tag>, language: String) {
        unimplemented!();
    //    for (final Examples examples : scenarioOutline.getExamples()) {
    //        if (examples.getTableHeader() == null) continue;
    //        Vec<TableCell> variableCells = examples.getTableHeader().getCells();
    //        for (final TableRow values : examples.getTableBody()) {
    //            Vec<TableCell> valueCells = values.getCells();
    //
    //            Vec<PickleStep> steps = Vec::new();
    //            if (!scenarioOutline.getSteps().isEmpty())
    //                steps.addAll(backgroundSteps);
    //
    //            Vec<Tag> tags = Vec::new();
    //            tags.addAll(featureTags);
    //            tags.addAll(scenarioOutline.getTags());
    //            tags.addAll(examples.getTags());
    //
    //            for (Step scenarioOutlineStep : scenarioOutline.getSteps()) {
    //                String stepText = interpolate(scenarioOutlineStep.getText(), variableCells, valueCells);
    //
    //                // TODO: Use an Array of location in DataTable/DocString as well.
    //                // If the Gherkin AST classes supported
    //                // a Vec of locations, we could just reuse the same classes
    //
    //                PickleStep pickleStep = PickleStep::new(
    //                        stepText,
    //                        createPickleArguments(scenarioOutlineStep.getArgument(), variableCells, valueCells),
    //                        asVec(
    //                                pickleLocation(values.getLocation()),
    //                                pickleStepLocation(scenarioOutlineStep)
    //                        )
    //                );
    //                steps.add(pickleStep);
    //            }
    //
    //            Pickle pickle = Pickle::new(
    //                    interpolate(scenarioOutline.getName(), variableCells, valueCells),
    //                    language,
    //                    steps,
    //                    pickleTags(tags),
    //                    asVec(
    //                            pickleLocation(values.getLocation()),
    //                            pickleLocation(scenarioOutline.getLocation())
    //                    )
    //            );
    //
    //            pickles.add(pickle);
    //        }
    //    }
    }

    fn create_pickle_arguments(&mut self, argument: Box<Node>) -> Vec<Box<Argument>> {
        unimplemented!();
        self.create_pickle_arguments_with_cells(argument, Vec::new(), Vec::new())
    }

    fn create_pickle_arguments_with_cells(&mut self, argument: Box<Node>,
        variable_cells: Vec<TableCell>, value_cells: Vec<TableCell>) -> Vec<Box<Argument>> {
        unimplemented!();
    //    Vec<Argument> result = Vec::new();
    //    if (argument == null) return result;
    //    if (argument instanceof DataTable) {
    //        DataTable t = (DataTable) argument;
    //        Vec<TableRow> rows = t.getRows();
    //        Vec<PickleRow> newRows = Vec::new<>(rows.size());
    //        for (TableRow row : rows) {
    //            Vec<TableCell> cells = row.getCells();
    //            Vec<PickleCell> newCells = Vec::new();
    //            for (TableCell cell : cells) {
    //                newCells.add(
    //                        PickleCell::new(
    //                                pickleLocation(cell.getLocation()),
    //                                interpolate(cell.getValue(), variableCells, valueCells)
    //                        )
    //                );
    //            }
    //            newRows.add(PickleRow::new(newCells));
    //        }
    //        result.add(PickleTable::new(newRows));
    //    } else if (argument instanceof DocString) {
    //        DocString ds = (DocString) argument;
    //        result.add(
    //                PickleString::new(
    //                        pickleLocation(ds.getLocation()),
    //                        interpolate(ds.getContent(), variableCells, valueCells),
    //                        ds.getContentType() == null ? null : interpolate(ds.getContentType(), variableCells, valueCells)
    //                )
    //        );
    //    } else {
    //        throw RuntimeException::new("Unexpected argument type: " + argument);
    //    }
    //    return result;
    }

    fn pickle_steps(&mut self, scenario_definition: Box<ScenarioDefinition>) -> Vec<PickleStep> {
        scenario_definition.get_steps().into_iter()
            .map(|step| self.pickle_step(step))
            .collect()
    }

    fn pickle_step(&mut self, step: &Step) -> PickleStep {
        unimplemented!();
    //    return PickleStep::new(
    //            step.getText(),
    //            createPickleArguments(step.getArgument()),
    //            singletonVec(pickleStepLocation(step))
    //    );
    }

    fn interpolate(&mut self, name: String, variable_cells: Vec<TableCell>,
        value_cells: Vec<TableCell>) -> String {
        unimplemented!();
    //    int col = 0;
    //    for (TableCell variableCell : variableCells) {
    //        TableCell valueCell = valueCells.get(col++);
    //        String header = variableCell.getValue();
    //        String value = valueCell.getValue();
    //        name = name.replace("<" + header + ">", value);
    //    }
    //    return name;
    }

    fn pickle_step_location(&mut self, step: Step) -> PickleLocation {
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
