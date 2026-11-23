//! Every function outside the tests states whether it is constant or variable
//! time, as the timing contract of the crate requires.

/// Each function's doc comment, joined across lines, must name exactly one of
/// the two timing classes.
fn missing_timing_docs(source: &str) -> (usize, Vec<String>) {
    let mut docs = String::new();
    let mut count = 0;
    let mut missing = Vec::new();
    for line in source.lines().map(str::trim) {
        if line == "mod tests {" {
            break;
        }
        if let Some(comment) = line.strip_prefix("///") {
            docs.push_str(comment.trim());
            docs.push(' ');
            continue;
        }
        if line.starts_with("#[") || line.is_empty() || line.starts_with("//") {
            continue;
        }
        if ["pub fn ", "pub(crate) fn ", "fn "]
            .iter()
            .any(|prefix| line.starts_with(prefix))
        {
            count += 1;
            let docs = docs.to_ascii_lowercase();
            if docs.contains("constant time") == docs.contains("variable time") {
                missing.push(line.to_owned());
            }
        }
        docs.clear();
    }
    (count, missing)
}

#[test]
fn every_function_states_constant_or_variable_time() {
    for (name, source, minimum) in [
        ("error.rs", include_str!("../src/error.rs"), 1),
        (
            "key_pair_generator.rs",
            include_str!("../src/key_pair_generator.rs"),
            6,
        ),
        (
            "key_generation_params.rs",
            include_str!("../src/params/key_generation_params.rs"),
            2,
        ),
        (
            "private_key.rs",
            include_str!("../src/params/private_key.rs"),
            5,
        ),
        (
            "public_key.rs",
            include_str!("../src/params/public_key.rs"),
            4,
        ),
    ] {
        let (count, missing) = missing_timing_docs(source);
        assert!(count >= minimum, "only {count} functions scanned in {name}");
        assert!(
            missing.is_empty(),
            "missing timing docs in {name}: {missing:?}"
        );
    }
}
