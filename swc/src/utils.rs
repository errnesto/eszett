use swc_core::{
    common::DUMMY_SP,
    ecma::ast::{BinExpr, BinaryOp, Expr, Ident},
};

pub fn string_literal_expr(s: &str) -> Expr {
    s.replace("\\`", "`").into()
}

pub fn ident(s: &str) -> Ident {
    Ident {
        sym: s.into(),
        span: DUMMY_SP,
        optional: false,
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
