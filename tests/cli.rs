use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

fn assert_default_output(output: &str) {
    assert!(output.contains("Rusty Search"));
    assert!(output.contains("Search time:"));
    assert!(output.contains("Total results:"));
}

#[test]
fn run_search_default() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rusty-search")?;

    let search_query = "foo";

    let output = cmd.arg(&search_query).output()?;
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[1]"));

    let output_str = String::from_utf8(output.stdout).unwrap();
    assert_default_output(&output_str);

    Ok(())
}

#[test]
fn run_search_num_count_5() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rusty-search")?;

    let search_query = "foo";
    let num_count = "5";

    let output = cmd.arg(&search_query).arg("-n").arg(&num_count).output()?;
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[1]"))
        .stdout(predicate::str::contains("[5]"));

    let output_str = String::from_utf8(output.stdout).unwrap();
    assert_default_output(&output_str);

    Ok(())
}

#[test]
fn run_search_no_result_found() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rusty-search")?;

    let search_query = "65a00b9e-bd14-45a1-b1c1-5ed7b1042707sdsdsadasdsadsadasdasdasdadadasdadas";

    let output = cmd.arg(&search_query).output()?;
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("[0] No results found"))
        .stdout(predicate::str::contains("Total results: 0"));

    let output_str = String::from_utf8(output.stdout).unwrap();
    assert_default_output(&output_str);

    Ok(())
}

#[test]
fn run_search_empty_search_query() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rusty-search")?;

    let search_query = "";

    let output = cmd.arg(&search_query).output()?;
    cmd.assert().failure().stderr(predicate::str::contains(
        "Error: Failed to get response, status code: 400 Bad Request",
    ));

    let output_str = String::from_utf8(output.stdout)?;
    assert!(output_str.contains("Rusty Search"));
    assert!(!output_str.contains("Search time:"));
    assert!(!output_str.contains("Total results:"));

    Ok(())
}
