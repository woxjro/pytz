from dataclasses import dataclass
from typing import Final, List, TypeVar, Generic, Optional

Address = str
Bytes = str
Bool = bool
Int = int
Key = str
Mutez = int
Signature = str
String = str


class Operation:
    pass


class Unit:
    pass


T = TypeVar('T')
U = TypeVar('U')


@dataclass
class Pair(Generic[T, U]):
    first: T
    second: U


@dataclass
class Contract(Generic[T]):
    pass


def make_pair(first: T, second: U) -> Pair[T, U]:
    return Pair(first, second)


def get_fst(pair: Pair[T, U]) -> T:
    return pair.first


def get_snd(pair: Pair[T, U]) -> U:
    return pair.second


def get_amount() -> Mutez:
    DUMMY_AMOUNT: Final[Mutez] = 5000
    return DUMMY_AMOUNT


def make_list() -> List:
    return []


def append(operations: List[Operation], operation: Operation) -> List[Operation]:
    operations.append(operation)
    return operations


def get_source() -> Address:
    DUMMY_SOURCE: Final[Address] = "tz1KqTpEZ7Yob7QbPE4Hy4Wo8fHG8LhKxZSx"
    return DUMMY_SOURCE


def get_contract(address: Address) -> Optional[Contract[T]]:
    return Contract()


def assrt(condition: Bool):
    if not condition:
        raise Exception("Assertion failed")


def check_signature(key: Key, signature: Signature, bytes: Bytes) -> Bool:
    return True


def assert_some(value: Optional[T]) -> T:
    if value is None:
        raise Exception("Expected Some but got None")
    return value


def transfer_tokens(param: T, amount: Mutez, contract: Contract[T]) -> Operation:
    return Operation()


def pack(str: String) -> Bytes:
    return str

# translate the following typescript code to python
# function smartContract(
#   storage: Pair<signature, String>,
#   param: key
# ): Pair<Operation[], Pair<signature, String>> {
#   const signature: signature = getFst(storage);
#   const str: String = getSnd(storage);
#   const byt: bytes = pack(str);
#   const result: bool = checkSignature(param, signature, byt);
#   assert(result);
#   const nil: Operation[] = makeList();
#
#   const p: Pair<Operation[], Pair<signature, String>> = makePair(nil, storage);
#   return p;
# }


def smart_contract(storage: Pair[Signature, String], param: Key) -> Pair[List[Operation], Pair[Signature, String]]:
    signature: Final[Signature] = get_fst(storage)
    str: Final[String] = get_snd(storage)
    byt: Final[Bytes] = pack(str)
    result: Final[Bool] = check_signature(param, signature, byt)
    assrt(result)
    nil: Final[List[Operation]] = make_list()

    p: Final[Pair[List[Operation], Pair[Signature, String]]
             ] = make_pair(nil, storage)
    return p
