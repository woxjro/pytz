from dataclasses import dataclass
from typing import Final, List, TypeVar, Generic

mutez = int


class Operation:
    pass


T = TypeVar('T')
U = TypeVar('U')


@dataclass
class Pair(Generic[T, U]):
    first: T
    second: U


def make_pair(first: T, second: U) -> Pair[T, U]:
    return Pair(first, second)


def get_amount() -> mutez:
    DUMMY_AMOUNT: Final[mutez] = 5000
    return DUMMY_AMOUNT


def make_list() -> List:
    return []


def append(operations: List[Operation], operation: Operation) -> List[Operation]:
    operations.append(operation)
    return operations


def smart_contract(storage: mutez, param: mutez) -> Pair[List[Operation], mutez]:
    amount: Final[mutez] = get_amount()
    operations: Final[List[Operation]] = make_list()
    p: Final[Pair[List[Operation], mutez]] = make_pair(operations, amount)
    return p
