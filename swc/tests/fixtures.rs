use std::path::{Path, PathBuf};

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
    let project_root = Path::new("project");
    let filepath = Path::new("project/file.js");
    chain!(
        resolver(Mark::new(), Mark::new(), false),
        as_folder(transformer(project_root, filepath))
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
