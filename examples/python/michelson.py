from typing import Final, List

mutez = int

class Operation:
    pass

def get_amount() -> mutez:
    DUMMY_AMOUNT: Final[mutez] = 0
    return DUMMY_AMOUNT

def make_list() -> List:
    return []

def smart_contract(storage: mutez, param: mutez) -> (List[Operation], mutez):
    amount: Final[mutez] = get_amount()
    list: Final[List[Operation]] = make_list()
    return list, amount


# a: Final[mutez] = get_amount()
# y: Final[mutez] = 20
# list: Final[List[Operation]] = make_list()
# list.append(Operation())
