use std::mem::take;

use swc_core::{
    atoms::Atom,
    common::{collections::AHashSet, util::take::Take, DUMMY_SP},
    ecma::{
        ast::*,
        visit::{VisitMut, VisitMutWith},
    },
};

use crate::utils::{add, and, ident, is_capitalized, not_eq, or, string_literal_expr};

const IMPORT_NAME: &str = "eszett";
const TEMPLATE_LITERAL_TAG_NAME: &str = "sz";

pub struct Eszett {
    filepath_hash: String,
    template_tag_identifier: Option<Id>,
    scope_name_identifier: Option<Id>,
    scope_counter: usize,
    current_scope: Option<usize>,
    nearest_bindings: AHashSet<Id>,
}

impl Eszett {
    pub fn new(filepath: impl Into<String>) -> Self {
        Self {
            filepath_hash: filepath.into(),
            template_tag_identifier: None,
            scope_name_identifier: None,
            scope_counter: 0,
            current_scope: None,
            nearest_bindings: Default::default(),
        }
    }

    fn get_scope_name(&self) -> String {
        let current_scope = match self.current_scope {
            Some(current_scope) => current_scope,
            None => 0,
        };

        return format!("sz-{}-{}", self.filepath_hash, current_scope);
    }

    fn visit_mut_children_providing_current_scope(&mut self, node: &mut dyn VisitMutWith<Self>) {
        let mut did_create_new_scope = false;
        let surrounding_bindings = take(&mut self.nearest_bindings);
        // self.nearest_bindings.extend(collect_decls(&declaration));

        if self.current_scope == None {
            self.scope_counter += 1;
            self.current_scope = Some(self.scope_counter);
            did_create_new_scope = true;
        }

        node.visit_mut_children_with(self);

        if did_create_new_scope {
            self.current_scope = None
        }
        self.nearest_bindings = surrounding_bindings;
    }

    fn replace_sz_identifier_with_scope_name(&mut self, expression: &mut Expr) {
        let template_tag_identifier = match &self.template_tag_identifier {
            Some(template_tag_identifier) => template_tag_identifier,
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

        if tag.to_id() != *template_tag_identifier {
            return;
        }

        let scope_name = self.get_scope_name() + " ";

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

        let scope_name = self.get_scope_name();
        *expression = Expr::Lit(scope_name.into());
    }

    fn get_existing_class_name(
        el: &JSXOpeningElement,
    ) -> (Option<Expr>, Option<usize>, Option<usize>) {
        let mut spreads = vec![];
        let mut class_name_expr = None;
        let mut existing_index = None;
        let mut existing_spread_index = None;
        for i in (0..el.attrs.len()).rev() {
            match &el.attrs[i] {
                JSXAttrOrSpread::JSXAttr(JSXAttr {
                    name: JSXAttrName::Ident(Ident { sym, .. }),
                    value,
                    ..
                }) => {
                    if sym == "className" {
                        existing_index = Some(i);
                        class_name_expr = match value {
                            Some(JSXAttrValue::Lit(str_lit)) => Some(Expr::Lit(str_lit.clone())),
                            Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
                                expr: JSXExpr::Expr(expr),
                                ..
                            })) => Some(*expr.clone()),
                            None => None,
                            _ => None,
                        };
                        break;
                    }
                }
                JSXAttrOrSpread::SpreadElement(SpreadElement { expr, .. }) => {
                    if let Expr::Object(ObjectLit { props, .. }) = &**expr {
                        let mut has_spread = false;
                        let mut has_class_name = false;
                        for j in 0..props.len() {
                            if let PropOrSpread::Prop(prop) = &props[j] {
                                if let Prop::KeyValue(KeyValueProp {
                                    key: PropName::Ident(Ident { sym, .. }),
                                    value,
                                }) = &**prop
                                {
                                    if sym == "className" {
                                        has_class_name = true;
                                        class_name_expr = Some(*value.clone());
                                        if props.len() == 1 {
                                            existing_spread_index = Some(i);
                                        }
                                    }
                                }
                            } else {
                                has_spread = true;
                            }
                        }
                        if has_class_name {
                            break;
                        }
                        if !has_spread {
                            continue;
                        }
                    }

                    let valid_spread = matches!(&**expr, Expr::Member(_) | Expr::Ident(_));

                    if valid_spread {
                        let member_dot_name = Expr::Member(MemberExpr {
                            obj: Box::new(*expr.clone()),
                            prop: MemberProp::Ident(ident("className")),
                            span: DUMMY_SP,
                        });
                        // `${name} && ${name}.className != null && ${name}.className`
                        spreads.push(and(
                            and(
                                *expr.clone(),
                                not_eq(
                                    member_dot_name.clone(),
                                    Expr::Lit(Lit::Null(Null { span: DUMMY_SP })),
                                ),
                            ),
                            member_dot_name.clone(),
                        ));
                    }
                }
                _ => {}
            };
        }

        let spread_expr = match spreads.len() {
            0 => None,
            _ => Some(join_spreads(spreads)),
        };

        let class_name_expr = match class_name_expr {
            Some(e @ Expr::Tpl(_) | e @ Expr::Lit(Lit::Str(_))) => Some(e),
            None => None,
            _ => Some(or(class_name_expr.unwrap(), string_literal_expr(""))),
        };

        let existing_class_name_expr = match (spread_expr, class_name_expr) {
            (Some(spread_expr), Some(class_name_expr)) => Some(or(spread_expr, class_name_expr)),
            (Some(spread_expr), None) => Some(or(spread_expr, string_literal_expr(""))),
            (None, Some(class_name_expr)) => Some(class_name_expr),
            _ => None,
        };

        (
            existing_class_name_expr,
            existing_index,
            existing_spread_index,
        )
    }
}

fn join_spreads(spreads: Vec<Expr>) -> Expr {
    let mut new_expr = spreads[0].clone();
    for i in spreads.iter().skip(1) {
        new_expr = Expr::Bin(BinExpr {
            op: op!("||"),
            left: Box::new(new_expr.clone()),
            right: Box::new(i.clone()),
            span: DUMMY_SP,
        })
    }
    new_expr
}

impl VisitMut for Eszett {
    // go through import declarations to see what identifier is used
    // to import the eszett tag
    fn visit_mut_import_decl(&mut self, import_decl: &mut ImportDecl) {
        import_decl.visit_mut_children_with(self);

        if import_decl.src.value != IMPORT_NAME {
            return;
        }

        for specifier in &import_decl.specifiers {
            // store scope name identifier from default import expression
            // e.g. `import eszett from "eszett"`
            if let ImportSpecifier::Default(default_import) = specifier {
                self.scope_name_identifier = Some(default_import.local.to_id())
            }

            // store template tag identifier
            if let ImportSpecifier::Named(named_import) = specifier {
                let import_identifier = &named_import.local;
                let export_name: &Atom = match &named_import.imported {
                    // e.g. `import { "sz" as prefix } from "eszett"``
                    Some(ModuleExportName::Str(imported)) => &imported.value,
                    // e.g. `import { sz as prefix } from "eszett"`
                    // then we find the export name in here
                    Some(ModuleExportName::Ident(imported)) => &imported.sym,
                    // otherwise: `import { sz } from "eszett"`
                    // we can just use the local symbol
                    None => &import_identifier.sym,
                };

                if export_name == TEMPLATE_LITERAL_TAG_NAME {
                    self.template_tag_identifier = Some(import_identifier.to_id());
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

    fn visit_mut_binding_ident(&mut self, ident: &mut BindingIdent) {
        if self.current_scope.is_some() {
            self.nearest_bindings.insert(ident.id.to_id());
        }
    }

    fn visit_mut_assign_pat_prop(&mut self, asign_pat_prop: &mut AssignPatProp) {
        if self.current_scope.is_some() {
            self.nearest_bindings.insert(asign_pat_prop.key.to_id());
        }
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

    fn visit_mut_jsx_opening_element(&mut self, jsx_opening_element: &mut JSXOpeningElement) {
        jsx_opening_element.visit_mut_children_with(self);

        if let JSXElementName::Ident(identifier) = &jsx_opening_element.name {
            if identifier.sym != "style"
                && (!is_capitalized(&identifier.sym)
                    || self.nearest_bindings.contains(&identifier.to_id()))
            {
                let (existing_class_name, existing_index, existing_spread_index) =
                    Eszett::get_existing_class_name(&jsx_opening_element);

                let scope_name_expression = Expr::Lit(self.get_scope_name().into());
                let new_class_name = match existing_class_name {
                    Some(existing_class_name) => add(
                        add(scope_name_expression, string_literal_expr(" ")),
                        existing_class_name,
                    ),
                    None => scope_name_expression,
                };

                let class_name_attr = JSXAttrOrSpread::JSXAttr(JSXAttr {
                    span: DUMMY_SP,
                    name: JSXAttrName::Ident(ident("className")),
                    value: Some(JSXAttrValue::JSXExprContainer(JSXExprContainer {
                        expr: JSXExpr::Expr(Box::new(new_class_name)),
                        span: DUMMY_SP,
                    })),
                });
                jsx_opening_element.attrs.push(class_name_attr);

                if let Some(existing_spread_index) = existing_spread_index {
                    jsx_opening_element.attrs.remove(existing_spread_index);
                }
                if let Some(existing_index) = existing_index {
                    jsx_opening_element.attrs.remove(existing_index);
                }
            }
        }
    }
}
