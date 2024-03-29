extern crate nice_rust;

#[test]
fn integration_niceonly_integer_benchmark() {
    nice_rust::run(
        nice_rust::Mode::Niceonly,
        "https://nicenumbers.net/api".to_string(),
        "anonymous".to_string(),
        false,
        false,
        true,
        true,
        false,
        None,
        Some(1000000),
        None,
    );
}
#[test]
fn integration_niceonly_natural_benchmark() {
    nice_rust::run(
        nice_rust::Mode::Niceonly,
        "https://nicenumbers.net/api".to_string(),
        "anonymous".to_string(),
        false,
        false,
        true,
        true,
        true,
        None,
        Some(1000000),
        None,
    );
}

#[test]
fn integration_detailed_integer_benchmark() {
    nice_rust::run(
        nice_rust::Mode::Detailed,
        "https://nicenumbers.net/api".to_string(),
        "anonymous".to_string(),
        false,
        false,
        true,
        true,
        false,
        None,
        Some(100000),
        None,
    );
}
#[test]
fn integration_detailed_natural_benchmark() {
    nice_rust::run(
        nice_rust::Mode::Detailed,
        "https://nicenumbers.net/api".to_string(),
        "anonymous".to_string(),
        false,
        false,
        true,
        true,
        true,
        None,
        Some(100000),
        None,
    );
}

#[test]
fn integration_niceonly_integer_standard() {
    nice_rust::run(
        nice_rust::Mode::Niceonly,
        "https://nicenumbers.net/api".to_string(),
        "anonymous".to_string(),
        false,
        false,
        false,
        true,
        false,
        None,
        Some(1000000),
        None,
    );
}
#[test]
fn integration_niceonly_natural_standard() {
    nice_rust::run(
        nice_rust::Mode::Niceonly,
        "https://nicenumbers.net/api".to_string(),
        "anonymous".to_string(),
        false,
        false,
        false,
        true,
        true,
        None,
        Some(1000000),
        None,
    );
}

#[test]
fn integration_detailed_integer_standard() {
    nice_rust::run(
        nice_rust::Mode::Detailed,
        "https://nicenumbers.net/api".to_string(),
        "anonymous".to_string(),
        false,
        false,
        false,
        true,
        false,
        None,
        Some(100000),
        None,
    );
}
#[test]
fn integration_detailed_natural_standard() {
    nice_rust::run(
        nice_rust::Mode::Detailed,
        "https://nicenumbers.net/api".to_string(),
        "anonymous".to_string(),
        false,
        false,
        false,
        true,
        true,
        None,
        Some(100000),
        None,
    );
}

#[test]
fn integration_niceonly_natural_b120() {
    nice_rust::run(
        nice_rust::Mode::Niceonly,
        "https://nicenumbers.net/api".to_string(),
        "anonymous".to_string(),
        false,
        false,
        false,
        true,
        true,
        Some(120),
        Some(1000000),
        None,
    );
}

#[test]
fn integration_detailed_natural_b120() {
    nice_rust::run(
        nice_rust::Mode::Detailed,
        "https://nicenumbers.net/api".to_string(),
        "anonymous".to_string(),
        false,
        false,
        false,
        true,
        true,
        Some(120),
        Some(100000),
        None,
    );
}
