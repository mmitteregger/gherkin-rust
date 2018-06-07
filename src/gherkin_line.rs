use constant;
use gherkin_line_span::GherkinLineSpan;

#[derive(Debug, Clone)]
pub struct GherkinLine {
    line_text: String,
    trimmed_line_text: String,
}

impl GherkinLine {
    pub fn new(line_text: String) -> GherkinLine {
        let trimmed_line_text = line_text.trim_left().to_string();

        GherkinLine {
            line_text,
            trimmed_line_text,
        }
    }

    pub fn indent(&self) -> u32 {
        self.line_text.chars().count() as u32 - self.trimmed_line_text.chars().count() as u32
    }

    pub fn detach(&self) {}

    pub fn get_line_text(&self, indent_to_remove: isize) -> &str {
        if indent_to_remove < 0 || indent_to_remove > self.indent() as isize {
            &self.trimmed_line_text
        } else {
            let mut chars = self.line_text.chars();
            for _ in 0..indent_to_remove {
                chars.next();
            }
            chars.as_str()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.trimmed_line_text.is_empty()
    }

    pub fn starts_with(&self, prefix: &str) -> bool {
        self.trimmed_line_text.starts_with(prefix)
    }

    pub fn get_rest_trimmed(&self, length: usize) -> &str {
        let mut chars = self.trimmed_line_text.chars();
        for _ in 0..length {
            chars.next();
        }
        chars.as_str().trim()
    }

    pub fn get_tags(&self) -> Vec<GherkinLineSpan> {
        let mut line_spans: Vec<GherkinLineSpan> = Vec::new();

        let mut spans: Vec<(u32, String)> = Vec::new();
        let mut preceding_whitespace_count = 0;
        let mut span = String::new();

        for c in self.trimmed_line_text.chars() {
            if c.is_whitespace() {
                if !span.is_empty() {
                    spans.push((preceding_whitespace_count, span.clone()));
                    span.clear();
                    preceding_whitespace_count = 1;
                } else {
                    preceding_whitespace_count += 1;
                }
            } else {
                span.push(c);
            }
        }

        if !span.is_empty() {
            spans.push((preceding_whitespace_count, span));
        }

        let mut column = self.indent() + 1;
        for (preceding_whitespace_count, text) in spans {
            column += preceding_whitespace_count;
            let text_chars_count = text.chars().count() as u32;
            let span = GherkinLineSpan::new(column, text);
            line_spans.push(span);
            column += text_chars_count;
        }

        line_spans
    }

    pub fn starts_with_title_keyword(&self, text: &str) -> bool {
        let text_chars_count = text.chars().count();

        let mut chars = self.trimmed_line_text.chars();
        for _ in 0..text_chars_count {
            chars.next();
        }
        let separator_chars_count = constant::TITLE_KEYWORD_SEPARATOR.chars().count();
        let separator: String = chars.take(separator_chars_count).collect();

        self.trimmed_line_text.chars().count() > text_chars_count
            && self.trimmed_line_text.starts_with(text)
            && separator == constant::TITLE_KEYWORD_SEPARATOR
    }

    pub fn get_table_cells(&self) -> Vec<GherkinLineSpan> {
        let mut line_spans: Vec<GherkinLineSpan> = Vec::new();
        let mut cell = String::new();
        let mut before_first = true;
        let mut start_col = 0;
        let mut after_backslash = false;

        for (col, c) in self.trimmed_line_text.chars().enumerate() {
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
                    let mut content_start = cell.chars()
                        .enumerate()
                        .skip_while(|(_index, cell_char)| cell_char.is_whitespace())
                        .map(|(index, _cell_char)| index)
                        .next()
                        .unwrap_or(0) as u32;

                    let column = self.indent() + start_col + content_start + 2;
                    let text = cell.trim().to_owned();
                    line_spans.push(GherkinLineSpan::new(column, text));

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

    #[test]
    fn get_tags() {
        let gherkin_line = GherkinLine::new("    @this @is  @atag  ".to_owned());
        let gherkin_line_spans = gherkin_line.get_tags();

        assert_eq!(
            gherkin_line_spans,
            vec![
                GherkinLineSpan::new(5, "@this".to_owned()),
                GherkinLineSpan::new(11, "@is".to_owned()),
                GherkinLineSpan::new(16, "@atag".to_owned()),
            ]
        );
    }
}
