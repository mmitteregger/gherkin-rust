extern crate gherkin;

use gherkin::event::{self, SourceEvent, GherkinDocumentEvent, PickleEvent};

#[test]
fn generates_events_for_english_by_default() {
    let data = "Feature: Hello\n  Scenario: World\n    Given hello";
    let uri = "features/hello.feature";

    let events = event::generate(data, uri).unwrap();

    assert!(events[0].downcast_ref::<SourceEvent>().is_some());
    assert!(events[1].downcast_ref::<GherkinDocumentEvent>().is_some());
    assert!(events[2].downcast_ref::<PickleEvent>().is_some());
}

#[test]
fn generates_events_for_specific_language() {
    let data = "Fonctionnalité: Bonjour\n  Scénario: Monde\n    Soit une étape";
    let  uri = "features/hello.feature";

    let events = event::generate_with_language(data, uri, "fr").unwrap();

    assert!(events[0].downcast_ref::<SourceEvent>().is_some());
    assert!(events[1].downcast_ref::<GherkinDocumentEvent>().is_some());
    assert!(events[2].downcast_ref::<PickleEvent>().is_some());
}
