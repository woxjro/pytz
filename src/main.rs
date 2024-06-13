use pytz::{
    mlir::{Type, Value},
    python::AnnotationToken,
};
use rustpython_parser::{ast, Parse};

fn main() {
    let python_source = include_str!("../examples/python/michelson.py");
    let ast = ast::Suite::parse(python_source, "<embedded>").unwrap();

    for stmt in ast {
        match stmt {
            ast::Stmt::FunctionDef(stmt_function_def) => {
                if stmt_function_def.name.contains("smart_contract") {
                    stmt_function_def.body.iter().for_each(|stmt| match stmt {
                        // Annotated Assignment
                        ast::Stmt::AnnAssign(stmt_ann_assign) => {
                            // annotation should be Final[]
                            let annotation = *stmt_ann_assign.annotation.to_owned();
                            if let ast::Expr::Subscript(expr_subscript) = annotation.to_owned() {
                                let value = *expr_subscript.value.to_owned();
                                if let ast::Expr::Name(expr_name) = value.to_owned() {
                                    let id: String = expr_name.id.into();
                                    if id == AnnotationToken::Final.to_string() {
                                        let ty = get_mlir_type_from_annotation(
                                            *expr_subscript.slice.to_owned(),
                                        );
                                        let target = *stmt_ann_assign.target.to_owned();
                                        if let ast::Expr::Name(expr_name) = target {
                                            let id: String = expr_name.id.into();
                                            let value = Value { id, ty };
                                            dbg!(&value);
                                            dbg!(&stmt_ann_assign);
                                        }
                                    } else {
                                        panic!("Annotation should be Final[]");
                                    }
                                }
                            }

                            if let Some(call) = stmt_ann_assign.value.as_ref() {
                                if let ast::Expr::Call(expr_call) = *call.to_owned() {
                                    let func = *expr_call.func.to_owned();
                                    if let ast::Expr::Name(expr_name) = func {
                                        let id: String = expr_name.id.into();
                                        if id == "make_list" {}
                                    }
                                }
                            }
                        }
                        ast::Stmt::Return(..) => {}
                        _ => {}
                    });
                }
            }
            _ => {}
        }
    }
}

fn get_mlir_type_from_annotation(annotation: ast::Expr) -> Type {
    match annotation {
        ast::Expr::Name(expr_name) => {
            let id: String = expr_name.id.into();
            if id == AnnotationToken::Mutez.to_string() {
                Type::Mutez
            } else if id == AnnotationToken::Operation.to_string() {
                Type::Operation
            } else {
                todo!("{id} is not supported")
            }
        }
        ast::Expr::Subscript(expr_subscript) => {
            let value = *expr_subscript.value.to_owned();
            if let ast::Expr::Name(expr_name) = value.to_owned() {
                let id: String = expr_name.id.into();
                if id == AnnotationToken::List.to_string() {
                    Type::List {
                        elem: Box::new(get_mlir_type_from_annotation(
                            *expr_subscript.slice.to_owned(),
                        )),
                    }
                } else {
                    todo!();
                }
            } else {
                panic!();
            }
        }
        _ => panic!("Annotation should be Final[]"),
    }
}
