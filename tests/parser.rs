extern crate gherkin;
extern crate serde_json;

use gherkin::{Parser, TokenMatcher};
use gherkin::pickle::Compiler;

#[test]
fn parse_feature_after_parse_error() {
    let source1 = r###"
# a comment
Feature: Foo
  Scenario: Bar
    Given x
      ```
      unclosed docstring
"###;
    let source2 = r###"
Feature: Foo
  Scenario: Bar
    Given x
      """
      closed docstring
      """
"###;
    let mut matcher = TokenMatcher::default();
    let mut parser = Parser::default();

    if let Ok(_) = parser.parse_str_with_matcher(source1, &mut matcher) {
        panic!("Error expected");
    }

    let gherkin_document = parser.parse_str_with_matcher(source2, &mut matcher)
            .unwrap();
    let gherkin_document_json = serde_json::to_string_pretty(&gherkin_document)
        .unwrap();

    assert_eq!(r###"{
  "type": "GherkinDocument",
  "feature": {
    "type": "Feature",
    "location": {
      "line": 2,
      "column": 1
    },
    "tags": [],
    "language": "en",
    "keyword": "Feature",
    "name": "Foo",
    "children": [
      {
        "type": "Scenario",
        "location": {
          "line": 3,
          "column": 3
        },
        "keyword": "Scenario",
        "name": "Bar",
        "steps": [
          {
            "type": "Step",
            "location": {
              "line": 4,
              "column": 5
            },
            "keyword": "Given ",
            "text": "x",
            "argument": {
              "type": "DocString",
              "location": {
                "line": 5,
                "column": 7
              },
              "content": "closed docstring"
            }
          }
        ],
        "tags": []
      }
    ]
  },
  "comments": []
}"###,
        &gherkin_document_json);
}

#[test]
fn change_default_language() {
    let source = "Egenskap: i18n support\n";
    let mut matcher = TokenMatcher::with_default_dialect_name("no");
    let mut parser = Parser::default();

    let gherkin_document = parser.parse_str_with_matcher(source, &mut matcher)
        .unwrap();
    let gherkin_document_json = serde_json::to_string_pretty(&gherkin_document)
        .unwrap();

    assert_eq!(r###"{
  "type": "GherkinDocument",
  "feature": {
    "type": "Feature",
    "location": {
      "line": 1,
      "column": 1
    },
    "tags": [],
    "language": "no",
    "keyword": "Egenskap",
    "name": "i18n support",
    "children": []
  },
  "comments": []
}"###,
        &gherkin_document_json);
}

#[test]
fn doc_string_content_type() {
    let source = r###"
Feature: Foo
  Scenario: Bar
    Given x
      """json
      {}
      """
"###;
    let mut matcher = TokenMatcher::default();
    let mut parser = Parser::default();

    let gherkin_document = parser.parse_str_with_matcher(source, &mut matcher)
        .unwrap();
    let gherkin_document_json = serde_json::to_string_pretty(&gherkin_document)
        .unwrap();

    assert_eq!(r###"{
  "type": "GherkinDocument",
  "feature": {
    "type": "Feature",
    "location": {
      "line": 2,
      "column": 1
    },
    "tags": [],
    "language": "en",
    "keyword": "Feature",
    "name": "Foo",
    "children": [
      {
        "type": "Scenario",
        "location": {
          "line": 3,
          "column": 3
        },
        "keyword": "Scenario",
        "name": "Bar",
        "steps": [
          {
            "type": "Step",
            "location": {
              "line": 4,
              "column": 5
            },
            "keyword": "Given ",
            "text": "x",
            "argument": {
              "type": "DocString",
              "location": {
                "line": 5,
                "column": 7
              },
              "contentType": "json",
              "content": "{}"
            }
          }
        ],
        "tags": []
      }
    ]
  },
  "comments": []
}"###,
        &gherkin_document_json);

    let mut compiler = Compiler::default();
    let pickles = compiler.compile(&gherkin_document);
    let pickles_json = serde_json::to_string_pretty(&pickles).unwrap();

    assert_eq!(r###"[
  {
    "name": "Bar",
    "language": "en",
    "steps": [
      {
        "text": "x",
        "arguments": [
          {
            "location": {
              "line": 5,
              "column": 7
            },
            "content": "{}",
            "contentType": "json"
          }
        ],
        "locations": [
          {
            "line": 4,
            "column": 11
          }
        ]
      }
    ],
    "tags": [],
    "locations": [
      {
        "line": 3,
        "column": 3
      }
    ]
  }
]"###,
        &pickles_json);
}
