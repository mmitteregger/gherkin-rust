use crate::constant;
use crate::{Error, Location, Result};

#[derive(Debug, Clone)]
pub struct Line {
    text: String,
    trimmed_text: String,
    ident: u32,
    line: u32,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct LineSpan {
    pub column: u32,
    pub text: String,
}

impl Line {
    pub fn new(text: String, line: u32) -> Line {
        let trimmed_text = text.trim_start().to_string();
        let ident = text.chars().count() as u32 - trimmed_text.chars().count() as u32;

        Line {
            text,
            trimmed_text,
            ident,
            line,
        }
    }

    pub fn indent(&self) -> u32 {
        self.ident
    }

    pub fn detach(&self) {}

    pub fn get_text(&self, indent_to_remove: isize) -> &str {
        if indent_to_remove < 0 || indent_to_remove > self.indent() as isize {
            &self.trimmed_text
        } else {
            let mut chars = self.text.chars();
            for _ in 0..indent_to_remove {
                chars.next();
            }
            chars.as_str()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.trimmed_text.is_empty()
    }

    pub fn starts_with(&self, prefix: &str) -> bool {
        self.trimmed_text.starts_with(prefix)
    }

    pub fn get_rest_trimmed(&self, length: usize) -> &str {
        let mut chars = self.trimmed_text.chars();
        for _ in 0..length {
            chars.next();
        }
        chars.as_str().trim()
    }

    pub fn get_tags(&self) -> Result<Vec<LineSpan>> {
        let mut tags = Vec::new();

        let mut uncommented_line_len = 0;
        let mut prev_was_whitespace = false;
        for char in self.trimmed_text.chars() {
            if !prev_was_whitespace {
                if char.is_whitespace() {
                    prev_was_whitespace = true;
                }
            } else if char == '#' {
                break;
            } else {
                prev_was_whitespace = false;
            }

            uncommented_line_len += char.len_utf8();
        }

        let uncommented_line = &self.trimmed_text[0..uncommented_line_len];
        let mut index_in_uncommented_line = 0;

        let elements = uncommented_line.split(constant::TAG_PREFIX);
        for element in elements {
            let token = element.trim_end();
            if token.is_empty() {
                continue;
            }
            let symbol_length = uncommented_line[0..index_in_uncommented_line]
                .chars()
                .count() as u32;
            let column = self.indent() + symbol_length + 1;
            if token.contains(|c: char| c.is_whitespace()) {
                return Err(Error::DocumentBuilder {
                    message: String::from("A tag may not contain whitespace"),
                    location: Location::new(self.line, column),
                });
            }

            let mut text = String::with_capacity(token.len() + 1);
            text.push_str(constant::TAG_PREFIX);
            text.push_str(token);

            tags.push(LineSpan { column, text });
            index_in_uncommented_line += element.len() + 1;
        }

        Ok(tags)
    }

    pub fn starts_with_title_keyword(&self, text: &str) -> bool {
        let text_chars_count = text.chars().count();

        let mut chars = self.trimmed_text.chars();
        for _ in 0..text_chars_count {
            chars.next();
        }
        let separator_chars_count = constant::TITLE_KEYWORD_SEPARATOR.chars().count();
        let separator: String = chars.take(separator_chars_count).collect();

        self.trimmed_text.chars().count() > text_chars_count
            && self.trimmed_text.starts_with(text)
            && separator == constant::TITLE_KEYWORD_SEPARATOR
    }

    pub fn get_table_cells(&self) -> Vec<LineSpan> {
        let mut line_spans: Vec<LineSpan> = Vec::new();
        let mut cell = String::new();
        let mut before_first = true;
        let mut start_col = 0;
        let mut after_backslash = false;

        for (col, c) in self.trimmed_text.chars().enumerate() {
            if after_backslash {
                if c == 'n' {
                    cell.push('\n');
                } else {
                    if c != '|' && c != '\\' {
                        cell.push('\\');
                    }
                    cell.push(c);
                }

                after_backslash = false;
            } else if c == '|' {
                if before_first {
                    // Skip the first empty span
                    before_first = false;
                } else {
                    let content_start = cell
                        .chars()
                        .enumerate()
                        .skip_while(|(_index, cell_char)| {
                            *cell_char != '\n' && cell_char.is_whitespace()
                        })
                        .map(|(index, _cell_char)| index)
                        .next()
                        .unwrap_or(0) as u32;

                    let column = self.indent() + start_col + content_start + 2;
                    let text = cell
                        .trim_matches(|c: char| c != '\n' && c.is_whitespace())
                        .to_owned();
                    line_spans.push(LineSpan { column, text });

                    start_col = col as u32;
                }
                cell.clear();
            } else if c == '\\' {
                after_backslash = true;
                continue;
            } else {
                cell.push(c);
            }
        }

        line_spans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_line_tags(line_text: &str) -> Result<Vec<LineSpan>> {
        Line::new(line_text.to_string(), 12).get_tags()
    }

    #[test]
    fn find_tags() {
        assert_eq!(
            get_line_tags("@this @is @a @tag").unwrap(),
            vec![
                LineSpan {
                    column: 1,
                    text: "@this".to_owned(),
                },
                LineSpan {
                    column: 7,
                    text: "@is".to_owned()
                },
                LineSpan {
                    column: 11,
                    text: "@a".to_owned()
                },
                LineSpan {
                    column: 14,
                    text: "@tag".to_owned()
                },
            ]
        );
    }

    #[test]
    fn error_on_tags_with_spaces() {
        assert!(get_line_tags("@this @is @a space separated @tag").is_err());
    }

    #[test]
    fn error_on_tags_with_leading_spaces() {
        assert!(get_line_tags("@ leadingSpace").is_err());
    }

    #[test]
    fn ignores_empty_tag() {
        assert!(get_line_tags("@").unwrap().is_empty(),);
    }

    #[test]
    fn ignores_empty_tags() {
        assert!(get_line_tags("@@").unwrap().is_empty(),);
    }

    #[test]
    fn finds_tags_trim_whitespace() {
        assert_eq!(
            get_line_tags("    @this @is  @a @tag  ").unwrap(),
            vec![
                LineSpan {
                    column: 5,
                    text: "@this".to_owned(),
                },
                LineSpan {
                    column: 11,
                    text: "@is".to_owned()
                },
                LineSpan {
                    column: 16,
                    text: "@a".to_owned()
                },
                LineSpan {
                    column: 19,
                    text: "@tag".to_owned()
                },
            ]
        );
    }

    #[test]
    fn finds_tags_comment_char_belonging_to_tag() {
        assert_eq!(
            get_line_tags("@this @is#not_a_comment  ").unwrap(),
            vec![
                LineSpan {
                    column: 1,
                    text: "@this".to_owned(),
                },
                LineSpan {
                    column: 7,
                    text: "@is#not_a_comment".to_owned()
                },
            ]
        );
    }

    #[test]
    fn finds_tags_comment_inside_tag() {
        assert_eq!(
            get_line_tags("@this @is #acomment  ").unwrap(),
            vec![
                LineSpan {
                    column: 1,
                    text: "@this".to_owned(),
                },
                LineSpan {
                    column: 7,
                    text: "@is".to_owned()
                },
            ]
        );
    }

    #[test]
    fn finds_tags_commented_before_tag() {
        assert_eq!(
            get_line_tags("@this @is #@a commented tag").unwrap(),
            vec![
                LineSpan {
                    column: 1,
                    text: "@this".to_owned(),
                },
                LineSpan {
                    column: 7,
                    text: "@is".to_owned()
                },
            ]
        );
    }

    #[test]
    fn finds_tags_commented_multiple_tags() {
        assert_eq!(
            get_line_tags("@this @is #@a @commented @sequence of tags").unwrap(),
            vec![
                LineSpan {
                    column: 1,
                    text: "@this".to_owned(),
                },
                LineSpan {
                    column: 7,
                    text: "@is".to_owned()
                },
            ]
        );
    }
}
