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

        if let Some(t) = n.as_tagged_tpl() {
            let tpl = Expr::Tpl(*t.tpl.clone());
            let sz = Expr::Ident(sz_identifier.clone());
            let expr = Expr::Bin(BinExpr {
                left: Box::new(sz),
                op: op!(bin, "+"),
                right: Box::new(tpl),
                span: t.span,
            });
            *n = expr;
        }
    }
}

#[cfg(test)]
use swc_core::ecma::visit::as_folder;
test_inline!(
    Default::default(),
    |_| as_folder(TransformVisitor::default()),
    boo,
    r#"
        import sz from 'errnesto/eszett'
        const hui = sz`my-class`
    "#,
    r#"
        const hui = sz + `my-class`
    "#
);
