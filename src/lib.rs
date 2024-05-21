use swc_core::{
    atoms::Atom,
    common::util::take::Take,
    ecma::{
        ast::*,
        transforms::testing::test_inline,
        visit::{VisitMut, VisitMutWith},
    },
};
// use tracing::debug;

const IMPORT_NAME: &str = "errnesto/eszett";

#[derive(Default)]
pub struct TransformVisitor {
    sz_identifier: Option<Atom>,
}

impl VisitMut for TransformVisitor {
    fn visit_mut_import_decl(&mut self, import_decl: &mut ImportDecl) {
        import_decl.visit_mut_children_with(self);

        if import_decl.src.value != IMPORT_NAME {
            return;
        }

        // store identifier
        for specifier in &import_decl.specifiers {
            if let ImportSpecifier::Default(default_import) = specifier {
                self.sz_identifier = Some(default_import.local.sym.clone())
            }
        }

        // convert import to an invalid value
        import_decl.take();
    }

    fn visit_mut_module_items(&mut self, stmts: &mut Vec<ModuleItem>) {
        stmts.visit_mut_children_with(self);

        // remove invalid imports
        stmts.retain(|module_item| match module_item {
            ModuleItem::ModuleDecl(ModuleDecl::Import(import)) => !import.src.is_empty(),
            _ => true,
        });
    }

    fn visit_mut_expr(&mut self, n: &mut Expr) {
        n.visit_mut_children_with(self);

        let sz_identifier = match &self.sz_identifier {
            Some(sz_identifier) => sz_identifier,
            None => return,
        };
        let tagged_template = match n.as_tagged_tpl() {
            Some(tagged_template) => tagged_template,
            None => return,
        };
        let tag = match tagged_template.tag.as_ident() {
            Some(tag) => tag,
            None => return,
        };

        if tag.sym != *sz_identifier {
            return;
        }

        let tpl = Expr::Tpl(*tagged_template.tpl.clone());
        let scope = Expr::Lit("scope".into());
        let expr = Expr::Bin(BinExpr {
            left: Box::new(scope),
            op: op!(bin, "+"),
            right: Box::new(tpl),
            span: tagged_template.span,
        });

        *n = expr;
    }
}

#[cfg(test)]
use swc_core::ecma::visit::as_folder;
test_inline!(
    Default::default(),
    |_| as_folder(TransformVisitor::default()),
    should_remove_sz_import,
    r#"import sz from 'errnesto/eszett'"#,
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
        import sz from 'errnesto/eszett'
        const hui = sz`my-class`
    "#,
    r#"
        const hui = "scope" + `my-class`
    "#
);

#[cfg(test)]
test_inline!(
    Default::default(),
    |_| as_folder(TransformVisitor::default()),
    should_work_with_empty_template_literal,
    r#"
        import sz from 'errnesto/eszett'
        const hui = sz``
    "#,
    r#"
        const hui = "scope" + ``
    "#
);

#[cfg(test)]
test_inline!(
    Default::default(),
    |_| as_folder(TransformVisitor::default()),
    should_leave_non_sz_template_literals_alone,
    r#"
        import sz from 'errnesto/eszett'
        const hui = css`my-class`
    "#,
    r#"
        const hui = css`my-class`
    "#
);
