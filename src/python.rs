#[derive(Debug, Clone, PartialEq)]
pub enum AnnotationToken {
    Address,
    Bool,
    Bytes,
    Contract,
    Final,
    Int,
    Key,
    List,
    Mutez,
    Nat,
    Operation,
    Optional,
    Pair,
    Signature,
    String,
    Unit,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MichelsonFunction {
    Append,
    AssertSome,
    GetAmount,
    GetBytes,
    GetContract,
    GetSource,
    MakeList,
    MakePair,
    Sha256,
    TransferTokens,
}

impl std::fmt::Display for AnnotationToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Address => write!(f, "Address"),
            Self::Bool => write!(f, "Bool"),
            Self::Bytes => write!(f, "Bytes"),
            Self::Contract => write!(f, "Contract"),
            Self::Final => write!(f, "Final"),
            Self::Int => write!(f, "Int"),
            Self::Key => write!(f, "Key"),
            Self::List => write!(f, "List"),
            Self::Mutez => write!(f, "Mutez"),
            Self::Nat => write!(f, "Nat"),
            Self::Operation => write!(f, "Operation"),
            Self::Optional => write!(f, "Optional"),
            Self::Pair => write!(f, "Pair"),
            Self::Signature => write!(f, "Signature"),
            Self::String => write!(f, "String"),
            Self::Unit => write!(f, "Unit"),
        }
    }
}

impl From<&str> for MichelsonFunction {
    fn from(s: &str) -> Self {
        match s {
            "append" => Self::Append,
            "assert_some" => Self::AssertSome,
            "get_amount" => Self::GetAmount,
            "get_bytes" => Self::GetBytes,
            "get_contract" => Self::GetContract,
            "get_source" => Self::GetSource,
            "make_list" => Self::MakeList,
            "make_pair" => Self::MakePair,
            "sha256" => Self::Sha256,
            "transfer_tokens" => Self::TransferTokens,
            _ => panic!("Unknown Michelson function: {}", s),
        }
    }
}
