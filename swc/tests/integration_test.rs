use eszett::transformer;
use swc_core::{
    common::{chain, Mark},
    ecma::{
        transforms::{base::resolver, testing::test_inline},
        visit::{as_folder, Fold},
    },
};

fn transform() -> impl Fold {
    let filepath = "file.js";
    chain!(
        resolver(Mark::new(), Mark::new(), false),
        // Most of transform does not care about globals so it does not need `SyntaxContext`
        as_folder(transformer(filepath))
    )
}

test_inline!(
    Default::default(),
    |_| transform(),
    should_remove_sz_import_when_using_default_import,
    r#"import sz from 'eszett'"#,
    r#""#
);

test_inline!(
    Default::default(),
    |_| transform(),
    should_remove_sz_import_when_using_named_import,
    r#"import { scopeName } from 'eszett'"#,
    r#""#
);

test_inline!(
    Default::default(),
    |_| transform(),
    should_remove_sz_import_when_using_both_import,
    r#"import sz, { scopeName as foo } from 'eszett'"#,
    r#""#
);

test_inline!(
    Default::default(),
    |_| transform(),
    should_keep_other_imports,
    r#"import sz from 'some_import'"#,
    r#"import sz from 'some_import'"#
);

test_inline!(
    Default::default(),
    |_| transform(),
    should_replace_tagged_template_literals,
    r#"
        import sz from 'eszett'
        const hui = sz`my-class`
    "#,
    r#"
        const hui = "ß-file_js-0 " + `my-class`
    "#
);

test_inline!(
    Default::default(),
    |_| transform(),
    should_work_with_empty_template_literal,
    r#"
        import sz from 'eszett'
        const hui = sz``
    "#,
    r#"
        const hui = "ß-file_js-0 " + ``
    "#
);

test_inline!(
    Default::default(),
    |_| transform(),
    should_leave_non_sz_template_literals_alone,
    r#"
        import sz from 'eszett'
        const hui = css`my-class`
    "#,
    r#"
        const hui = css`my-class`
    "#
);

test_inline!(
    Default::default(),
    |_| transform(),
    should_create_a_new_scope_for_each_root_function,
    r#"
        import sz from 'eszett'
        function one() {
            const hui = sz`my-class`
        }
        function two() {
            const hui = sz`my-class`
        }
    "#,
    r#"
        function one() {
            const hui = "ß-file_js-1 " + `my-class`
        }
        function two() {
            const hui = "ß-file_js-2 " + `my-class`
        }
    "#
);

test_inline!(
    Default::default(),
    |_| transform(),
    should_use_the_same_scope_throughout_a_function_body,
    r#"
        import sz from 'eszett'
        function one() {
            const hui = sz`my-class`
            const buh = sz`my-class`
        }
    "#,
    r#"
        function one() {
            const hui = "ß-file_js-1 " + `my-class`
            const buh = "ß-file_js-1 " + `my-class`
        }
    "#
);

test_inline!(
    Default::default(),
    |_| transform(),
    should_use_the_same_scope_in_lexically_nested_functions,
    r#"
        import sz from 'eszett'
        function one() {
            const hui = sz`my-class`
            function two() {
                const buh = sz`my-class`
            }
        }
    "#,
    r#"
        function one() {
            const hui = "ß-file_js-1 " + `my-class`
            function two() {
                const buh = "ß-file_js-1 " + `my-class`
            }
        }
    "#
);

test_inline!(
    Default::default(),
    |_| transform(),
    should_create_a_new_scope_for_each_root_arrow_function,
    r#"
        import sz from 'eszett'
        const one = () => {
            const hui = sz`my-class`
        }
    "#,
    r#"
        const one = () => {
            const hui = "ß-file_js-1 " + `my-class`
        }
    "#
);

test_inline!(
    Default::default(),
    |_| transform(),
    should_use_the_same_scope_throughout_a_arrow_function_body,
    r#"
        import sz from 'eszett'
        const one = () => {
            const hui = sz`my-class`
            const buh = sz`my-class`
        }
    "#,
    r#"
        const one = () => {
            const hui = "ß-file_js-1 " + `my-class`
            const buh = "ß-file_js-1 " + `my-class`
        }
    "#
);

test_inline!(
    Default::default(),
    |_| transform(),
    should_use_the_same_scope_in_lexically_nested_arrow_functions,
    r#"
        import sz from 'eszett'
        const one = () => {
            const hui = sz`my-class`
            function two() {
                const buh = sz`my-class`
            }
        }
    "#,
    r#"
        const one = () => {
            const hui = "ß-file_js-1 " + `my-class`
            function two() {
                const buh = "ß-file_js-1 " + `my-class`
            }
        }
    "#
);

test_inline!(
    Default::default(),
    |_| transform(),
    should_replace_scope_name_variable_with_current_scope,
    r#"
        import { scopeName } from 'eszett'
        const scope = scopeName
    "#,
    r#"
        const scope = "ß-file_js-0"
    "#
);

test_inline!(
    Default::default(),
    |_| transform(),
    should_replace_scope_name_variable_when_renamed_in_import,
    r#"
        import { scopeName as sc } from 'eszett'
        const scope = sc
    "#,
    r#"
        const scope = "ß-file_js-0"
    "#
);

test_inline!(
    Default::default(),
    |_| transform(),
    should_not_replace_other_variables_with_the_scope_name_name,
    r#"
        import { scopeName as sc } from 'eszett'
        const scope = scopeName
    "#,
    r#"
        const scope = scopeName
    "#
);

test_inline!(
    Default::default(),
    |_| transform(),
    should_not_replace_variables_shaddowing_scope_name,
    r#"
        import { scopeName } from 'eszett'
        function hui() {
            const scopeName = 'lorem'
            const bar = scopeName
        }
    "#,
    r#"
        function hui() {
            const scopeName = 'lorem'
            const bar = scopeName
        }
    "#
);
