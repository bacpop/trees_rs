use std::io::Write;
use regex::Regex;

use snapbox::cmd::{cargo_bin, Command};
use float_cmp::assert_approx_eq;

pub mod common;
use crate::common::*;

#[cfg(test)]
use pretty_assertions::assert_eq;

#[test]
fn jc69_likelihood() {
    // Makes a temporary directory to work in
    let sandbox = TestSetup::setup();
    let input_alignment = sandbox.file_string("listeria0.phylip", TestDir::Input);

    // Likelihood and treefrom program
    // TODO when this can take input file use `.arg(sandbox.file_string("listeria0.aln", TestDir::Input))` to provide it
    let output = Command::new(cargo_bin("bactrees"))
        .current_dir(sandbox.get_wd())
        .output()
        .unwrap()
        .stdout;
    let output_string = String::from_utf8(output).unwrap();
    let output_parts: Vec<&str> = output_string.split("\n").collect();
    let likelihood: f64 = output_parts[0].parse().unwrap();
    let mut output_tr_file = sandbox.create_file("tree.nwk").unwrap();
    // Remove the quotes
    let mut tree_string = output_parts[1].to_string();
    tree_string.pop();
    tree_string.remove(0);
    writeln!(output_tr_file.0, "{tree_string}").unwrap();

    // Parallel alignment algorithm used
    // phyml -i <path_to_sequence> -u <path to newick tree> -m JC69 -o n -a 1
    let mut phyml_likelihood: f64 = 0.0;
    let phyml_out = Command::new("phyml")
        .current_dir(sandbox.get_wd())
        .args(&["-i", input_alignment.as_str(), "-u", output_tr_file.1.as_str(), "-m", "JC69", "-o", "n", "-a", "1"])
        .output()
        .unwrap()
        .stdout;
    let phyml_stdout = String::from_utf8(phyml_out).unwrap();

    let re = Regex::new(r"^\. Log likelihood of the current tree: (.+)\.$").unwrap();
    for phyml_outline in phyml_stdout.split("\n") {
        if let Some(caps) = re.captures(phyml_outline) {
            phyml_likelihood = caps.get(1).map_or("", |m| m.as_str()).parse().unwrap();
        }
    }

    assert_approx_eq!(f64, likelihood, phyml_likelihood, ulps = 2)
}