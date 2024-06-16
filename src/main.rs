use pytz::{
    mlir::{Operation, OperationKind, Type, Value},
    python::AnnotationToken,
};
use rustpython_parser::{ast, Parse};

fn main() {
    let python_source = include_str!("../examples/python/michelson.py");
    let ast = ast::Suite::parse(python_source, "<embedded>").unwrap();

    let mut operations = vec![];
    let mut type_env = vec![];
    for stmt in ast {
        if let ast::Stmt::FunctionDef(stmt_function_def) = stmt {
            if stmt_function_def.name.contains("smart_contract") {
                for arg in stmt_function_def.args.args {
                    let value = get_mlir_value_from_function_arg(arg.to_owned());
                    type_env.push(value.to_owned());
                }

                stmt_function_def.body.iter().for_each(|stmt| match stmt {
                    // Annotated Assignment
                    ast::Stmt::AnnAssign(stmt_ann_assign) => {
                        // annotation should be Final[]
                        let value = get_value_from_annassign(stmt.to_owned());

                        if let Some(value) = value {
                            type_env.push(value.to_owned());
                            if let Some(call) = stmt_ann_assign.value.as_ref() {
                                if let ast::Expr::Call(expr_call) = *call.to_owned() {
                                    let func = *expr_call.func.to_owned();
                                    if let ast::Expr::Name(expr_name) = func {
                                        let id: String = expr_name.id.into();
                                        if id == "make_list" {
                                            let op = Operation {
                                                kind: OperationKind::MakeList,
                                                args: vec![],
                                                results: vec![Value {
                                                    id: value.id,
                                                    ty: value.ty,
                                                }],
                                            };
                                            operations.push(op);
                                        } else if id == "get_amount" {
                                            let op = Operation {
                                                kind: OperationKind::GetAmount,
                                                args: vec![],
                                                results: vec![Value {
                                                    id: value.id,
                                                    ty: value.ty,
                                                }],
                                            };
                                            operations.push(op);
                                        } else if id == "make_pair" {
                                            let args = expr_call
                                                .args
                                                .to_owned()
                                                .iter()
                                                .map(|arg| {
                                                    if let ast::Expr::Name(expr_name) = arg {
                                                        let id: String =
                                                            expr_name.id.to_owned().into();
                                                        let value = type_env
                                                            .iter()
                                                            .find(|value| {
                                                                value.id == format!("%{}", id)
                                                            })
                                                            .unwrap();
                                                        value.to_owned()
                                                    } else {
                                                        panic!();
                                                    }
                                                })
                                                .collect::<Vec<Value>>();
                                            let op = Operation {
                                                kind: OperationKind::MakePair,
                                                args,
                                                results: vec![Value {
                                                    id: value.id,
                                                    ty: value.ty,
                                                }],
                                            };
                                            operations.push(op);
                                        } else {
                                            todo!("{id} is not supported");
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ast::Stmt::Return(stmt_return) => {
                        let value = *stmt_return.value.to_owned().unwrap();
                        if let ast::Expr::Name(expr_name) = value {
                            let id: String = expr_name.id.into();
                            let value = type_env
                                .iter()
                                .find(|value| value.id == format!("%{}", id))
                                .unwrap();
                            let op = Operation {
                                kind: OperationKind::Return,
                                args: vec![value.to_owned()],
                                results: vec![],
                            };
                            operations.push(op);
                        }
                    }
                    _ => {}
                });
            }
        }
    }

    let storage = type_env
        .iter()
        .find(|value| value.id == "%storage")
        .unwrap();
    let param = type_env.iter().find(|value| value.id == "%param").unwrap();
    println!("module {{");
    println!(
        "  func.func @smart_contract({}: {}, {}: {}) -> {} {{",
        param.id,
        param.ty,
        storage.id,
        storage.ty,
        Type::Pair {
            fst: Box::new(Type::List {
                elem: Box::new(Type::Operation)
            }),
            snd: Box::new(storage.ty.to_owned())
        }
    );
    for op in operations {
        println!("    {}", op.to_string());
    }
    println!("  }}");
    println!("}}");
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
                } else if id == AnnotationToken::Pair.to_string() {
                    if let ast::Expr::Tuple(expr_tuple) = *expr_subscript.slice.to_owned() {
                        let mut types = vec![];
                        for expr in expr_tuple.elts {
                            types.push(get_mlir_type_from_annotation(expr));
                        }
                        Type::Pair {
                            fst: Box::new(types[0].to_owned()),
                            snd: Box::new(types[1].to_owned()),
                        }
                    } else {
                        panic!();
                    }
                } else {
                    todo!("{id} is not supported");
                }
            } else {
                panic!();
            }
        }
        _ => panic!("Annotation should be Final[]"),
    }
}

fn get_mlir_value_from_function_arg(arg: ast::ArgWithDefault) -> Value {
    let arg = arg.def;
    let id = format!("%{}", arg.arg);
    let ty = get_mlir_type_from_annotation(*arg.annotation.unwrap());
    Value { id, ty }
}

// expected input: v: Final[mutez] = get_amount()
// expected output: Some(Value { id: "v", ty: Type::Mutez })
fn get_value_from_annassign(stmt: ast::Stmt) -> Option<Value> {
    // Annotated Assignment
    if let ast::Stmt::AnnAssign(stmt_ann_assign) = stmt {
        // annotation should be Final[]
        let annotation = *stmt_ann_assign.annotation.to_owned();
        if let ast::Expr::Subscript(expr_subscript) = annotation.to_owned() {
            let value = *expr_subscript.value.to_owned();
            if let ast::Expr::Name(expr_name) = value.to_owned() {
                let id: String = expr_name.id.into();
                if id == AnnotationToken::Final.to_string() {
                    let ty = get_mlir_type_from_annotation(*expr_subscript.slice.to_owned());
                    let target = *stmt_ann_assign.target.to_owned();
                    if let ast::Expr::Name(expr_name) = target {
                        let id: String = expr_name.id.into();
                        Some(Value {
                            id: format!("%{id}"),
                            ty,
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}
