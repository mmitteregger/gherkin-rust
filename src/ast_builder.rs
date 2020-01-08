use std::any::Any;
use std::default::Default;
use std::mem;

use crate::ast::*;
use crate::ast_node::AstNode;
use crate::error::{Error, Result};
use crate::parser::{self, Builder, RuleType, TokenType};
use crate::token::Token;

pub struct AstBuilder {
    stack: Vec<AstNode>,
    comments: Vec<Comment>,
}

impl Default for AstBuilder {
    fn default() -> AstBuilder {
        let mut ast_builder = AstBuilder {
            stack: Vec::new(),
            comments: Vec::new(),
        };
        ast_builder.reset();
        ast_builder
    }
}

impl parser::Builder for AstBuilder {
    type BuilderResult = GherkinDocument;

    fn build(&mut self, token: Token) -> Result<()> {
        let (rule_type, is_comment) = {
            let token_type = token.matched_type.unwrap();
            let rule_type = RuleType::from(token_type);
            let is_comment = token_type == TokenType::Comment;
            (rule_type, is_comment)
        };

        if is_comment {
            let location = self.get_location(&token, 0);
            let text = token.matched_text.as_ref().unwrap().clone();
            let comment = Comment::new(location, text);
            self.comments.push(comment);
        } else {
            self.current_node().add(rule_type, Box::new(token));
        }

        Ok(())
    }

    fn start_rule(&mut self, rule_type: RuleType) -> Result<()> {
        self.stack.push(AstNode::new(rule_type));
        Ok(())
    }

    fn end_rule(&mut self, _rule_type: RuleType) -> Result<()> {
        let node = self.stack.pop().unwrap();
        let rule_type = node.rule_type();

        let transformed_node = self.get_transformed_node(node)?;
        self.current_node().add(rule_type, transformed_node);

        Ok(())
    }

    fn get_result(&mut self) -> GherkinDocument {
        self.current_node().remove(RuleType::GherkinDocument)
    }

    fn reset(&mut self) {
        self.stack.clear();
        self.stack.push(AstNode::new(RuleType::None));

        self.comments.clear();
    }
}

impl AstBuilder {
    fn current_node(&mut self) -> &mut AstNode {
        self.stack
            .last_mut()
            .expect("current node on AstBuilder stack")
    }

    fn get_location(&self, token: &Token, column: u32) -> Location {
        let token_location = token.location.expect("token location");

        if column == 0 {
            token_location
        } else {
            Location::new(token_location.line, column)
        }
    }

    fn get_transformed_node(&mut self, mut node: AstNode) -> Result<Box<dyn Any>> {
        match node.rule_type() {
            RuleType::Step => {
                let step_line: Token = node.remove_token(TokenType::StepLine);

                let step_arg: Option<Argument> = {
                    let data_table: Option<DataTable> = node.remove_opt(RuleType::DataTable);
                    match data_table {
                        Some(data_table) => Some(Argument::from(data_table)),
                        None => {
                            let doc_string: Option<DocString> =
                                node.remove_opt(RuleType::DocString);
                            match doc_string {
                                Some(doc_string) => Some(Argument::from(doc_string)),
                                None => None,
                            }
                        }
                    }
                };

                let location = self.get_location(&step_line, 0);
                let keyword = step_line.matched_keyword.as_ref().unwrap().to_owned();
                let text = step_line.matched_text.as_ref().unwrap().to_owned();

                let step = Step::new(location, keyword, text, step_arg);
                Ok(Box::new(step))
            }
            RuleType::DocString => {
                let separator_tokens = node.remove_tokens(TokenType::DocStringSeparator);
                let separator_token = &separator_tokens[0];
                let separator_token_text = separator_token.matched_text.as_ref().unwrap();
                let content_type = if separator_token_text.chars().count() > 0 {
                    Some(separator_token_text.to_owned())
                } else {
                    None
                };
                let content = node
                    .remove_tokens(TokenType::Other)
                    .into_iter()
                    .map(|line_token| line_token.matched_text.as_ref().unwrap().to_owned())
                    .collect::<Vec<String>>()
                    .join("\n");
                let location = self.get_location(&separator_token, 0);

                let doc_string = DocString::new(location, content_type, content);
                Ok(Box::new(doc_string))
            }
            RuleType::DataTable => {
                let rows = self.get_table_rows(node)?;

                Ok(Box::new(DataTable::new(rows)))
            }
            RuleType::Background => {
                let background_line: Token = node.remove_token(TokenType::BackgroundLine);

                let description = self.get_description(&mut node);
                let steps = self.get_steps(&mut node);
                let location = self.get_location(&background_line, 0);
                let keyword = background_line.matched_keyword.as_ref().unwrap().to_owned();
                let name = background_line.matched_text.as_ref().unwrap().to_owned();

                let background = Background::new(location, keyword, name, description, steps);
                Ok(Box::new(background))
            }
            RuleType::ScenarioDefinition => {
                let tags = self.get_tags(&mut node);
                let mut scenario_node = node.remove::<AstNode>(RuleType::Scenario);
                let scenario_line = scenario_node.remove_token(TokenType::ScenarioLine);

                let location = self.get_location(&scenario_line, 0);
                let keyword = scenario_line.matched_keyword.as_ref().unwrap().to_owned();
                let name = scenario_line.matched_text.as_ref().unwrap().to_owned();
                let description = self.get_description(&mut scenario_node);
                let steps = self.get_steps(&mut scenario_node);
                let examples = scenario_node.remove_items(RuleType::ExamplesDefinition);

                let scenario_definition: ScenarioDefinition = if examples.is_empty() {
                    ScenarioDefinition::from(Scenario::new(
                        location,
                        keyword,
                        name,
                        description,
                        steps,
                        tags,
                    ))
                } else {
                    ScenarioDefinition::from(ScenarioOutline::new(
                        location,
                        keyword,
                        name,
                        description,
                        steps,
                        tags,
                        examples,
                    ))
                };

                Ok(Box::new(scenario_definition))
            }
            RuleType::ExamplesDefinition => {
                let tags = self.get_tags(&mut node);
                let mut examples_node: AstNode = node.remove(RuleType::Examples);
                let examples_line = examples_node.remove_token(TokenType::ExamplesLine);
                let description = self.get_description(&mut examples_node);
                let rows: Option<Vec<TableRow>> = examples_node.remove_opt(RuleType::ExamplesTable);
                let (table_header, table_body) = match rows {
                    Some(mut rows) => {
                        if rows.is_empty() {
                            (None, None)
                        } else {
                            let table_header = Some(rows.remove(0));
                            (table_header, Some(rows))
                        }
                    }
                    None => (None, None),
                };
                let location = self.get_location(&examples_line, 0);
                let keyword = examples_line.matched_keyword.as_ref().unwrap().to_owned();
                let name = examples_line.matched_text.as_ref().unwrap().to_owned();

                let examples = Examples::new(
                    location,
                    tags,
                    keyword,
                    name,
                    description,
                    table_header,
                    table_body,
                );
                Ok(Box::new(examples))
            }
            RuleType::ExamplesTable => {
                let rows = self.get_table_rows(node)?;
                Ok(Box::new(rows))
            }
            RuleType::Description => {
                let line_tokens = node.remove_tokens(TokenType::Other);

                let mut end = line_tokens.len();
                while end > 0
                    && line_tokens[end - 1]
                        .matched_text
                        .as_ref()
                        .unwrap()
                        .chars()
                        .all(|c| c.is_whitespace())
                {
                    end -= 1;
                }

                let line_tokens = &line_tokens[0..end];

                let description = line_tokens
                    .iter()
                    .map(|token| token.matched_text.as_ref().unwrap().to_owned())
                    .collect::<Vec<String>>()
                    .join("\n");

                Ok(Box::new(description))
            }
            RuleType::Feature => {
                let mut feature_header = node.remove(RuleType::FeatureHeader);
                let tags = self.get_tags(&mut feature_header);
                let feature_line = feature_header.remove_token(TokenType::FeatureLine);

                let mut scenario_definitions: Vec<ScenarioDefinition> = Vec::new();

                if let Some(background) = node.remove_opt::<Background>(RuleType::Background) {
                    scenario_definitions.push(ScenarioDefinition::from(background));
                }

                scenario_definitions
                    .extend(node.remove_items::<ScenarioDefinition>(RuleType::ScenarioDefinition));

                let location = self.get_location(&feature_line, 0);
                let language = feature_line
                    .matched_gherkin_dialect
                    .as_ref()
                    .unwrap()
                    .get_language()
                    .to_owned();
                let keyword = feature_line.matched_keyword.as_ref().unwrap().to_owned();
                let name = feature_line.matched_text.as_ref().unwrap().to_owned();
                let description = self.get_description(&mut feature_header);

                let feature = Feature::new(
                    location,
                    tags,
                    language,
                    keyword,
                    name,
                    description,
                    scenario_definitions,
                );
                Ok(Box::new(feature))
            }
            RuleType::GherkinDocument => {
                let feature: Option<Feature> = node.remove_opt(RuleType::Feature);
                let comments = mem::replace(&mut self.comments, Vec::new());

                let gherkin_document = GherkinDocument::new(feature, comments);
                Ok(Box::new(gherkin_document))
            }
            _ => Ok(Box::new(node)),
        }
    }

    fn get_table_rows(&self, mut node: AstNode) -> Result<Vec<TableRow>> {
        let rows: Vec<TableRow> = node
            .remove_tokens(TokenType::TableRow)
            .into_iter()
            .map(|token| {
                let location = self.get_location(&token, 0);
                let cells = self.get_cells(&token);
                TableRow::new(location, cells)
            })
            .collect();

        self.ensure_cell_count(&rows)?;

        Ok(rows)
    }

    fn ensure_cell_count(&self, rows: &[TableRow]) -> Result<()> {
        if rows.is_empty() {
            return Ok(());
        }

        let cell_count = rows[0].cells.len();

        for row in rows {
            if row.cells.len() != cell_count {
                return Err(Error::AstBuilder {
                    location: row.location,
                    message: "inconsistent cell count within the table".to_owned(),
                });
            }
        }

        Ok(())
    }

    fn get_cells(&self, token: &Token) -> Vec<TableCell> {
        token
            .matched_items
            .iter()
            .map(|cell_item| {
                let location = self.get_location(&token, cell_item.column);
                let text = cell_item.text.to_owned();
                TableCell::new(location, text)
            })
            .collect()
    }

    fn get_steps(&self, node: &mut AstNode) -> Vec<Step> {
        node.remove_items(RuleType::Step)
    }

    fn get_description(&self, node: &mut AstNode) -> Option<String> {
        node.remove_opt(RuleType::Description)
    }

    fn get_tags(&self, node: &mut AstNode) -> Vec<Tag> {
        let default_tags_node = AstNode::new(RuleType::None);
        let mut tags_node = node.remove_or(RuleType::Tags, default_tags_node);

        let mut tokens = tags_node.remove_tokens(TokenType::TagLine);

        let mut tags = Vec::new();
        for token in tokens.iter_mut() {
            let tag_items = mem::replace(&mut token.matched_items, Vec::new());
            for tag_item in tag_items {
                let location = self.get_location(&token, tag_item.column);
                tags.push(Tag::new(location, tag_item.text))
            }
        }

        tags
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ParserOptions;

    #[test]
    fn is_reusable() {
        let mut parser = ParserOptions::with_builder(AstBuilder::default()).create();

        let document_1 = parser.parse_str("Feature: 1").unwrap();
        let document_2 = parser.parse_str("Feature: 2").unwrap();

        assert_eq!(document_1.feature.unwrap().name, "1");
        assert_eq!(document_2.feature.unwrap().name, "2");
    }
}
