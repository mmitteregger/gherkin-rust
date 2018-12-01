extern crate gherkin;

use gherkin::event::{self, CucumberEvent};

#[test]
fn generates_events_for_english_by_default() {
    let data = "Feature: Hello\n  Scenario: World\n    Given hello";
    let uri = "features/hello.feature";

    let events = event::generate(data, uri).unwrap();

    assert!(match events[0] { CucumberEvent::Source(..) => true, _ => false});
    assert!(match events[1] { CucumberEvent::GherkinDocument(..) => true, _ => false});
    assert!(match events[2] { CucumberEvent::Pickle(..) => true, _ => false});
}

#[test]
fn generates_events_for_specific_language() {
    let data = "Fonctionnalité: Bonjour\n  Scénario: Monde\n    Soit une étape";
    let uri = "features/hello.feature";

    let events = event::generate_with_language(data, uri, "fr").unwrap();

    assert!(match events[0] { CucumberEvent::Source(..) => true, _ => false});
    assert!(match events[1] { CucumberEvent::GherkinDocument(..) => true, _ => false});
    assert!(match events[2] { CucumberEvent::Pickle(..) => true, _ => false});
}
