pub mod mlir {

    #[derive(Debug, Clone, PartialEq)]
    pub struct Value {
        pub id: String, // "%0" "%a" "%b" etc
        pub ty: Type,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        Mutez,
        Operation,
        List { elem: Box<Type> },
        Pair { fst: Box<Type>, snd: Box<Type> },
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum OperationKind {
        MakePair,
        MakeList,
        GetAmount,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Operation {
        pub kind: OperationKind,
        pub args: Vec<Value>,
        pub results: Vec<Value>,
    }

    impl std::fmt::Display for Type {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Mutez => write!(f, "!michelson.mutez"),
                Self::Operation => write!(f, "!michelson.operation"),
                Self::List { elem } => write!(f, "!michelson.list<{}>", elem),
                Self::Pair { fst, snd } => write!(f, "!michelson.pair<{}, {}>", fst, snd),
            }
        }
    }

    impl ToString for Operation {
        fn to_string(&self) -> String {
            match self.kind {
                OperationKind::MakePair => {
                    let result_type = Type::Pair {
                        fst: Box::new(self.args[0].ty.clone()),
                        snd: Box::new(self.args[1].ty.clone()),
                    };
                    format!(
                        "{} = !michelson.make_pair({}, {}): {result_type}",
                        self.results[0].id, self.args[0].id, self.args[1].id
                    )
                }
                OperationKind::MakeList => {
                    /*
                    let mut res = format!("make_list");
                    for arg in &self.args {
                        res.push_str(&format!(" {}", arg.id));
                    }
                    res
                    */
                    todo!()
                }
                OperationKind::GetAmount => {
                    // format!("get_amount {}", self.args[0].id)
                    todo!()
                }
            }
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;
        #[test]
        fn test_list_type_to_string() {
            let t = Type::List {
                elem: Box::new(Type::Mutez),
            };
            assert_eq!(t.to_string(), "!michelson.list<!michelson.mutez>");
        }

        #[test]
        fn test_pair_type_to_string() {
            let t = Type::Pair {
                fst: Box::new(Type::Mutez),
                snd: Box::new(Type::Operation),
            };
            assert_eq!(
                t.to_string(),
                "!michelson.pair<!michelson.mutez, !michelson.operation>"
            );
        }

        #[test]
        fn test_nested_type_to_string() {
            let t = Type::List {
                elem: Box::new(Type::Pair {
                    fst: Box::new(Type::Mutez),
                    snd: Box::new(Type::Operation),
                }),
            };
            assert_eq!(
                t.to_string(),
                "!michelson.list<!michelson.pair<!michelson.mutez, !michelson.operation>>"
            );
        }

        #[test]
        fn test_make_pair_to_string() {
            let op = Operation {
                kind: OperationKind::MakePair,
                args: vec![
                    Value {
                        id: "%a".to_string(),
                        ty: Type::Mutez,
                    },
                    Value {
                        id: "%b".to_string(),
                        ty: Type::Operation,
                    },
                ],
                results: vec![Value {
                    id: "%0".to_string(),
                    ty: Type::Pair {
                        fst: Box::new(Type::Mutez),
                        snd: Box::new(Type::Operation),
                    },
                }],
            };

            assert_eq!(
                op.to_string(),
                "%0 = !michelson.make_pair(%a, %b): !michelson.pair<!michelson.mutez, !michelson.operation>"
            );
        }
    }
}
