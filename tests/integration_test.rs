extern crate nice_rust;

#[test]
fn integration_niceonly_benchmark() {
    nice_rust::run(
        nice_rust::Mode::Niceonly,
        "https://nicenumbers.net/api".to_string(),
        "anonymous".to_string(),
        false,
        false,
        true,
        None,
        Some(1000000),
        None,
    );
}

#[test]
fn integration_detailed_benchmark() {
    nice_rust::run(
        nice_rust::Mode::Detailed,
        "https://nicenumbers.net/api".to_string(),
        "anonymous".to_string(),
        false,
        false,
        true,
        None,
        Some(100000),
        None,
    );
}

#[test]
fn integration_niceonly() {
    nice_rust::run(
        nice_rust::Mode::Niceonly,
        "https://nicenumbers.net/api".to_string(),
        "anonymous".to_string(),
        false,
        false,
        false,
        None,
        Some(1000000),
        None,
    );
}

#[test]
fn integration_detailed() {
    nice_rust::run(
        nice_rust::Mode::Detailed,
        "https://nicenumbers.net/api".to_string(),
        "anonymous".to_string(),
        false,
        false,
        false,
        None,
        Some(100000),
        None,
    );
}