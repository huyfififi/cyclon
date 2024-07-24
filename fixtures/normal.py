def a() -> int:
    if True:
        return 0
    return 1


def b() -> int:
    if False:
        return 0
    elif True:
        return 1


def c() -> int:
    n = 1
    if n == 0:
        return 0
    elif n == 1:
        return 1
    elif n == 2:
        return 2
    else:
        return 3
