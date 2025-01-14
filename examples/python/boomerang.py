from dataclasses import dataclass
from typing import Final, List, TypeVar, Generic, Optional

Mutez = int
Address = str
Int = int


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


def smart_contract(storage: Unit, param: Unit) -> Pair[List[Operation], Unit]:
    amount: Final[Mutez] = get_amount()
    nil: Final[List[Operation]] = make_list()
    address: Final[Address] = get_source()
    some_contract: Final[Optional[Contract[Unit]]] = get_contract(address)
    contract: Final[Contract[Unit]] = assert_some(some_contract)
    operation: Final[Operation] = transfer_tokens(param, amount, contract)
    operations: Final[List[Operation]] = append(nil, operation)

    p: Final[Pair[List[Operation], Unit]] = make_pair(operations, param)
    return p
