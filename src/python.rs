#[derive(Debug, Clone, PartialEq)]
pub enum AnnotationToken {
    Final,
    Mutez,
    Unit,
    Address,
    Contract,
    Operation,
    Optional,
    List,
    Pair,
}

impl std::fmt::Display for AnnotationToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Final => write!(f, "Final"),
            Self::Mutez => write!(f, "Mutez"),
            Self::Unit => write!(f, "Unit"),
            Self::Address => write!(f, "Address"),
            Self::Contract => write!(f, "Contract"),
            Self::Operation => write!(f, "Operation"),
            Self::Optional => write!(f, "Optional"),
            Self::List => write!(f, "List"),
            Self::Pair => write!(f, "Pair"),
        }
    }
}
