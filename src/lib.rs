use swc_core::{
    common::Spanned,
    ecma::{
        ast::*,
        transforms::testing::test_inline,
        visit::{VisitMut, VisitMutWith},
    },
};

pub struct TransformVisitor;

impl VisitMut for TransformVisitor {
    fn visit_mut_bin_expr(&mut self, e: &mut BinExpr) {
        e.visit_mut_children_with(self);

        if e.op == op!("===") {
            e.left = Ident::new("kdy1".into(), e.left.span()).into();
        }
    }
}

#[cfg(test)]
use swc_core::ecma::visit::as_folder;
test_inline!(
    Default::default(),
    |_| as_folder(TransformVisitor),
    boo,
    r#"foo === bar;"#,
    r#"kdy1 === bar;"#
);
