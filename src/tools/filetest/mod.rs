//! File tests.
//!
//! This module contains the main driver for `cton-util test` as well as implementations of the
//! available test commands.

use std::path::Path;
use cton_reader::TestCommand;
use CommandResult;
use cat;
use print_cfg;
use filetest::runner::TestRunner;

pub mod subtest;
mod runner;
mod domtree;
mod verifier;

/// Main entry point for `cton-util test`.
///
/// Take a list of filenames which can be either `.cton` files or directories.
///
/// Files are interpreted as test cases and executed immediately.
///
/// Directories are scanned recursively for test cases ending in `.cton`. These test cases are
/// executed on background threads.
///
pub fn run(files: Vec<String>) -> CommandResult {
    let mut runner = TestRunner::new();

    for path in files.iter().map(Path::new) {
        if path.is_file() {
            runner.push_test(path);
        } else {
            runner.push_dir(path);
        }
    }

    runner.run()
}

/// Create a new subcommand trait object to match `parsed.command`.
///
/// This function knows how to create all of the possible `test <foo>` commands that can appear in
/// a .cton test file.
fn new_subtest(parsed: &TestCommand) -> subtest::Result<Box<subtest::SubTest>> {
    match parsed.command {
        "cat" => cat::subtest(parsed),
        "print-cfg" => print_cfg::subtest(parsed),
        "domtree" => domtree::subtest(parsed),
        "verifier" => verifier::subtest(parsed),
        _ => Err(format!("unknown test command '{}'", parsed.command)),
    }
}
