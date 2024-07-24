# func itself does not exceed complexity = 7
# but if inner function is also counted, it exceeds the threshold
def func() -> None:

    if True:
        pass

    if False:
        pass
    elif True:
        pass

    if "True":
        pass

    def inner():
        n = 5
        match n:
            case 1:
                pass
            case 2:
                pass
            case 3:
                pass
            case 4:
                pass
            case 5:
                pass

    inner()
    return None
