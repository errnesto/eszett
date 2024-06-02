use regex::Regex;
use swc_core::{
    atoms::Atom,
    common::{util::take::Take, DUMMY_SP},
    ecma::{
        ast::*,
        transforms::testing::test_inline,
        visit::{VisitMut, VisitMutWith},
    },
    plugin::{
        metadata::TransformPluginMetadataContextKind, plugin_transform,
        proxies::TransformPluginProgramMetadata,
    },
};
// use tracing::debug;

const IMPORT_NAME: &str = "eszett";
const SCOPE_NAME_NAME: &str = "scopeName";

pub struct TransformVisitor {
    filepath: String,
    sz_identifier: Option<Id>,
    scope_name_identifier: Option<Id>,
    scope_counter: usize,
    current_scope: Option<usize>,
}

impl TransformVisitor {
    pub fn new(filepath: impl Into<String>) -> Self {
        Self {
            filepath: filepath.into(),
            sz_identifier: None,
            scope_name_identifier: None,
            scope_counter: 0,
            current_scope: None,
        }
    }

    fn get_scope(&self) -> String {
        let re = Regex::new(r"[^a-zA-Z0-9_-]").unwrap();
        let prefix = re.replace_all(&self.filepath, "_");
        let current_scope = match self.current_scope {
            Some(current_scope) => current_scope,
            None => 0,
        };

        return format!("ß-{}-{}", prefix, current_scope);
    }

    fn visit_mut_children_providing_current_scope(&mut self, node: &mut dyn VisitMutWith<Self>) {
        let mut did_create_new_scope = false;
        if self.current_scope == None {
            self.scope_counter += 1;
            self.current_scope = Some(self.scope_counter);
            did_create_new_scope = true;
        }

        node.visit_mut_children_with(self);

        if did_create_new_scope {
            self.current_scope = None
        }
    }

    fn replace_sz_identifier_with_scope_name(&mut self, expression: &mut Expr) {
        let sz_identifier = match &self.sz_identifier {
            Some(sz_identifier) => sz_identifier,
            None => return,
        };
        let tagged_template = match expression.as_tagged_tpl() {
            Some(tagged_template) => tagged_template,
            None => return,
        };
        let tag = match tagged_template.tag.as_ident() {
            Some(tag) => tag,
            None => return,
        };

        if tag.to_id() != *sz_identifier {
            return;
        }

        let scope_name = self.get_scope() + " ";

        let template_literal = Expr::Tpl(*tagged_template.tpl.clone());
        let scope_string = Expr::Lit(scope_name.into());

        // replace node with new expression
        *expression = Expr::Bin(BinExpr {
            left: Box::new(scope_string),
            op: op!(bin, "+"),
            right: Box::new(template_literal),
            span: DUMMY_SP,
        });
    }

    fn replace_scope_name_identifier_with_scope_name(&mut self, expression: &mut Expr) {
        let scope_name_identifier = match &self.scope_name_identifier {
            Some(scope_name_identifier) => scope_name_identifier,
            None => return,
        };
        let identifier = match expression.as_ident() {
            Some(tagged_template) => tagged_template,
            None => return,
        };
        if identifier.to_id() != *scope_name_identifier {
            return;
        }

        let scope_name = self.get_scope();
        *expression = Expr::Lit(scope_name.into());
    }
}

impl Default for TransformVisitor {
    fn default() -> Self {
        Self {
            filepath: String::from("file.js"),
            sz_identifier: None,
            scope_name_identifier: None,
            scope_counter: 0,
            current_scope: None,
        }
    }
}

impl VisitMut for TransformVisitor {
    // go through import declarations to see what identifier is used
    // to import the eszett tag
    fn visit_mut_import_decl(&mut self, import_decl: &mut ImportDecl) {
        import_decl.visit_mut_children_with(self);

        if import_decl.src.value != IMPORT_NAME {
            return;
        }

        for specifier in &import_decl.specifiers {
            // store sz identifier
            if let ImportSpecifier::Default(default_import) = specifier {
                self.sz_identifier = Some(default_import.local.to_id())
            }

            // store scope name identifier
            if let ImportSpecifier::Named(named_import) = specifier {
                let import_identifier = &named_import.local;
                let export_name: &Atom = match &named_import.imported {
                    // e.g. `import { "scopeName" as prefix } from "eszett"``
                    Some(ModuleExportName::Str(imported)) => &imported.value,
                    // e.g. `import { scopeName as prefix } from "eszett"`
                    // then we find the export name in here
                    Some(ModuleExportName::Ident(imported)) => &imported.sym,
                    // otherwise: `import { scopeName } from "eszett"`
                    // we can just use the local symbol
                    None => &import_identifier.sym,
                };

                if export_name == SCOPE_NAME_NAME {
                    self.scope_name_identifier = Some(import_identifier.to_id());
                }
            }
        }

        // convert import to an invalid value
        import_decl.take();
    }

    // remove the eszett import declaration
    // since it is not a real js function
    fn visit_mut_module_items(&mut self, stmts: &mut Vec<ModuleItem>) {
        stmts.visit_mut_children_with(self);

        // remove invalid imports
        stmts.retain(|module_item| match module_item {
            ModuleItem::ModuleDecl(ModuleDecl::Import(import)) => !import.src.is_empty(),
            _ => true,
        });
    }

    fn visit_mut_fn_decl(&mut self, declaration: &mut FnDecl) {
        self.visit_mut_children_providing_current_scope(declaration);
    }
    fn visit_mut_arrow_expr(&mut self, arrow_expr: &mut ArrowExpr) {
        self.visit_mut_children_providing_current_scope(arrow_expr)
    }

    // replace all uses of sz indentifier as a template tag
    // with a unique scope string prefixing the template literal
    fn visit_mut_expr(&mut self, expression: &mut Expr) {
        expression.visit_mut_children_with(self);
        self.replace_sz_identifier_with_scope_name(expression);
        self.replace_scope_name_identifier_with_scope_name(expression);
    }
}

#[plugin_transform]
pub fn process_transform(mut program: Program, data: TransformPluginProgramMetadata) -> Program {
    let filepath = match data.get_context(&TransformPluginMetadataContextKind::Filename) {
        Some(s) => s,
        None => String::from(""),
    };

    program.visit_mut_with(&mut TransformVisitor::new(filepath));

    return program;
}

#[cfg(test)]
use swc_core::ecma::visit::as_folder;
test_inline!(
    Default::default(),
    |_| as_folder(TransformVisitor::default()),
    should_remove_sz_import_when_using_default_import,
    r#"import sz from 'eszett'"#,
    r#""#
);

#[cfg(test)]
test_inline!(
    Default::default(),
    |_| as_folder(TransformVisitor::default()),
    should_remove_sz_import_when_using_named_import,
    r#"import { scopeName } from 'eszett'"#,
    r#""#
);

#[cfg(test)]
test_inline!(
    Default::default(),
    |_| as_folder(TransformVisitor::default()),
    should_remove_sz_import_when_using_both_import,
    r#"import sz, { scopeName as foo } from 'eszett'"#,
    r#""#
);

#[cfg(test)]
test_inline!(
    Default::default(),
    |_| as_folder(TransformVisitor::default()),
    should_keep_other_imports,
    r#"import sz from 'some_import'"#,
    r#"import sz from 'some_import'"#
);

#[cfg(test)]
test_inline!(
    Default::default(),
    |_| as_folder(TransformVisitor::default()),
    should_replace_tagged_template_literals,
    r#"
        import sz from 'eszett'
        const hui = sz`my-class`
    "#,
    r#"
        const hui = "ß-file_js-0 " + `my-class`
    "#
);

#[cfg(test)]
test_inline!(
    Default::default(),
    |_| as_folder(TransformVisitor::default()),
    should_work_with_empty_template_literal,
    r#"
        import sz from 'eszett'
        const hui = sz``
    "#,
    r#"
        const hui = "ß-file_js-0 " + ``
    "#
);

#[cfg(test)]
test_inline!(
    Default::default(),
    |_| as_folder(TransformVisitor::default()),
    should_leave_non_sz_template_literals_alone,
    r#"
        import sz from 'eszett'
        const hui = css`my-class`
    "#,
    r#"
        const hui = css`my-class`
    "#
);

#[cfg(test)]
test_inline!(
    Default::default(),
    |_| as_folder(TransformVisitor::default()),
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

#[cfg(test)]
test_inline!(
    Default::default(),
    |_| as_folder(TransformVisitor::default()),
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

#[cfg(test)]
test_inline!(
    Default::default(),
    |_| as_folder(TransformVisitor::default()),
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

#[cfg(test)]
test_inline!(
    Default::default(),
    |_| as_folder(TransformVisitor::default()),
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

#[cfg(test)]
test_inline!(
    Default::default(),
    |_| as_folder(TransformVisitor::default()),
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

#[cfg(test)]
test_inline!(
    Default::default(),
    |_| as_folder(TransformVisitor::default()),
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

#[cfg(test)]
test_inline!(
    Default::default(),
    |_| as_folder(TransformVisitor::default()),
    should_replace_scope_name_variable_with_current_scope,
    r#"
        import { scopeName } from 'eszett'
        const scope = scopeName
    "#,
    r#"
        const scope = "ß-file_js-0"
    "#
);

#[cfg(test)]
test_inline!(
    Default::default(),
    |_| as_folder(TransformVisitor::default()),
    should_replace_scope_name_variable_when_renamed_in_import,
    r#"
        import { scopeName as sc } from 'eszett'
        const scope = sc
    "#,
    r#"
        const scope = "ß-file_js-0"
    "#
);

#[cfg(test)]
test_inline!(
    Default::default(),
    |_| as_folder(TransformVisitor::default()),
    should_not_replace_other_variables_with_the_scope_name_name,
    r#"
        import { scopeName as sc } from 'eszett'
        const scope = scopeName
    "#,
    r#"
        const scope = scopeName
    "#
);

#[cfg(test)]
test_inline!(
    Default::default(),
    |_| as_folder(TransformVisitor::default()),
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
