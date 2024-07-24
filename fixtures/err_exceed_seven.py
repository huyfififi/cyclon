# TODO: Add GitHub Actions to test these
def is_even(n: int) -> int:
    match n:
        case 1:
            return False
        case 2:
            return True
        case 3:
            return False
        case 4:
            return True
        case 5:
            return False
        case 6:
            return True
        case 7:
            return False
        case 8:
            return True
        case _:
            return False


def is_odd(n: int) -> int:
    if n == 1:
        return True
    elif n == 2:
        return False
    elif n == 3:
        return True
    elif n == 4:
        return False
    elif n == 5:
        return True
    elif n == 6:
        return False
    elif n == 7:
        return True
    elif n == 8:
        return False
    else:
        return False
