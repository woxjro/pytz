#[derive(Debug, Clone, PartialEq)]
pub enum AnnotationToken {
    Address,
    Bytes,
    Contract,
    Final,
    Int,
    List,
    Mutez,
    Nat,
    Operation,
    Optional,
    Pair,
    Unit,
}

impl std::fmt::Display for AnnotationToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Address => write!(f, "Address"),
            Self::Bytes => write!(f, "Bytes"),
            Self::Contract => write!(f, "Contract"),
            Self::Final => write!(f, "Final"),
            Self::Int => write!(f, "Int"),
            Self::List => write!(f, "List"),
            Self::Mutez => write!(f, "Mutez"),
            Self::Nat => write!(f, "Nat"),
            Self::Operation => write!(f, "Operation"),
            Self::Optional => write!(f, "Optional"),
            Self::Pair => write!(f, "Pair"),
            Self::Unit => write!(f, "Unit"),
        }
    }
}
