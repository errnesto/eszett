use std::path::{Path, PathBuf};

use eszett::transformer;
use swc_core::{
    common::Mark, ecma::{transforms::{base::resolver, testing::test_fixture}, visit::visit_mut_pass}, testing
};
use swc_ecma_parser::{EsSyntax, Syntax};

fn syntax() -> Syntax {
    Syntax::Es(EsSyntax {
        jsx: true,
        ..Default::default()
    })
}

#[testing::fixture("tests/fixture/**/input.js")]
fn fix(input: PathBuf) {
    let output = input.with_file_name("output.js");
    test_fixture(
        syntax(),
        &|_| {
            let project_root = Path::new("project");
            let filepath = Path::new("project/file.js");

            (
                resolver(Mark::new(), Mark::new(), false),
                visit_mut_pass(transformer(project_root, filepath)),
            )
        },
        &input,
        &output,
        Default::default(),
    );
}
