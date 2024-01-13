import numpy as np
from testlib import multiply
#  The module name matches the Rust module name, $NAME


def main():
    x: np.ndarray = np.array([10., 20., 30.])
    multiply(3.0, x)
    print(x[2])


if __name__ == "__main__":
    main()
