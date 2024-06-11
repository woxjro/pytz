from typing import Final

def add(a: int, b: int) -> int:
    return a + b

x: Final[int] = 10
y: Final[int] = 20

print(add(x, y))
