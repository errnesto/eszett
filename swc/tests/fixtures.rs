use std::path::PathBuf;

use eszett::transformer;
use swc_core::{
    common::{chain, Mark},
    ecma::{
        transforms::{base::resolver, testing::test_fixture},
        visit::{as_folder, Fold},
    },
    testing,
};

fn transform() -> impl Fold {
    let filepath = "file.js";
    chain!(
        resolver(Mark::new(), Mark::new(), false),
        // Most of transform does not care about globals so it does not need `SyntaxContext`
        as_folder(transformer(filepath))
    )
}

#[testing::fixture("tests/fixture/**/input.js")]
fn fix(input: PathBuf) {
    let output = input.with_file_name("output.js");
    test_fixture(
        Default::default(),
        &|_| transform(),
        &input,
        &output,
        Default::default(),
    );
}
