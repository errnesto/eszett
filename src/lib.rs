use swc_core::{
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
    sz_identifier: Option<Ident>,
}

impl VisitMut for TransformVisitor {
    fn visit_mut_import_decl(&mut self, n: &mut ImportDecl) {
        n.visit_mut_children_with(self);

        if n.src.value == IMPORT_NAME {
            // store identifier
            for specifier in &n.specifiers {
                if let ImportSpecifier::Default(s) = specifier {
                    self.sz_identifier = Some(s.local.clone())
                }
            }
            // convert import to an invalid value
            n.take();
        }
    }
    fn visit_mut_module_items(&mut self, stmts: &mut Vec<ModuleItem>) {
        stmts.visit_mut_children_with(self);

        // remove invalid imports
        stmts.retain(|s| {
            if let ModuleItem::ModuleDecl(ModuleDecl::Import(x)) = s {
                return !x.src.is_empty();
            }
            return true;
        });
    }
    fn visit_mut_expr(&mut self, n: &mut Expr) {
        n.visit_mut_children_with(self);

        let sz_identifier;
        match &self.sz_identifier {
            Some(sz) => sz_identifier = sz,
            _ => return,
        }
        let tagged_template;
        match n.as_tagged_tpl() {
            Some(t) => tagged_template = t,
            _ => return,
        }

        let tag;
        match tagged_template.tag.as_ident() {
            Some(t) => tag = t,
            _ => return
        }

        if tag.sym != sz_identifier.sym {
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
    should_leave_non_sz_template_literals_alone,
    r#"
        import sz from 'errnesto/eszett'
        const hui = css`my-class`
    "#,
    r#"
        const hui = css`my-class`
    "#
);
