from dataclasses import dataclass
from typing import Final, List, TypeVar, Generic, Optional

Mutez = int
Address = str
Int = int
Nat = int
Bytes = bytes


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


def get_bytes(number: Int) -> Bytes:
    return number.to_bytes(32, 'big')


def sha256(x: Bytes) -> Bytes:
    return x


def make_pair(first: T, second: U) -> Pair[T, U]:
    return Pair(first, second)


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


def assert_some(value: Optional[T]) -> T:
    if value is None:
        raise Exception("Expected Some but got None")
    return value


def transfer_tokens(param: T, amount: Mutez, contract: Contract[T]) -> Operation:
    return Operation()


def smart_contract(storage: Bytes, param: Int) -> Pair[List[Operation], Bytes]:
    byt: Final[Bytes] = get_bytes(param)
    hashed_param: Final[Bytes] = sha256(byt)
    nil: Final[List[Operation]] = make_list()

    p: Final[Pair[List[Operation], Bytes]] = make_pair(nil, hashed_param)
    return p
