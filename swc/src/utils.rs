use swc_core::{
    common::DUMMY_SP,
    ecma::ast::{BinExpr, BinaryOp, Expr, IdentName},
};

pub fn string_literal_expr(s: &str) -> Expr {
    s.replace("\\`", "`").into()
}

pub fn ident(s: &str) -> IdentName {
    IdentName {
        sym: s.into(),
        span: DUMMY_SP,
    }
}

pub fn is_capitalized(word: &str) -> bool {
    word.chars().next().unwrap().is_uppercase()
}

pub fn add(left: Expr, right: Expr) -> Expr {
    binary_expr(BinaryOp::Add, left, right)
}

pub fn and(left: Expr, right: Expr) -> Expr {
    binary_expr(BinaryOp::LogicalAnd, left, right)
}

pub fn or(left: Expr, right: Expr) -> Expr {
    binary_expr(BinaryOp::LogicalOr, left, right)
}

pub fn not_eq(left: Expr, right: Expr) -> Expr {
    binary_expr(BinaryOp::NotEq, left, right)
}

pub fn binary_expr(op: BinaryOp, left: Expr, right: Expr) -> Expr {
    Expr::Bin(BinExpr {
        op,
        left: Box::new(left),
        right: Box::new(right),
        span: DUMMY_SP,
    })
}
