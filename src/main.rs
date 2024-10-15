use clap::Parser as CParser;
use pytz::{
    mlir::{Operation, OperationKind, Type, Value},
    python::{AnnotationToken, MichelsonFunction},
};
use rustpython_parser::{ast, Parse};
use std::path::PathBuf;

#[derive(CParser, Debug)]
#[command(name = "FilePath")]
struct Args {
    #[arg(short, long, value_name = "INPUT", required = true)]
    input: PathBuf,
    #[arg(short, long, value_name = "OUTPUT", required = false)]
    output: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    // load python source code using args.input
    let python_source = std::fs::read_to_string(args.input).unwrap();
    let ast = ast::Suite::parse(&python_source, "<embedded>").unwrap();

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
                        dbg!(&value);
                        if let Some(value) = value {
                            type_env.push(value.to_owned());
                            let results = vec![Value {
                                id: value.id,
                                ty: value.ty,
                            }];
                            if let Some(call) = stmt_ann_assign.value.as_ref() {
                                if let ast::Expr::Call(expr_call) = *call.to_owned() {
                                    let func = *expr_call.func.to_owned();
                                    if let ast::Expr::Name(expr_name) = func {
                                        let id: String = expr_name.id.into();
                                        let args = expr_call
                                            .args
                                            .to_owned()
                                            .iter()
                                            .map(|arg| {
                                                if let ast::Expr::Name(expr_name) = arg {
                                                    let id: String = expr_name.id.to_owned().into();
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

                                        let michelson_function: MichelsonFunction =
                                            id.as_str().into();
                                        let kind = match michelson_function {
                                            MichelsonFunction::Append => OperationKind::Append,
                                            MichelsonFunction::Assert => OperationKind::Assert,
                                            MichelsonFunction::AssertSome => {
                                                OperationKind::AssertSome
                                            }
                                            MichelsonFunction::CheckSignature => {
                                                OperationKind::CheckSignature
                                            }
                                            MichelsonFunction::GetAmount => {
                                                OperationKind::GetAmount
                                            }
                                            MichelsonFunction::GetBytes => OperationKind::GetBytes,
                                            MichelsonFunction::GetContract => {
                                                OperationKind::GetContract
                                            }
                                            MichelsonFunction::GetFst => OperationKind::GetFst,
                                            MichelsonFunction::GetSnd => OperationKind::GetSnd,
                                            MichelsonFunction::GetSource => {
                                                OperationKind::GetSource
                                            }
                                            MichelsonFunction::MakeList => OperationKind::MakeList,
                                            MichelsonFunction::MakePair => OperationKind::MakePair,
                                            MichelsonFunction::Pack => OperationKind::Pack,
                                            MichelsonFunction::Sha256 => OperationKind::Sha256,
                                            MichelsonFunction::TransferTokens => {
                                                OperationKind::TransferTokens
                                            }
                                        };
                                        operations.push(Operation {
                                            kind,
                                            args,
                                            results,
                                        });
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
                    stmt => {
                        panic!("{:?} is not supported yet", stmt);
                    }
                });
            }
        }
    }

    let storage = type_env
        .iter()
        .find(|value| value.id == "%storage")
        .unwrap();
    let param = type_env.iter().find(|value| value.id == "%param").unwrap();

    let mut mlir_code = String::new();

    mlir_code.push_str("module {\n");
    mlir_code.push_str(&format!(
        "  func.func @smart_contract({}: {}, {}: {}) -> {} {{\n",
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
    ));
    for op in operations {
        mlir_code.push_str(&format!("    {}\n", op));
    }
    mlir_code.push_str("  }\n");
    mlir_code.push('}');

    if let Some(output) = args.output {
        std::fs::write(output, mlir_code).unwrap();
    } else {
        println!("{}", mlir_code);
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
            } else if id == AnnotationToken::Key.to_string() {
                Type::Key
            } else if id == AnnotationToken::Bool.to_string() {
                Type::Bool
            } else if id == AnnotationToken::Signature.to_string() {
                Type::Signature
            } else if id == AnnotationToken::String.to_string() {
                Type::String
            } else if id == AnnotationToken::Unit.to_string() {
                Type::Unit
            } else if id == AnnotationToken::Address.to_string() {
                Type::Address
            } else if id == AnnotationToken::Nat.to_string() {
                Type::Nat
            } else if id == AnnotationToken::Int.to_string() {
                Type::Int
            } else if id == AnnotationToken::Bytes.to_string() {
                Type::Bytes
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
                } else if id == AnnotationToken::Optional.to_string() {
                    Type::Option {
                        elem: Box::new(get_mlir_type_from_annotation(
                            *expr_subscript.slice.to_owned(),
                        )),
                    }
                } else if id == AnnotationToken::Contract.to_string() {
                    Type::Contract {
                        param: Box::new(get_mlir_type_from_annotation(
                            *expr_subscript.slice.to_owned(),
                        )),
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
